use bevy::{prelude::*, window::PrimaryWindow};
use bevy::app::AppExit;

use crate::game::SimulationState;
use crate::{events::*, AppState};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != Some(AppState::Game) {
            app_state.set(AppState::Game);
            println!("Entered AppState::Game.");
        }
    }
}

pub fn transition_to_main_menu (
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut simulation_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != Some(AppState::MainMenu) {
            app_state.set(AppState::MainMenu);
            println!("Entered AppState::MainMenu.");
        }
        if simulation_state.0 != Some(SimulationState::Paused) {
            simulation_state.set(SimulationState::Paused);
            println!("Paused simulation!");
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit); 
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut app_state: ResMut<NextState<AppState>>
) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {} ", event.score.to_string());
        app_state.set(AppState::GameOver);
    }
}