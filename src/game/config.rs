use std::time::Duration;
use bevy::prelude::*;

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
    #[default]
    Freemode,
    // Future: GridShot, Tracking, Deathmatch, etc.
}

impl GameMode {
    pub fn name(&self) -> &'static str {
        match self {
            GameMode::Freemode => "Freemode",
        }
    }
}

/// Available maps
#[derive(Default, Clone, PartialEq, Debug)]
pub enum MapId {
    #[default]
    Default,
    // Future: Additional maps
}

impl MapId {
    pub fn name(&self) -> &'static str {
        match self {
            MapId::Default => "Default",
        }
    }
}

/// Match-specific settings
#[derive(Clone, Debug)]
pub struct MatchSettings {
    pub time_limit: Option<Duration>,  // None = unlimited
    pub score_limit: Option<u32>,      // None = unlimited  
    pub respawn_time: Duration,        // 0 = instant
}

impl Default for MatchSettings {
    fn default() -> Self {
        Self {
            time_limit: None,
            score_limit: None,
            respawn_time: Duration::ZERO,
        }
    }
}

