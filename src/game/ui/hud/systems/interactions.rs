use bevy::prelude::*;

use crate::game::ui::hud::components::*;
use crate::game::score::resources::*;

pub fn update_score( 
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>

) {
    for mut text in &mut score_text_query {
        if score.is_changed() {
            text.sections[0].value = format!("{}",score.value.to_string());
        }
    }
}