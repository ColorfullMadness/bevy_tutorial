pub mod systems;
pub mod components;
pub mod styles;

use bevy::prelude::*;

use crate::game::ui::hud::systems::layout::*;
use crate::game::ui::hud::systems::interactions::*;

use crate::AppState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
        //On enter state systems
        .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
        // systems
        .add_system(update_score.run_if(in_state(AppState::Game)))
        ;
    }
}