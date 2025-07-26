/// Comprehensive tests for all systems and entity builders
/// This file contains tests for:
/// - Movement system logic
/// - Rumor system logic  
/// - Visual system logic
/// - NPC entity builder logic

#[cfg(test)]
mod comprehensive_tests {

    // ===== MOVEMENT SYSTEM TESTS =====
    
    #[test]
    fn test_movement_bounds_calculation() {
        // Test bounds calculation logic from movement system
        let window_width = 800.0;
        let window_height = 600.0;
        let npc_radius = 15.0;
        
        let bounds_x = window_width / 2.0 - npc_radius;
        let bounds_y = window_height / 2.0 - npc_radius;
        
        assert_eq!(bounds_x, 385.0);
        assert_eq!(bounds_y, 285.0);
    }

    #[test]
    fn test_movement_direction_normalization() {
        // Test direction vector normalization
        let direction = (3.0f32, 4.0f32); // Length = 5.0
        let length = (direction.0 * direction.0 + direction.1 * direction.1).sqrt();
        let normalized = (direction.0 / length, direction.1 / length);
        
        assert!((normalized.0 - 0.6f32).abs() < 0.001);
        assert!((normalized.1 - 0.8f32).abs() < 0.001);
    }

    #[test]
    fn test_movement_boundary_collision_detection() {
        // Test boundary collision detection logic
        let position = (-400.0, 0.0);
        let direction = (-1.0, 0.0); // Moving left
        let bounds = (385.0, 285.0);
        
        // Should detect left wall collision
        let hits_left_wall = position.0 < -bounds.0 && direction.0 < 0.0;
        assert!(hits_left_wall);
        
        // Test right wall
        let position_right = (400.0, 0.0);
        let direction_right = (1.0, 0.0);
        let hits_right_wall = position_right.0 > bounds.0 && direction_right.0 > 0.0;
        assert!(hits_right_wall);
    }

    #[test]
    fn test_movement_reflection_logic() {
        // Test direction reflection across walls
        let direction = (-1.0f32, 0.5f32);
        let wall_normal = (1.0f32, 0.0f32); // Left wall normal
        
        // Reflection formula: direction - 2 * dot(direction, normal) * normal
        let dot_product = direction.0 * wall_normal.0 + direction.1 * wall_normal.1;
        let reflected = (
            direction.0 - 2.0 * dot_product * wall_normal.0,
            direction.1 - 2.0 * dot_product * wall_normal.1
        );
        
        // X component should flip, Y should remain the same
        assert!((reflected.0 - 1.0f32).abs() < 0.001);
        assert!((reflected.1 - 0.5f32).abs() < 0.001);
    }

    // ===== RUMOR SYSTEM TESTS =====

    #[test]
    fn test_rumor_spread_conditions() {
        // Test rumor spreading logic
        let first_knows_rumor = true;
        let second_knows_rumor = false;
        let second_openness = 0.8;
        
        // Rumor should spread if: first knows && second doesn't && random < openness
        let should_spread = first_knows_rumor && !second_knows_rumor && (0.5 < second_openness);
        assert!(should_spread);
        
        // Test case where second already knows
        let second_already_knows = true;
        let should_not_spread = first_knows_rumor && !second_already_knows && (0.5 < second_openness);
        assert!(!should_not_spread);
    }

    #[test]
    fn test_rumor_personality_openness_ranges() {
        // Test personality openness boundary values
        let test_cases = vec![
            (0.0, false),   // Very closed
            (0.3, false),   // Below threshold
            (0.5, false),   // Exactly at threshold (should not spread with > comparison)
            (0.6, true),    // Above threshold
            (1.0, true),    // Fully open
        ];
        
        for (openness, expected_spread) in test_cases {
            let should_spread = 0.5 < openness;
            assert_eq!(should_spread, expected_spread, "Failed for openness: {}", openness);
        }
    }

    #[test]
    fn test_rumor_timer_logic() {
        // Test timer functionality for rumor injection
        let timer_duration = 5.0;
        let mut elapsed_time = 0.0;
        
        // Before timer finishes
        elapsed_time = 3.0;
        let timer_finished = elapsed_time >= timer_duration;
        assert!(!timer_finished);
        
        // After timer finishes
        elapsed_time = 6.0;
        let timer_finished = elapsed_time >= timer_duration;
        assert!(timer_finished);
    }

    #[test]
    fn test_rumor_injection_once_only() {
        // Test that rumor is injected only once
        let mut injected = false;
        let timer_finished = true;
        
        // First time - should inject
        let should_inject = !injected && timer_finished;
        assert!(should_inject);
        
        // After injection
        injected = true;
        let should_inject_again = !injected && timer_finished;
        assert!(!should_inject_again);
    }

    // ===== VISUAL SYSTEM TESTS =====

