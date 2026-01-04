use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_fps_controller::controller::FpsController;

use super::super::GameState;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), lock_cursor)
            .add_systems(OnExit(GameState::Playing), unlock_cursor)
            .add_systems(Update, manage_cursor);
    }
}

/// Lock cursor when entering Playing state
fn lock_cursor(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut controller_query: Query<&mut FpsController>,
) {
    let Ok(mut window) = window_query.single_mut() else {
        return;
    };
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
    for mut controller in &mut controller_query {
        controller.enable_input = true;
        }
    }

/// Unlock cursor when exiting Playing state
fn unlock_cursor(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut controller_query: Query<&mut FpsController>,
) {
    let Ok(mut window) = window_query.single_mut() else {
        return;
    };
    window.cursor_options.grab_mode = CursorGrabMode::None;
    window.cursor_options.visible = true;
    for mut controller in &mut controller_query {
        controller.enable_input = false;
    }
}

fn manage_cursor(
    key: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => {
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                next_state.set(GameState::Playing);
            }
            GameState::MainMenu => {
                // Do nothing in main menu
            }
        }
    }
}
