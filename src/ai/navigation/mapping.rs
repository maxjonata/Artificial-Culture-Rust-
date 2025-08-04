use bevy::prelude::*;
use std::collections::HashMap;

/// Component representing an agent's cognitive map of the environment
/// Based on Cognitive Mapping Theory and Place Cell research
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CognitiveMap {
    /// Known landmarks in the environment
    pub landmarks: Vec<Landmark>,
    /// Spatial relationships between landmarks
    pub spatial_connections: Vec<SpatialConnection>,
    /// Grid cells for spatial representation (simplified model)
    pub grid_cells: HashMap<GridCell, f32>,
    /// Current position estimate in cognitive space
    pub estimated_position: Vec2,
    /// Confidence in current position estimate
    pub position_confidence: f32,
}

/// Represents a landmark in the agent's cognitive map
/// Based on Landmark Navigation and Spatial Memory research
#[derive(Debug, Reflect, Clone)]
pub struct Landmark {
    /// Unique identifier for this landmark
    pub id: Entity,
    /// Position in the agent's cognitive map
    pub cognitive_position: Vec2,
    /// Real-world position (for ground truth comparison)
    pub world_position: Vec2,
    /// How salient/memorable this landmark is
    pub salience: f32,
    /// Confidence in the position estimate
    pub position_confidence: f32,
    /// What type of landmark this is
    pub landmark_type: LandmarkType,
    /// When this landmark was last observed
    pub last_observed: f32,
}

/// Types of landmarks agents can remember
#[derive(Debug, Reflect, Clone, Copy, PartialEq)]
pub enum LandmarkType {
    /// Thing sources (wells, restaurants, etc.)
    Resource,
    /// Other agents
    Agent,
    /// Environmental features
    Environmental,
    /// Social gathering points
    Social,
}

/// Spatial connection between two landmarks
/// Based on Graph Theory and Spatial Cognition
#[derive(Debug, Reflect, Clone)]
pub struct SpatialConnection {
    /// First landmark in the connection
    pub landmark_a: Entity,
    /// Second landmark in the connection
    pub landmark_b: Entity,
    /// Estimated distance between landmarks
    pub distance: f32,
    /// Confidence in this distance estimate
    pub confidence: f32,
    /// Directional vector from A to B
    pub direction: Vec2,
    /// How often this connection has been traversed
    pub traversal_count: u16,
}

/// Simplified grid cell representation for spatial coding
/// Based on Nobel Prize-winning Grid Cell research
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCell {
    /// Grid coordinate X
    pub x: i16,
    /// Grid coordinate Y
    pub y: i16,
    /// Scale of this grid cell (different scales for different distances)
    pub scale: u8,
}

/// Component for spatial navigation network processing
/// Implements simplified place cell and grid cell functionality
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SpatialNavigationNetwork {
    /// Place cells that fire at specific locations
    pub place_cells: Vec<PlaceCell>,
    /// Head direction cells for orientation
    pub head_direction_cells: Vec<HeadDirectionCell>,
    /// Current heading estimate
    pub current_heading: f32,
    /// Movement speed estimate
    pub speed_estimate: f32,
}

/// Simplified place cell that activates at specific locations
#[derive(Debug, Reflect, Clone)]
pub struct PlaceCell {
    /// Center of the place field
    pub center: Vec2,
    /// Radius of activation
    pub radius: f32,
    /// Maximum activation strength
    pub max_activation: f32,
    /// Current activation level
    pub current_activation: f32,
}

/// Head direction cell for orientation tracking
#[derive(Debug, Reflect, Clone)]
pub struct HeadDirectionCell {
    /// Preferred direction (in radians)
    pub preferred_direction: f32,
    /// Tuning width (how sharply tuned to direction)
    pub tuning_width: f32,
    /// Current activation level
    pub current_activation: f32,
}

/// Event fired when a new landmark is discovered
#[derive(Event, Debug)]
pub struct LandmarkDiscovered {
    pub agent: Entity,
    pub landmark: Entity,
    pub position: Vec2,
    pub landmark_type: LandmarkType,
}

/// Event fired when spatial map is updated
#[derive(Event, Debug)]
pub struct SpatialMapUpdated {
    pub agent: Entity,
    pub update_type: MapUpdateType,
}

#[derive(Debug, Clone)]
pub enum MapUpdateType {
    LandmarkAdded,
    ConnectionStrengthened,
    PositionRecalibrated,
}

impl Default for CognitiveMap {
    fn default() -> Self {
        Self {
            landmarks: Vec::new(),
            spatial_connections: Vec::new(),
            grid_cells: HashMap::new(),
            estimated_position: Vec2::ZERO,
            position_confidence: 0.5,
        }
    }
}