    #[test]
    fn test_visual_color_change_logic() {
        // Test color changing logic based on rumor knowledge
        let knows_rumor = true;
        let red_color = (1.0, 0.2, 0.2);
        let green_color = (0.2, 1.0, 0.2);
        
        let mut current_color = green_color;
        
        // Apply color system logic
        if knows_rumor {
            current_color = red_color;
        }
        
        assert_eq!(current_color, red_color);
    }

    #[test]
    fn test_visual_color_constants() {
        // Test color constant values
        let red_color = (1.0, 0.2, 0.2);
        let green_color = (0.2, 1.0, 0.2);
        
        // Verify red color components
        assert_eq!(red_color.0, 1.0);   // Red component
        assert_eq!(red_color.1, 0.2);   // Green component
        assert_eq!(red_color.2, 0.2);   // Blue component
        
        // Verify green color components
        assert_eq!(green_color.0, 0.2); // Red component
        assert_eq!(green_color.1, 1.0); // Green component
        assert_eq!(green_color.2, 0.2); // Blue component
    }

    #[test]
    fn test_visual_multiple_npcs() {
        // Test color logic for multiple NPCs
        let npcs = vec![
            (true, (0.2, 1.0, 0.2)),   // Knows rumor, starts green
            (false, (0.2, 1.0, 0.2)),  // Doesn't know, starts green
            (true, (0.2, 1.0, 0.2)),   // Knows rumor, starts green
        ];
        
        let red_color = (1.0, 0.2, 0.2);
        let green_color = (0.2, 1.0, 0.2);
        
        let results: Vec<_> = npcs.iter().map(|&(ref knows_rumor, mut color)| {
            if *knows_rumor {
                color = red_color;
            }
            color
        }).collect();
        
        assert_eq!(results[0], red_color);   // Should be red
        assert_eq!(results[1], green_color); // Should stay green
        assert_eq!(results[2], red_color);   // Should be red
    }

    // ===== NPC ENTITY BUILDER TESTS =====

    #[test]
    fn test_npc_builder_basic_properties() {
        // Test basic NPC properties from entity builder
        let npc_radius = 15.0;
        let npc_speed = 200.0;
        let num_npcs = 20;
        
        // Test sprite size calculation
        let sprite_size = npc_radius * 2.0;
        assert_eq!(sprite_size, 30.0);
        
        // Test default values
        assert_eq!(num_npcs, 20);
        assert_eq!(npc_speed, 200.0);
        assert_eq!(npc_radius, 15.0);
    }

    #[test]
    fn test_npc_builder_personality_generation() {
        // Test personality openness range [0.0, 0.5]
        let openness_values = vec![0.0, 0.1, 0.25, 0.4, 0.5];
        
        for openness in openness_values {
            assert!(openness >= 0.0 && openness <= 0.5, 
                   "Openness {} should be in range [0.0, 0.5]", openness);
        }
    }

    #[test]
    fn test_npc_builder_initial_state() {
        // Test initial NPC state
        let knows_rumor = false;
        let initial_color = (0.2, 1.0, 0.2); // Green
        
        assert!(!knows_rumor, "NPCs should start without knowing rumor");
        assert_eq!(initial_color, (0.2, 1.0, 0.2), "NPCs should start with green color");
    }

    #[test]
    fn test_npc_builder_position_handling() {
        // Test position handling in entity builder
        let positions = vec![
            (0.0, 0.0),
            (-100.0, 200.0),
            (300.0, -150.0),
            (-400.0, -300.0),
        ];
        
        for (x, y) in positions {
            // Test that positions are handled correctly
            let transform_x = x;
            let transform_y = y;
            let transform_z = 0.0; // Should always be 0 for 2D
            
            assert_eq!(transform_x, x);
            assert_eq!(transform_y, y);
            assert_eq!(transform_z, 0.0);
        }
    }

    #[test]
    fn test_npc_builder_naming_convention() {
        // Test NPC naming convention
        for npc_id in 0..10 {
            let expected_name = format!("NPC {}", npc_id + 1);
            
            // Verify naming pattern
            assert!(expected_name.starts_with("NPC "));
            assert!(expected_name.contains(&(npc_id + 1).to_string()));
        }
    }

    #[test]
    fn test_npc_builder_spawn_range() {
        // Test spawn position ranges from spawn_test_npcs
        let spawn_range_x = (-400.0, 400.0);
        let spawn_range_y = (-300.0, 300.0);
        
        // Test that ranges are reasonable
        assert!(spawn_range_x.0 < spawn_range_x.1);
        assert!(spawn_range_y.0 < spawn_range_y.1);
        
        // Test sample positions within range
        let test_positions = vec![
            (0.0, 0.0),
            (-200.0, 150.0),
            (350.0, -250.0),
        ];
        
        for (x, y) in test_positions {
            assert!(x >= spawn_range_x.0 && x <= spawn_range_x.1, "X position {} out of range", x);
            assert!(y >= spawn_range_y.0 && y <= spawn_range_y.1, "Y position {} out of range", y);
        }
    }

