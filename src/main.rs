mod components;
mod systems;
mod entity_builders;
mod utils;

use crate::components::components_default::CustomComponentsPlugin;
use crate::components::components_resources::{ColorConstants, GameConstants, RumorTimer};
use crate::components::*;
use crate::entity_builders::{entity_builders_environment::spawn_environmental_resources, entity_builders_npc::spawn_test_npcs};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::WorldInspectorPlugin,
};
use bevy_rapier2d::prelude::*;
use systems::events::events_environment::{ResourceDepletionEvent, ResourceInteractionEvent};
use systems::events::events_movement::{BoundaryCollisionEvent, MovementBehaviorEvent};
// Import all the ML-ready events for future integration
use systems::events::events_needs::{DesireChangeEvent, NeedDecayEvent, SocialInteractionEvent};
use systems::events::events_pathfinding::{PathTargetReachedEvent, PathTargetSetEvent, ResourceDiscoveredEvent};
use systems::events::events_rumor::{RumorInjectionEvent, RumorSpreadAttemptEvent, RumorSpreadEvent};

use crate::systems::systems_environment::{
    handle_hotel_interactions,
    handle_restaurant_interactions,
    handle_well_interactions,
    regenerate_environmental_resources,
};
use crate::systems::systems_movement::{analyze_movement_patterns, movement_system};
use crate::systems::systems_needs::{
    decay_basic_needs,
    fulfill_desires,
    handle_social_interactions,
    update_desires_from_needs,
};
use crate::systems::systems_pathfinding::{
    analyze_pathfinding_behavior,
    resource_discovery_system,
    steering_navigation_system,
    target_cleanup_system,
    target_selection_system,
};
// Import all the systems we need
use crate::systems::systems_rumor::{analyze_rumor_diffusion, handle_rumor_spread, inject_rumor_system};
use crate::systems::systems_visual::color_system;

fn setup_simulation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_constants: Res<GameConstants>,
    windows: Query<&Window>,
) {
    commands.spawn(Camera2d);

    // Spawn NPCs first
    spawn_test_npcs(&mut commands, &asset_server, &game_constants);

    // Spawn environmental resources randomly across the space
    if let Ok(window) = windows.single() {
        spawn_environmental_resources(
            &mut commands,
            &asset_server,
            &game_constants,
            window.width(),
            window.height(),
        );
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            CustomComponentsPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        // Resources initialization
        .insert_resource(RumorTimer(Timer::from_seconds(3.0, TimerMode::Once)))
        .insert_resource(GameConstants::default())
        .insert_resource(ColorConstants::default())

        // Register Rapier debug render context for inspector control
        .register_type::<bevy_rapier2d::render::DebugRenderContext>()

        // ML-HOOK: Register all events for quantifiable behavior tracking
        .add_event::<NeedDecayEvent>()
        .add_event::<DesireChangeEvent>()
        .add_event::<SocialInteractionEvent>()
        .add_event::<RumorInjectionEvent>()
        .add_event::<RumorSpreadEvent>()
        .add_event::<RumorSpreadAttemptEvent>()
        .add_event::<BoundaryCollisionEvent>()
        .add_event::<MovementBehaviorEvent>()
        .add_event::<ResourceInteractionEvent>()
        .add_event::<ResourceDepletionEvent>()
        .add_event::<PathTargetSetEvent>()
        .add_event::<PathTargetReachedEvent>()
        .add_event::<ResourceDiscoveredEvent>()

        // Startup systems
        .add_systems(Startup, setup_simulation)

        // Update systems organized by priority and dependencies
        .add_systems(Update, (
            // Core need management (highest priority)
            decay_basic_needs,
            update_desires_from_needs,

            // Movement system with desire-driven behavior
            movement_system,

            // Social and rumor systems
            (
                inject_rumor_system,
                handle_rumor_spread,
                handle_social_interactions,
            ),

            // Environmental resource interactions
            (
                handle_well_interactions,
                handle_restaurant_interactions,
                handle_hotel_interactions,
                regenerate_environmental_resources,
            ),

            // Desire fulfillment and visual feedback
            (
                fulfill_desires,
                color_system,
            ).chain(),

            // Pathfinding systems
            (
                resource_discovery_system,
                target_selection_system,
                steering_navigation_system,
                target_cleanup_system,
            ),

            // Analysis and debugging systems (lower priority)
            (
                analyze_rumor_diffusion,
                analyze_movement_patterns,
                analyze_pathfinding_behavior,
            ),
        ))
        .run();
}
