use bevy::prelude::*;

pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;
pub mod ui;

use crate::AppState;

use super::events::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<SimulationState>()
        //Events
        .add_event::<GameOver>()
        //On Enter Systems
        .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
        //Plugins
        .add_plugin(EnemyPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        //Systems
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
        //On exit systems
        .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States,Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
