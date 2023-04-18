use bevy::prelude::*;

use self::systems::*;
use super::AppState;
use super::SimulationState;

pub mod components;
mod systems;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub enum PlayerSystemSet {
//     Movement, 
//     Confinement
// }

// pub enum PlayerSystemSet {
//     Movement, 
//     Confinement
// }

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_set(MovementSystemSet.before(ConfinementSystemSet))
        //.add_startup_system(spawn_player)
        //On Enter State
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))

        //? first way to order system execution
        //.add_system(player_movement)
        //.add_system(confine_player_movement.after(player_movement))

        //? second and third way to order systems execution
        // .add_systems(
        //     (
        //         player_movement,
        //         confine_player_movement.after(player_movement)
        //     )//.chain()
        // )

        //? fourth way to order systems
        .add_system(player_movement
            .in_set(MovementSystemSet)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
        .add_system(confine_player_movement
            .in_set(ConfinementSystemSet)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
        .add_systems(
            (player_hit_enemy,player_hit_star)
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        //On exit system
        .add_system(despawn_player.in_schedule(OnExit(AppState::Game))
    );
        
    }
}