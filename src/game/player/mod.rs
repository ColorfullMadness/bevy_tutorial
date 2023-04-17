use bevy::prelude::*;

use self::systems::*;

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
        .add_startup_system(spawn_player)
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
        .add_system(player_movement.in_set(MovementSystemSet))
        .add_system(confine_player_movement.in_set(ConfinementSystemSet))

        .add_system(player_hit_enemy)
        .add_system(player_hit_star);
    }
}