use bevy::prelude::*;

pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;

use crate::AppState;

use super::events::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

pub struct GamePlug

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<SimulationState>()
        //Events
        .add_event::<GameOver>()
        //Plugins
        .add_plugin(EnemyPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        //Systems
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States,Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
