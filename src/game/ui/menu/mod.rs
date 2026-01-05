use bevy::prelude::*;

use crate::game::config::MapId;
use crate::game::player::skins::{SkinId, SkinRegistry};

pub mod debug_widgets;
pub mod home_scene;
pub mod home_tab;
pub mod inventory_tab;
pub mod nav_bar;
pub mod play_tab;
pub mod settings_tab;

/// Menu tab state - which tab is currently active
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MenuTab {
    #[default]
    Home,
    Play,
    Inventory,
    Settings,
}

/// Resource tracking the currently selected map (if any)
#[derive(Resource, Default)]
pub struct SelectedMap {
    pub map: Option<MapId>,
    pub ready_to_start: bool,
}

/// Resource for player settings
#[derive(Resource)]
pub struct PlayerSettings {
    pub sensitivity: f32,
    pub fov: f32,
    pub master_volume: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,
            fov: 103.0,
            master_volume: 1.0,
        }
    }
}

/// Available weapons for loadout
#[derive(Clone, PartialEq, Debug)]
pub enum WeaponId {
    AK47,
    M4A1,
    AWP,
}

impl WeaponId {
    pub fn name(&self) -> &'static str {
        match self {
            WeaponId::AK47 => "AK-47",
            WeaponId::M4A1 => "M4A1",
            WeaponId::AWP => "AWP",
        }
    }

    pub fn all() -> Vec<WeaponId> {
        vec![WeaponId::AK47, WeaponId::M4A1, WeaponId::AWP]
    }
}

/// Resource for player loadout
#[derive(Resource)]
pub struct PlayerLoadout {
    pub primary_weapon: WeaponId,
    pub selected_skin: SkinId,
}

impl Default for PlayerLoadout {
    fn default() -> Self {
        Self {
            primary_weapon: WeaponId::AK47,
            selected_skin: SkinId::default(),
        }
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuTab>()
            .init_resource::<SelectedMap>()
            .init_resource::<PlayerSettings>()
            .init_resource::<PlayerLoadout>()
            .init_resource::<SkinRegistry>()
            .add_plugins((
                nav_bar::NavBarPlugin,
                home_scene::HomeScenePlugin,
                home_tab::HomeTabPlugin,
                play_tab::PlayTabPlugin,
                inventory_tab::InventoryTabPlugin,
                settings_tab::SettingsTabPlugin,
            ));
    }
}

