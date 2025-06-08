use crate::systems::spawn;
use bevy_ecs::prelude::*;
mod components;
mod models;
mod systems;
mod types;
mod utils;

fn setup(commands: Commands) {
    spawn::spawn_random_being(commands);
}

fn main() {
    let mut world = World::new();

    let mut schedule = Schedule::default();

    schedule.add_systems((
        systems::update::basic_needs_decrease_system,
        systems::update::check_injuries_system,
    ));
        
    setup(world.commands());

    // Loop de simulação — aqui rodamos só algumas iterações
    for _ in 0..10000 {
        schedule.run(&mut world);
    }
}