impl Default for SpatialNavigationNetwork {
    fn default() -> Self {
        Self {
            place_cells: Vec::new(),
            head_direction_cells: create_default_head_direction_cells(),
            current_heading: 0.0,
            speed_estimate: 0.0,
        }
    }
}

/// Create default head direction cells covering all directions
fn create_default_head_direction_cells() -> Vec<HeadDirectionCell> {
    let mut cells = Vec::new();
    let num_cells = 8; // 8 direction cells covering 360 degrees

    for i in 0..num_cells {
        let direction = (i as f32) * (2.0 * std::f32::consts::PI) / (num_cells as f32);
        cells.push(HeadDirectionCell {
            preferred_direction: direction,
            tuning_width: std::f32::consts::PI / 4.0, // 45-degree tuning width
            current_activation: 0.0,
        });
    }

    cells
}

/// System for updating cognitive maps based on movement and perception
/// Based on Simultaneous Localization and Mapping (SLAM) principles
pub fn cognitive_mapping_system(
    mut query: Query<(Entity, &mut CognitiveMap, &mut SpatialNavigationNetwork, &Transform)>,
    mut landmark_events: EventWriter<LandmarkDiscovered>,
    mut map_events: EventWriter<SpatialMapUpdated>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (entity, mut cognitive_map, mut nav_network, transform) in query.iter_mut() {
        let current_position = transform.translation.truncate();

        // Update place cells based on current position
        update_place_cells(&mut nav_network, current_position);

        // Update grid cells
        update_grid_cells(&mut cognitive_map, current_position);

        // Update position estimate using grid cell and place cell activity
        update_position_estimate(&mut cognitive_map, &nav_network, current_position);

        // Look for new landmarks to add to the map
        // Note: In a full implementation, this would integrate with the perception system
        check_for_new_landmarks(
            entity,
            &mut cognitive_map,
            current_position,
            current_time,
            &mut landmark_events,
            &mut map_events,
        );

        // Update spatial connections based on movement
        update_spatial_connections(&mut cognitive_map, current_position, &mut map_events);

        // Perform map maintenance (remove outdated landmarks, etc.)
        maintain_cognitive_map(&mut cognitive_map, current_time);
    }
}

/// Update place cell activations based on current position
fn update_place_cells(nav_network: &mut SpatialNavigationNetwork, current_position: Vec2) {
    for place_cell in nav_network.place_cells.iter_mut() {
        let distance = current_position.distance(place_cell.center);

        // Gaussian activation function
        if distance <= place_cell.radius {
            let normalized_distance = distance / place_cell.radius;
            place_cell.current_activation = place_cell.max_activation *
                (-normalized_distance * normalized_distance).exp();
        } else {
            place_cell.current_activation = 0.0;
        }
    }
}

/// Update grid cell representations
fn update_grid_cells(cognitive_map: &mut CognitiveMap, current_position: Vec2) {
    // Simplified grid cell model - divide space into grid and track activation
    let grid_sizes = [10.0, 20.0, 40.0]; // Different scales

    for (scale_index, &grid_size) in grid_sizes.iter().enumerate() {
        let grid_x = (current_position.x / grid_size).floor() as i16;
        let grid_y = (current_position.y / grid_size).floor() as i16;

        let grid_cell = GridCell {
            x: grid_x,
            y: grid_y,
            scale: scale_index as u8,
        };

        // Increase activation for current grid cell
        let activation = cognitive_map.grid_cells.entry(grid_cell).or_insert(0.0);
        *activation = (*activation + 0.1).min(1.0);

        // Decay other grid cells slightly
        for (_, other_activation) in cognitive_map.grid_cells.iter_mut() {
            *other_activation *= 0.99;
        }
    }
}

/// Update position estimate using neural activity
fn update_position_estimate(
    cognitive_map: &mut CognitiveMap,
    nav_network: &SpatialNavigationNetwork,
    actual_position: Vec2,
) {
    // Simple position estimate update based on place cell activity
    if !nav_network.place_cells.is_empty() {
        let mut weighted_position = Vec2::ZERO;
        let mut total_weight = 0.0;

        for place_cell in &nav_network.place_cells {
            if place_cell.current_activation > 0.1 {
                weighted_position += place_cell.center * place_cell.current_activation;
                total_weight += place_cell.current_activation;
            }
        }

        if total_weight > 0.0 {
            let estimated_pos = weighted_position / total_weight;

            // Blend with previous estimate
            cognitive_map.estimated_position =
                cognitive_map.estimated_position * 0.7 + estimated_pos * 0.3;

            // Update confidence based on estimation error
            let error = cognitive_map.estimated_position.distance(actual_position);
            cognitive_map.position_confidence = (1.0 - error / 100.0).max(0.1);
        }
    } else {
        // No place cells active - use actual position with low confidence
        cognitive_map.estimated_position = actual_position;
        cognitive_map.position_confidence = 0.3;
    }
}