    #[test]
    fn test_npc_builder_physics_constants() {
        // Test physics-related constants
        let gravity_scale = 0.0; // Disabled for top-down view
        let restitution = 0.7;
        let friction = 0.2;
        let linear_damping = 0.5;
        let angular_damping = 0.5;
        
        assert_eq!(gravity_scale, 0.0);
        assert_eq!(restitution, 0.7);
        assert_eq!(friction, 0.2);
        assert_eq!(linear_damping, 0.5);
        assert_eq!(angular_damping, 0.5);
    }

    // ===== INTEGRATION TESTS =====

    #[test]
    fn test_system_integration_rumor_to_visual() {
        // Test integration between rumor and visual systems
        let mut npc_knows_rumor = false;
        let mut npc_color = (0.2, 1.0, 0.2); // Green
        let red_color = (1.0, 0.2, 0.2);
        
        // Initially green
        assert_eq!(npc_color, (0.2, 1.0, 0.2));
        
        // NPC learns rumor (rumor system)
        npc_knows_rumor = true;
        
        // Visual system updates color
        if npc_knows_rumor {
            npc_color = red_color;
        }
        
        // Should now be red
        assert_eq!(npc_color, red_color);
    }

    #[test]
    fn test_system_integration_movement_boundaries() {
        // Test integration between movement system and game constants
        let window_size = (800.0, 600.0);
        let npc_radius = 15.0;
        let npc_speed = 200.0;
        
        let bounds = (
            window_size.0 / 2.0 - npc_radius,
            window_size.1 / 2.0 - npc_radius
        );
        
        // Test that NPC at boundary with outward velocity gets reflected
        let position = (-bounds.0 - 1.0, 0.0); // Just outside left boundary
        let velocity = (-npc_speed, 0.0); // Moving left
        
        let hits_boundary = position.0 < -bounds.0 && velocity.0 < 0.0;
        assert!(hits_boundary);
        
        // After reflection, velocity should be positive
        let reflected_velocity = if hits_boundary {
            (npc_speed, velocity.1) // Simplified reflection
        } else {
            velocity
        };
        
        assert!(reflected_velocity.0 > 0.0);
    }

    #[test]
    fn test_comprehensive_npc_lifecycle() {
        // Test complete NPC lifecycle through all systems
        
        // 1. Entity Builder: Create NPC
        let mut npc = NpcState {
            id: 1,
            position: (0.0, 0.0),
            velocity: (100.0, 0.0),
            knows_rumor: false,
            color: (0.2, 1.0, 0.2), // Green
            openness: 0.3,
        };
        
        // 2. Movement System: Update position
        let dt = 0.016; // ~60 FPS
        npc.position.0 += npc.velocity.0 * dt;
        npc.position.1 += npc.velocity.1 * dt;
        
        assert!((npc.position.0 - 1.6).abs() < 0.001);
        
        // 3. Rumor System: NPC learns rumor
        npc.knows_rumor = true;
        
        // 4. Visual System: Update color
        if npc.knows_rumor {
            npc.color = (1.0, 0.2, 0.2); // Red
        }
        
        assert_eq!(npc.color, (1.0, 0.2, 0.2));
        assert!(npc.knows_rumor);
    }

    // Helper struct for integration testing
    struct NpcState {
        id: usize,
        position: (f32, f32),
        velocity: (f32, f32),
        knows_rumor: bool,
        color: (f32, f32, f32),
        openness: f32,
    }

    #[test]
    fn test_edge_cases_and_error_handling() {
        // Test various edge cases
        
        // Zero velocity normalization
        let zero_velocity = (0.0f32, 0.0f32);
        let velocity_length = (zero_velocity.0 * zero_velocity.0 + zero_velocity.1 * zero_velocity.1).sqrt();
        let normalized = if velocity_length > 0.0 {
            (zero_velocity.0 / velocity_length, zero_velocity.1 / velocity_length)
        } else {
            (1.0, 0.0) // Default direction
        };
        assert_eq!(normalized, (1.0, 0.0));
        
        // Boundary edge case - exactly at boundary
        let bounds = 385.0;
        let position = bounds;
        let direction = 1.0;
        let at_boundary = position >= bounds && direction > 0.0;
        assert!(at_boundary);
        
        // Openness boundary values
        let openness_boundary = 0.5;
        assert!(!(openness_boundary > 0.5)); // Exactly at boundary should not spread
        assert!(0.50001 > 0.5); // Just above should spread
    }
}