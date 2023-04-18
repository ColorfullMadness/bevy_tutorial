use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    mut simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == Some(SimulationState::Running) {
            simulation_state.set(SimulationState::Running);
            println!("Simulation paused.");
        }
        if simulation_state.0 == Some(SimulationState::Paused) {
            simulation_state.set(SimulationState::Paused);
            println!("Simulation running.");
        }
    }
}