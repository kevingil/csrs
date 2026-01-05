use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use bevy_fps_controller::controller::FpsControllerPlugin;

use super::{config::GameConfig, level::level, player::player, ui::ui, window::window};

/// Game state for menu and gameplay flow
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<GameConfig>()
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default().disabled(),
                FpsControllerPlugin,
                level::LevelPlugin,
                player::PlayerPlugin,
                window::WindowSettingsPlugin,
                ui::UiPlugin,
            ));
    }
}
