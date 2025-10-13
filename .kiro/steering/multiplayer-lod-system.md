---
inclusion: fileMatch
fileMatchPattern: "src/ai/**/*.rs"
---

# Multi-Player Level of Detail (LOD) System

## Core Principle: Global Player Awareness

The LOD system must account for ALL players in the world, not just a single player perspective. This ensures consistent AI behavior quality across the entire multiplayer environment.

## Multi-Player Distance Calculation

### Player Tracking Resource
```rust
#[derive(Resource, Debug)]
pub struct PlayerTracker {
    pub active_players: HashMap<Entity, PlayerInfo>,
    pub player_clusters: Vec<PlayerCluster>,
    pub world_bounds: Rect,
    pub lod_grid: Grid2D<LodLevel>,
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub entity: Entity,
    pub position: Vec3,
    pub last_update: f64,
    pub view_distance: f32,    // How far this player can see
    pub importance_radius: f32, // Radius for high-detail AI
}

#[derive(Debug, Clone)]
pub struct PlayerCluster {
    pub center: Vec3,
    pub radius: f32,
    pub player_count: usize,
    pub combined_importance: f32,
}
```

### Distance-Based Importance Calculation
```rust
#[derive(Component, Debug)]
pub struct AgentImportance {
    pub level: ImportanceLevel,
    pub distance_to_nearest_player: f32,
    pub affecting_players: Vec<Entity>, // Which players can see this agent
    pub last_update: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImportanceLevel {
    Critical,   // 0-50m from any player - Full AI processing
    High,       // 50-150m from any player - High frequency updates
    Medium,     // 150-500m from any player - Medium frequency updates
    Low,        // 500m+ from all players - Background simulation only
    Dormant,    // No players in region - Minimal processing
}

fn calculate_agent_importance(
    agent_pos: Vec3,
    players: &HashMap<Entity, PlayerInfo>,
) -> AgentImportance {
    let mut min_distance = f32::INFINITY;
    let mut affecting_players = Vec::new();
    
    // Check distance to ALL players
    for (player_entity, player_info) in players.iter() {
        let distance = agent_pos.distance(player_info.position);
        
        if distance < min_distance {
            min_distance = distance;
        }
        
        // Track which players can potentially see this agent
        if distance < player_info.importance_radius {
            affecting_players.push(*player_entity);
        }
    }
    
    let level = match min_distance {
        d if d < 50.0 => ImportanceLevel::Critical,
        d if d < 150.0 => ImportanceLevel::High,
        d if d < 500.0 => ImportanceLevel::Medium,
        d if d < 1000.0 => ImportanceLevel::Low,
        _ => ImportanceLevel::Dormant,
    };
    
    AgentImportance {
        level,
        distance_to_nearest_player: min_distance,
        affecting_players,
        last_update: 0.0,
    }
}
```

## Spatial Grid-Based LOD

### World Partitioning
```rust
#[derive(Resource)]
pub struct LodGrid {
    pub grid: Grid2D<LodCell>,
    pub cell_size: f32,        // Size of each grid cell (e.g., 100m x 100m)
    pub world_bounds: Rect,
}

#[derive(Debug, Clone)]
pub struct LodCell {
    pub lod_level: ImportanceLevel,
    pub agent_count: usize,
    pub player_count: usize,
    pub last_player_visit: f64,
    pub computational_budget: f32,
}

impl LodGrid {
    pub fn update_from_players(&mut self, players: &HashMap<Entity, PlayerInfo>) {
        // Reset all cells to dormant
        for cell in self.grid.cells_mut() {
            cell.lod_level = ImportanceLevel::Dormant;
            cell.player_count = 0;
        }
        
        // Update cells based on player positions
        for player_info in players.values() {
            let player_cell = self.world_pos_to_cell(player_info.position);
            
            // Set high importance in a radius around each player
            let radius_cells = (player_info.importance_radius / self.cell_size).ceil() as i32;
            
            for dx in -radius_cells..=radius_cells {
                for dy in -radius_cells..=radius_cells {
                    if let Some(cell) = self.grid.get_mut(player_cell.x + dx, player_cell.y + dy) {
                        let cell_center = self.cell_to_world_pos(IVec2::new(player_cell.x + dx, player_cell.y + dy));
                        let distance = player_info.position.distance(cell_center);
                        
                        let cell_importance = match distance {
                            d if d < 50.0 => ImportanceLevel::Critical,
                            d if d < 150.0 => ImportanceLevel::High,
                            d if d < 300.0 => ImportanceLevel::Medium,
                            d if d < 500.0 => ImportanceLevel::Low,
                            _ => ImportanceLevel::Dormant,
                        };
                        
                        // Use highest importance level if multiple players affect this cell
                        if cell_importance as u8 > cell.lod_level as u8 {
                            cell.lod_level = cell_importance;
                        }
                        
                        cell.player_count += 1;
                    }
                }
            }
        }
    }
}
```

## Dynamic Update Frequency System

