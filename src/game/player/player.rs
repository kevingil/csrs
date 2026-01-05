use std::f32::consts::TAU;

use bevy::{prelude::*, render::camera::Exposure};
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::*;

use super::animation::{
    PlayerAnimationController, 
    detect_animation_state, 
    load_shared_animations,
    setup_animation_player,
    update_player_animations,
};
use super::player_shooting::TracerSpawnSpot;
use super::player_model::{PlayerModel, HitboxZoneMarker, COLLIDER_HEIGHT};
use super::skins::{SkinRegistry, HitboxZoneType, STANDARD_HITBOX};
use crate::game::math::coordinates::blender_to_world;
use crate::game::ui::menu::PlayerLoadout;
use crate::game::GameState;

pub struct PlayerPlugin;

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.625, 0.0);

/// Marker component for player-related entities (for cleanup)
#[derive(Component)]
pub struct PlayerEntity;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::game::shooting::tracer::TracerPlugin)
            // Load shared animations at startup
            .add_systems(Startup, load_shared_animations)
            // Animation systems
            .add_systems(Update, (
                setup_animation_player,
                detect_animation_state,
                update_player_animations,
                super::player_shooting::update_player,
                respawn,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), init_player)
            .add_systems(OnExit(GameState::Playing), cleanup_player);
    }
}

fn init_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    loadout: Res<PlayerLoadout>,
    skin_registry: Res<SkinRegistry>,
) {
    let fov = 103.0_f32.to_radians();

    // Get selected skin definition
    let skin_def = skin_registry
        .get(loadout.selected_skin)
        .expect("Selected skin should exist in registry");

    // Entity 1: LogicalPlayer (Physics body - invisible)
    let logical_entity = commands
        .spawn((
            PlayerEntity,
            Collider::cylinder(COLLIDER_HEIGHT / 2.0, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true },
            Transform::from_translation(SPAWN_POINT),
        ))
        .insert((
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                air_acceleration: 80.0,
                ..default()
            },
            CameraConfig {
                height_offset: -0.5,
            },
        ))
        .id();

    // Gun model - will be child of camera
    let gun_model = asset_server.load("models/weapons/ak.glb#Scene0");
    let gun_entity = commands.spawn((PlayerEntity, SceneRoot(gun_model))).id();

    // Tracer spawn spot - child of camera
    let spawn_spot = blender_to_world(Vec3::new(0.530462, 2.10557, -0.466568));
    let tracer_spawn_entity = commands
        .spawn((
            PlayerEntity,
            Transform::from_translation(spawn_spot),
            Visibility::default(),
            TracerSpawnSpot,
        ))
        .id();

    // Entity 2: RenderPlayer (Camera - first person view)
    commands
        .spawn((
            PlayerEntity,
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection {
                fov,
                ..default()
            }),
            Exposure::SUNLIGHT,
            RenderPlayer { logical_entity },
        ))
        .add_children(&[tracer_spawn_entity, gun_entity]);

    // Entity 3: PlayerModel (Visible character mesh)
    // Load the selected skin model
    let skin_model = asset_server.load(skin_def.model_path);
    let player_model_entity = commands.spawn((
        PlayerEntity,
        PlayerModel {
            logical_entity,
            is_local_player: true,
        },
        PlayerAnimationController::default(),
        SceneRoot(skin_model),
        Transform::default(),
        Visibility::Hidden, // Hidden for local player in first-person
    )).id();

    // Spawn hitbox colliders as children of the player model
    // These are sensor colliders (don't affect physics, only for hit detection)
    spawn_hitbox_colliders(&mut commands, player_model_entity, logical_entity);
}

/// Spawn hitbox sensor colliders as children of the player model entity.
/// Uses the STANDARD_HITBOX configuration for competitive fairness.
fn spawn_hitbox_colliders(
    commands: &mut Commands,
    player_model_entity: Entity,
    logical_entity: Entity,
) {
    // Spawn hitbox for each zone type
    let zones = [
        (HitboxZoneType::Head, &STANDARD_HITBOX.head),
        (HitboxZoneType::Torso, &STANDARD_HITBOX.torso),
        (HitboxZoneType::Legs, &STANDARD_HITBOX.legs),
    ];

    for (zone_type, zone) in zones {
        let hitbox_entity = commands.spawn((
            PlayerEntity,
            // Sensor collider - doesn't affect physics, only for raycasting
            Collider::cuboid(
                zone.half_extents.x,
                zone.half_extents.y,
                zone.half_extents.z,
            ),
            Sensor,
            // Position relative to player model
            Transform::from_translation(zone.offset),
            // Marker for hit detection
            HitboxZoneMarker {
                zone_type,
                player_entity: logical_entity,
            },
        )).id();

        // Make hitbox a child of the player model so it moves with it
        commands.entity(player_model_entity).add_child(hitbox_entity);
    }
}

fn cleanup_player(mut commands: Commands, query: Query<Entity, With<PlayerEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn respawn(mut query: Query<(&mut Transform, &mut Velocity), With<LogicalPlayer>>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }
        velocity.linvel = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}
