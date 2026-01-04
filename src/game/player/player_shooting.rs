use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{prelude::*, plugin::ReadRapierContext};
use bevy_fps_controller::controller::RenderPlayer;

use crate::game::{
    level::targets::{DeadTarget, Target},
    shooting,
};

#[derive(Component)]
pub struct Shootable;

#[derive(Component)]
pub struct TracerSpawnSpot;

pub fn update_player(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    rapier_context: ReadRapierContext,
    camera_query: Query<(&Camera, &GlobalTransform), With<RenderPlayer>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    target_query: Query<Option<&Target>, With<Shootable>>,
    spawn_spot: Query<&GlobalTransform, With<TracerSpawnSpot>>,
) {
    let Ok(spawn_spot) = spawn_spot.single() else {
        return;
    };
    let Ok(window) = window_query.single() else {
        return;
    };
    let Ok((camera, camera_global_transform)) = camera_query.single() else {
        return;
    };
    
        if mouse_input.just_pressed(MouseButton::Left) {
        let Ok(ray) = camera.viewport_to_world(
            camera_global_transform,
                Vec2::new(window.width() / 2., window.height() / 2.),
            ) else {
                return;
            };
        let Ok(context) = rapier_context.single() else {
            return;
            };
        let predicate = |handle| target_query.get(handle).is_ok();
            let query_filter = QueryFilter::new().predicate(&predicate);
        let hit = context.cast_ray_and_get_normal(
                ray.origin,
                ray.direction.into(),
                f32::MAX,
                true,
                query_filter,
            );
            if let Some((entity, ray_intersection)) = hit {
                if let Ok(target) = target_query.get(entity) {
                if target.is_some() {
                        commands.entity(entity).insert(DeadTarget);
                    }
                }
            // Spawn tracer
                let tracer_material = StandardMaterial {
                    base_color: Color::srgb(1., 1., 0.),
                    unlit: true,
                    ..default()
                };

                commands.spawn((
                Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(0.1, 0.1, 1.0)))),
                MeshMaterial3d(materials.add(tracer_material)),
                Transform::from_translation(Vec3::splat(f32::MAX)),
                    shooting::tracer::BulletTracer::new(
                        spawn_spot.translation(),
                        ray_intersection.point,
                        300.,
                    ),
                ));
        }
    }
}
