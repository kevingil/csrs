use crate::game::map::{MapConfig, spawn_lighting};
use crate::game::GameState;

use super::targets;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Marker component for level entities (for cleanup)
#[derive(Component, Clone)]
pub struct LevelEntity;

/// Resource to track the gameplay map config loading
#[derive(Resource)]
pub struct GameplayMapConfigHandle(Handle<MapConfig>);

/// Resource holding the loaded gameplay map config
#[derive(Resource, Default)]
pub struct LoadedGameplayMapConfig {
    pub config: Option<MapConfig>,
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(targets::TargetsPlugin)
            .init_resource::<LoadedGameplayMapConfig>()
            .add_systems(Startup, load_gameplay_map_config)
            .add_systems(Update, check_gameplay_map_config_loaded)
            .add_systems(OnEnter(GameState::Playing), init_level)
            .add_systems(OnExit(GameState::Playing), cleanup_level);
    }
}

/// Load the gameplay map config on startup
fn load_gameplay_map_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("maps/warehouse/config.map.ron");
    commands.insert_resource(GameplayMapConfigHandle(handle));
}

/// Check if gameplay map config is loaded and store it
fn check_gameplay_map_config_loaded(
    mut loaded_config: ResMut<LoadedGameplayMapConfig>,
    handle: Option<Res<GameplayMapConfigHandle>>,
    configs: Res<Assets<MapConfig>>,
) {
    if loaded_config.config.is_some() {
        return;
    }
    
    let Some(handle) = handle else { return };
    
    if let Some(config) = configs.get(&handle.0) {
        loaded_config.config = Some(config.clone());
    }
}

fn init_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gameplay_config: Res<LoadedGameplayMapConfig>,
) {
    // Get config or use defaults
    let config = gameplay_config.config.as_ref();
    
    // Set clear color from config
    if let Some(clear_color) = config.and_then(|c| c.clear_color.as_ref()) {
        commands.insert_resource(ClearColor(clear_color.to_color()));
    } else {
        // Default sky color
        commands.insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92)));
    }
    
    // Load the map model
    let model_path = config
        .map(|c| format!("maps/warehouse/{}#Scene0", c.model))
        .unwrap_or_else(|| "maps/warehouse/warehouse_map.glb#Scene0".to_string());
    
    let map_transform = config
        .map(|c| c.transform.to_transform())
        .unwrap_or_default();
    
    commands.spawn((
        LevelEntity,
        SceneRoot(asset_server.load(&model_path)),
        map_transform,
    ));

    // Invisible floor collider for physics (large flat plane)
    commands.spawn((
        LevelEntity,
        Collider::cuboid(500., 0.1, 500.),
        Transform::from_xyz(0., -0.1, 0.),
    ));

    // Spawn lighting from config
    if let Some(config) = config {
        spawn_lighting(&mut commands, &config.lighting, LevelEntity);
    } else {
        // Fallback lighting
        commands.spawn((
            LevelEntity,
            AmbientLight {
                color: Color::WHITE,
                brightness: 500.0,
                affects_lightmapped_meshes: true,
            },
        ));
        
        commands.spawn((
            LevelEntity,
            DirectionalLight {
                illuminance: light_consts::lux::FULL_DAYLIGHT,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(100., 200., 100.).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }
}

fn cleanup_level(mut commands: Commands, query: Query<Entity, With<LevelEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