### Importance-Based Scheduling
```rust
fn adaptive_ai_update_system(
    mut agents: Query<(&mut AgentImportance, &Transform, &mut PollingSchedule)>,
    players: Res<PlayerTracker>,
    world_time: Res<WorldTime>,
    quality_scaler: Res<QualityScaler>,
) {
    // Update agent importance based on current player positions
    agents.par_iter_mut().for_each(|(mut importance, transform, mut schedule)| {
        // Recalculate importance every few seconds
        if world_time.current_time - importance.last_update > 5.0 {
            *importance = calculate_agent_importance(
                transform.translation,
                &players.active_players,
            );
            importance.last_update = world_time.current_time;
        }
        
        // Set update frequency based on importance and performance
        let base_interval = match importance.level {
            ImportanceLevel::Critical => 0.1,   // 10 times per second
            ImportanceLevel::High => 0.5,       // 2 times per second
            ImportanceLevel::Medium => 2.0,     // Every 2 seconds
            ImportanceLevel::Low => 10.0,       // Every 10 seconds
            ImportanceLevel::Dormant => 60.0,   // Every minute
        };
        
        // Apply global quality scaling
        schedule.interval = base_interval * quality_scaler.get_update_frequency_multiplier();
    });
}
```

## Player Cluster Optimization

### Efficient Multi-Player Handling
```rust
impl PlayerTracker {
    pub fn update_player_clusters(&mut self) {
        self.player_clusters.clear();
        
        let mut unclustered_players: Vec<_> = self.active_players.values().collect();
        
        while !unclustered_players.is_empty() {
            let seed_player = unclustered_players.remove(0);
            let mut cluster = PlayerCluster {
                center: seed_player.position,
                radius: 0.0,
                player_count: 1,
                combined_importance: seed_player.importance_radius,
            };
            
            // Find nearby players to add to this cluster
            let cluster_radius = 200.0; // Players within 200m are considered clustered
            unclustered_players.retain(|player| {
                let distance = seed_player.position.distance(player.position);
                if distance < cluster_radius {
                    // Add to cluster
                    cluster.center = (cluster.center * cluster.player_count as f32 + player.position) / (cluster.player_count + 1) as f32;
                    cluster.radius = cluster.radius.max(distance);
                    cluster.player_count += 1;
                    cluster.combined_importance += player.importance_radius;
                    false // Remove from unclustered list
                } else {
                    true // Keep in unclustered list
                }
            });
            
            self.player_clusters.push(cluster);
        }
    }
    
    pub fn get_computational_budget_for_region(&self, position: Vec3) -> f32 {
        let mut total_budget = 0.0;
        
        for cluster in &self.player_clusters {
            let distance = position.distance(cluster.center);
            let influence = (cluster.combined_importance - distance).max(0.0) / cluster.combined_importance;
            total_budget += influence * cluster.player_count as f32;
        }
        
        total_budget.clamp(0.1, 10.0) // Minimum background simulation, maximum full detail
    }
}
```

## Performance Monitoring for Multi-Player

### Region-Based Performance Tracking
```rust
#[derive(Resource)]
pub struct RegionalPerformanceMonitor {
    pub region_stats: HashMap<IVec2, RegionStats>,
    pub global_stats: GlobalStats,
}

#[derive(Debug, Default)]
pub struct RegionStats {
    pub agent_count: usize,
    pub player_count: usize,
    pub avg_frame_time: f32,
    pub lod_level: ImportanceLevel,
    pub computational_budget: f32,
    pub budget_utilization: f32,
}

fn monitor_regional_performance(
    mut monitor: ResMut<RegionalPerformanceMonitor>,
    lod_grid: Res<LodGrid>,
    agents: Query<&Transform, With<Agent>>,
    players: Res<PlayerTracker>,
) {
    monitor.region_stats.clear();
    
    // Count agents per region
    for transform in agents.iter() {
        let cell = lod_grid.world_pos_to_cell(transform.translation);
        let stats = monitor.region_stats.entry(cell).or_default();
        stats.agent_count += 1;
    }
    
    // Add player counts and performance data
    for (cell_pos, cell) in lod_grid.grid.cells_with_positions() {
        if let Some(stats) = monitor.region_stats.get_mut(&cell_pos) {
            stats.player_count = cell.player_count;
            stats.lod_level = cell.lod_level;
            stats.computational_budget = cell.computational_budget;
            // Add actual performance measurements here
        }
    }
}
```

## Integration with Existing Systems

### Multi-Player Aware Social Systems
```rust
fn emotional_contagion_multiplayer_system(
    mut agents: Query<(&mut EmotionalState, &Transform, &AgentImportance)>,
    players: Res<PlayerTracker>,
) {
    agents.par_iter_mut().for_each(|(mut emotion, transform, importance)| {
        // Only process emotional contagion for agents that players might observe
        match importance.level {
            ImportanceLevel::Critical | ImportanceLevel::High => {
                // Full emotional contagion processing
                process_full_emotional_contagion(&mut emotion, transform);
            },
            ImportanceLevel::Medium => {
                // Simplified emotional contagion
                process_simplified_emotional_contagion(&mut emotion);
            },
            ImportanceLevel::Low | ImportanceLevel::Dormant => {
                // Skip emotional contagion entirely
                // These agents are too far from players to matter
            },
        }
    });
}
```

This multi-player LOD system ensures that:
1. **All players are considered** when calculating agent importance
2. **Multiple high-detail zones** exist around player clusters
3. **Computational resources** are distributed efficiently across the world
4. **Performance scales** with player distribution, not just player count
5. **Background regions** maintain minimal simulation for consistency