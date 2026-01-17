use crate::game::config::GameConfig;
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
pub struct GameplayMapConfigHandle {
    handle: Handle<MapConfig>,
    base_path: String,
}

/// Resource holding the loaded gameplay map config
#[derive(Resource, Default)]
pub struct LoadedGameplayMapConfig {
    pub config: Option<MapConfig>,
    pub base_path: String,
}

/// Track if level has been initialized this session
#[derive(Resource, Default)]
pub struct LevelInitialized(bool);

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(targets::TargetsPlugin)
            .init_resource::<LoadedGameplayMapConfig>()
            .init_resource::<LevelInitialized>()
            .add_systems(OnEnter(GameState::Playing), start_loading_map_config)
            .add_systems(
                Update,
                (check_gameplay_map_config_loaded, init_level_when_ready)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_level);
    }
}

/// Start loading the map config when entering Playing state
fn start_loading_map_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut loaded_config: ResMut<LoadedGameplayMapConfig>,
    mut level_initialized: ResMut<LevelInitialized>,
) {
    // Reset state for new level
    loaded_config.config = None;
    loaded_config.base_path = String::new();
    level_initialized.0 = false;

    let config_path = game_config.map.config_path();
    // Derive base path by removing the filename (e.g., "maps/de_dust_2" from "maps/de_dust_2/config.map.ron")
    let base_path = config_path
        .rsplit_once('/')
        .map(|(base, _)| base.to_string())
        .unwrap_or_else(|| "maps/warehouse".to_string());

    println!("[DEBUG] Loading map config: {}", config_path);
    println!("[DEBUG] Base path: {}", base_path);
    println!("[DEBUG] Selected map: {:?}", game_config.map);

    let handle = asset_server.load(config_path);
    commands.insert_resource(GameplayMapConfigHandle { handle, base_path });
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

    let Some(handle_res) = handle else { return };

    if let Some(config) = configs.get(&handle_res.handle) {
        loaded_config.config = Some(config.clone());
        loaded_config.base_path = handle_res.base_path.clone();
    }
}

/// Initialize the level once the map config is loaded
fn init_level_when_ready(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gameplay_config: Res<LoadedGameplayMapConfig>,
    mut level_initialized: ResMut<LevelInitialized>,
) {
    // Only init once config is loaded and level hasn't been initialized yet
    if level_initialized.0 || gameplay_config.config.is_none() {
        return;
    }

    level_initialized.0 = true;

    let config = gameplay_config.config.as_ref().unwrap();
    let base_path = &gameplay_config.base_path;

    println!("[DEBUG] Level init - Map name: {}", config.name);
    println!("[DEBUG] Level init - Model: {}", config.model);
    println!("[DEBUG] Level init - Spawn points: {:?}", config.spawn_points);

    // Set clear color from config
    if let Some(clear_color) = config.clear_color.as_ref() {
        commands.insert_resource(ClearColor(clear_color.to_color()));
    } else {
        // Default sky color
        commands.insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92)));
    }

    // Load the map model using the dynamic base path
    let model_path = format!("{}/{}#Scene0", base_path, config.model);

    let map_transform = config.transform.to_transform();

    // Spawn map with AsyncSceneCollider to auto-generate colliders from mesh geometry
    commands.spawn((
        LevelEntity,
        SceneRoot(asset_server.load(&model_path)),
        map_transform,
        // Automatically generate TriMesh colliders for all meshes in the scene
        AsyncSceneCollider {
            shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
            named_shapes: Default::default(),
        },
    ));

    // Fallback floor far below the map - catches player if they fall through geometry
    commands.spawn((
        LevelEntity,
        Collider::cuboid(1000., 0.1, 1000.),
        Transform::from_xyz(0., -100., 0.),
    ));

    // Spawn lighting from config
    spawn_lighting(&mut commands, &config.lighting, LevelEntity);
}

fn cleanup_level(mut commands: Commands, query: Query<Entity, With<LevelEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
