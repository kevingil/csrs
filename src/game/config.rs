use std::time::Duration;
use bevy::core_pipeline::tonemapping::Tonemapping;
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

/// Post-processing settings for visual effects
#[derive(Clone, Debug)]
pub struct PostProcessSettings {
    pub bloom_intensity: f32,
    pub bloom_threshold: f32,
    pub tonemapping: Tonemapping,
    pub contrast: f32,
    pub saturation: f32,
}

impl Default for PostProcessSettings {
    fn default() -> Self {
        Self {
            bloom_intensity: 0.05, // Subtle bloom - was 0.3
            bloom_threshold: 1.5,  // Higher threshold = only bright areas bloom
            tonemapping: Tonemapping::TonyMcMapface,
            contrast: 1.0,
            saturation: 1.0,
        }
    }
}

/// Configuration for a specific map including transforms and effects
#[derive(Resource, Clone, Debug)]
pub struct MapConfig {
    /// Player spawn position in world coordinates
    pub player_spawn: Vec3,
    /// Map model transform (position, scale, rotation)
    pub map_position: Vec3,
    pub map_scale: f32,
    pub map_rotation: Quat,
    /// Post-processing effects for this map
    pub post_process: PostProcessSettings,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            player_spawn: Vec3::new(0.0, 2.0, 0.0),
            map_position: Vec3::ZERO,
            map_scale: 1.0,
            map_rotation: Quat::IDENTITY,
            post_process: PostProcessSettings::default(),
        }
    }
}

