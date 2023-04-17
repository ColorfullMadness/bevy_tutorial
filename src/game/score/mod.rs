use bevy::prelude::*;

use self::resources::*;
use self::systems::*;

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
        .init_resource::<HighScores>()
        .add_system(update_score)
        .add_system(update_high_scores)
        .add_system(high_scores_updated);
    }
}
