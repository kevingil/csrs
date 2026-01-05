use crate::game::config::MapConfig;
use crate::game::GameState;

use super::targets;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Marker component for level entities (for cleanup)
#[derive(Component)]
pub struct LevelEntity;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(targets::TargetsPlugin)
            // Light blue sky color
            .insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92)))
            // Ambient light for overall illumination
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 500.0,
                affects_lightmapped_meshes: true,
            })
            .add_systems(OnEnter(GameState::Playing), init_level)
            .add_systems(OnExit(GameState::Playing), cleanup_level);
    }
}

fn init_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_config: Res<MapConfig>,
) {
    // Load the map model using MapConfig transform settings
    commands.spawn((
        LevelEntity,
        SceneRoot(asset_server.load("models/map/warehouse_map.glb#Scene0")),
        Transform::from_translation(map_config.map_position)
            .with_scale(Vec3::splat(map_config.map_scale))
            .with_rotation(map_config.map_rotation),
    ));

    // Invisible floor collider for physics (large flat plane)
    commands.spawn((
        LevelEntity,
        Collider::cuboid(500., 0.1, 500.),
        Transform::from_xyz(0., -0.1, 0.),
    ));

    // Directional sunlight
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

fn cleanup_level(mut commands: Commands, query: Query<Entity, With<LevelEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