/// Check for new landmarks to add to the cognitive map
fn check_for_new_landmarks(
    agent_entity: Entity,
    cognitive_map: &mut CognitiveMap,
    current_position: Vec2,
    current_time: f32,
    landmark_events: &mut EventWriter<LandmarkDiscovered>,
    map_events: &mut EventWriter<SpatialMapUpdated>,
) {
    // This is simplified - in a full implementation, this would integrate with perception
    // For now, we'll just create place cells at significant locations

    let discovery_threshold = 50.0; // Minimum distance from existing landmarks

    // Check if current position is far enough from existing landmarks to be significant
    let is_novel_location = cognitive_map.landmarks.iter()
        .all(|landmark| landmark.cognitive_position.distance(current_position) > discovery_threshold);

    if is_novel_location && rand::random::<f32>() < 0.01 { // 1% chance per frame to create landmark
        // Create a new place cell and landmark at this position
        let landmark_entity = Entity::from_raw(rand::random::<u32>()); // Simplified entity creation

        let new_landmark = Landmark {
            id: landmark_entity,
            cognitive_position: current_position,
            world_position: current_position,
            salience: 0.8,
            position_confidence: 0.9,
            landmark_type: LandmarkType::Environmental,
            last_observed: current_time,
        };

        cognitive_map.landmarks.push(new_landmark);

        // Send discovery event
        landmark_events.write(LandmarkDiscovered {
            agent: agent_entity,
            landmark: landmark_entity,
            position: current_position,
            landmark_type: LandmarkType::Environmental,
        });

        map_events.write(SpatialMapUpdated {
            agent: agent_entity,
            update_type: MapUpdateType::LandmarkAdded,
        });
    }
}

/// Update spatial connections between landmarks
fn update_spatial_connections(
    cognitive_map: &mut CognitiveMap,
    current_position: Vec2,
    map_events: &mut EventWriter<SpatialMapUpdated>,
) {
    // Find landmarks near current position
    let nearby_landmarks: Vec<&Landmark> = cognitive_map.landmarks.iter()
        .filter(|landmark| landmark.cognitive_position.distance(current_position) < 100.0)
        .collect();

    // Strengthen connections between nearby landmarks
    for i in 0..nearby_landmarks.len() {
        for j in (i + 1)..nearby_landmarks.len() {
            let landmark_a = nearby_landmarks[i].id;
            let landmark_b = nearby_landmarks[j].id;

            // Find existing connection or create new one
            if let Some(connection) = cognitive_map.spatial_connections.iter_mut()
                .find(|conn| (conn.landmark_a == landmark_a && conn.landmark_b == landmark_b) ||
                    (conn.landmark_a == landmark_b && conn.landmark_b == landmark_a)) {

                // Strengthen existing connection
                connection.traversal_count += 1;
                connection.confidence = (connection.confidence + 0.01).min(1.0);

                map_events.write(SpatialMapUpdated {
                    agent: Entity::from_raw(0), // Simplified
                    update_type: MapUpdateType::ConnectionStrengthened,
                });
            } else {
                // Create new connection
                let distance = nearby_landmarks[i].cognitive_position
                    .distance(nearby_landmarks[j].cognitive_position);
                let direction = (nearby_landmarks[j].cognitive_position -
                    nearby_landmarks[i].cognitive_position).normalize();

                let new_connection = SpatialConnection {
                    landmark_a,
                    landmark_b,
                    distance,
                    confidence: 0.5,
                    direction,
                    traversal_count: 1,
                };

                cognitive_map.spatial_connections.push(new_connection);
            }
        }
    }
}

/// Maintain the cognitive map by removing outdated information
fn maintain_cognitive_map(cognitive_map: &mut CognitiveMap, current_time: f32) {
    // Remove very old landmarks that haven't been observed recently
    cognitive_map.landmarks.retain(|landmark| {
        current_time - landmark.last_observed < 300.0 // 5 minutes
    });

    // Remove connections involving deleted landmarks
    let valid_landmark_ids: std::collections::HashSet<Entity> =
        cognitive_map.landmarks.iter().map(|l| l.id).collect();

    cognitive_map.spatial_connections.retain(|connection| {
        valid_landmark_ids.contains(&connection.landmark_a) &&
            valid_landmark_ids.contains(&connection.landmark_b)
    });

    // Decay grid cell activations
    for (_, activation) in cognitive_map.grid_cells.iter_mut() {
        *activation *= 0.999; // Very slow decay
    }

    // Remove very weak grid cells
    cognitive_map.grid_cells.retain(|_, activation| *activation > 0.01);
}
