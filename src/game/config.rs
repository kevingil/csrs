use crate::game::player::skins::SkinId;
use bevy::prelude::*;
use std::time::Duration;

/// Main game configuration resource
#[derive(Resource, Clone)]
pub struct GameConfig {
    pub mode: GameMode,
    pub map: MapId,
    pub match_settings: MatchSettings,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            mode: GameMode::default(),
            map: MapId::default(),
            match_settings: MatchSettings::default(),
        }
    }
}

/// Available game modes
#[derive(Default, Clone, PartialEq, Debug)]
pub enum GameMode {
    Freemode,
    #[default]
    TeamDeathmatch,
    // Future: GridShot, Tracking, Deathmatch, etc.
}

impl GameMode {
    pub fn name(&self) -> &'static str {
        match self {
            GameMode::Freemode => "Freemode",
            GameMode::TeamDeathmatch => "Team Deathmatch",
        }
    }
}

/// Available maps
#[derive(Default, Clone, PartialEq, Debug)]
pub enum MapId {
    Warehouse,
    #[default]
    Dust2,
}

impl MapId {
    pub fn name(&self) -> &'static str {
        match self {
            MapId::Warehouse => "Warehouse",
            MapId::Dust2 => "Dust 2",
        }
    }

    /// Get the config file path for this map
    pub fn config_path(&self) -> &'static str {
        match self {
            MapId::Warehouse => "maps/warehouse/config.map.ron",
            MapId::Dust2 => "maps/de_dust_2/config.map.ron",
        }
    }
}

/// Match-specific settings
#[derive(Clone, Debug)]
pub struct MatchSettings {
    pub time_limit: Option<Duration>, // None = unlimited
    pub score_limit: Option<u32>,     // None = unlimited
    pub respawn_time: Duration,       // 0 = instant
}

impl Default for MatchSettings {
    fn default() -> Self {
        Self {
            time_limit: Some(Duration::from_secs(600)),
            score_limit: Some(50),
            respawn_time: Duration::from_secs(3),
        }
    }
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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WeaponId {
    AK47,
    DefaultKnife,
}

impl WeaponId {
    pub fn name(&self) -> &'static str {
        match self {
            WeaponId::AK47 => "AK-47",
            WeaponId::DefaultKnife => "Default Knife",
        }
    }

    pub fn all() -> Vec<WeaponId> {
        vec![WeaponId::AK47, WeaponId::DefaultKnife]
    }
}

/// Resource for player loadout
#[derive(Resource)]
pub struct PlayerLoadout {
    pub primary_weapon: WeaponId,
    pub selected_skin: SkinId,
    pub melee_weapon: WeaponId,
}

impl Default for PlayerLoadout {
    fn default() -> Self {
        Self {
            primary_weapon: WeaponId::AK47,
            melee_weapon: WeaponId::DefaultKnife,
            selected_skin: SkinId::default(),
        }
    }
}
