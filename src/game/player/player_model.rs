//! Player model component and hitbox markers.
//!
//! The PlayerModel component links a visible character mesh to its physics body.
//! Hitbox markers are used for damage calculation when raycasting.

use bevy::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;

use super::skins::HitboxZoneType;

/// Height of the physics collider cylinder
pub const COLLIDER_HEIGHT: f32 = 3.0;

/// Component for the visible player model that follows the LogicalPlayer.
/// This is separate from the physics body to support multiplayer where:
/// - Local player: model is hidden (first-person view)
/// - Remote players: only the model is visible (their physics runs on their machine)
#[derive(Component)]
pub struct PlayerModel {
    pub logical_entity: Entity,
    pub is_local_player: bool,
}

/// Marker component for hitbox colliders attached to player models.
/// Used for damage calculation when raycasting hits a player.
#[derive(Component)]
pub struct HitboxZoneMarker {
    /// The type of hitbox zone (head, torso, legs)
    pub zone_type: HitboxZoneType,
    /// The player entity this hitbox belongs to
    pub player_entity: Entity,
}

impl HitboxZoneMarker {
    /// Get the damage multiplier for this hitbox zone
    pub fn damage_multiplier(&self) -> f32 {
        self.zone_type.damage_multiplier()
    }
}

/// System to sync PlayerModel transforms with their LogicalPlayer positions.
/// Only syncs position and yaw (body doesn't tilt with camera pitch).
pub fn sync_player_model(
    logical_query: Query<&Transform, With<LogicalPlayer>>,
    mut model_query: Query<(&PlayerModel, &mut Transform, &mut Visibility), Without<LogicalPlayer>>,
) {
    for (model, mut model_transform, mut visibility) in &mut model_query {
        if let Ok(logical_transform) = logical_query.get(model.logical_entity) {
            // Copy position from logical player
            model_transform.translation = logical_transform.translation;
            // Offset down so feet touch ground (collider center vs mesh origin)
            model_transform.translation.y -= COLLIDER_HEIGHT / 2.0;
            
            // Copy only yaw rotation (body doesn't tilt with head pitch)
            // The logical player's rotation is locked, so we'd need to get yaw from FpsControllerInput
            // For now, just keep the model upright
        }
        
        // Hide local player's model from their own camera
        *visibility = if model.is_local_player {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}
