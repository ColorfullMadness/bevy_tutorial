use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use systems::*;
use resources::*;

use crate::game::SimulationState;
use crate::AppState;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 300.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        //Resources
        .init_resource::<EnemySpawnTimer>()
        //Startup Systems
        //.add_startup_system(spawn_enemies)
        //Enter State Systems
        .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
        //Systems
        .add_systems(
            (
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                tick_enemy_spawn_timer,
                spawn_enemies_over_time
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        // Exit state systems
        .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)))
        ;
    }
}