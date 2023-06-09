use bevy::prelude::*;

use crate::game::SimulationState;

pub fn pause_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
     simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    mut simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == Some(SimulationState::Running) {
            simulation_state.set(SimulationState::Paused);
            println!("Simulation paused.");
        } else {
            simulation_state.set(SimulationState::Running);
            println!("Simulation running.");
        }
    }
}