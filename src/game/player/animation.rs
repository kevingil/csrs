//! Layered animation system for player models.
//!
//! Uses a two-layer approach for competitive FPS gameplay:
//! - Lower body layer: Locomotion (idle, walk, run, crouch)
//! - Upper body layer: Weapon-specific animations (idle, reload, fire)
//!
//! All player skins share the same skeleton and animations for fairness.

use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;

use super::player_model::PlayerModel;

// ============================================================================
// TYPES & ENUMS
// ============================================================================

/// Weapon types that affect upper body animations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum WeaponType {
    #[default]
    Rifle,
    Pistol,
    Sniper,
    Knife,
}

impl WeaponType {
    /// Get all weapon types
    pub fn all() -> &'static [WeaponType] {
        &[WeaponType::Rifle, WeaponType::Pistol, WeaponType::Sniper, WeaponType::Knife]
    }

    /// Get the reload duration for this weapon type
    pub fn reload_duration(&self) -> f32 {
        match self {
            WeaponType::Rifle => 2.5,
            WeaponType::Pistol => 1.8,
            WeaponType::Sniper => 3.5,
            WeaponType::Knife => 0.0, // Knife doesn't reload
        }
    }

    /// Get the fire rate cooldown for this weapon type
    pub fn fire_cooldown(&self) -> f32 {
        match self {
            WeaponType::Rifle => 0.1,
            WeaponType::Pistol => 0.15,
            WeaponType::Sniper => 1.5,
            WeaponType::Knife => 0.5,
        }
    }
}

/// Lower body movement states (locomotion layer)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MovementState {
    #[default]
    Idle,
    Walking,
    Running,
    CrouchIdle,
    CrouchWalking,
}

impl MovementState {
    /// Get the animation node index for this movement state
    pub fn node_index(&self, animations: &SharedAnimations) -> AnimationNodeIndex {
        match self {
            MovementState::Idle => animations.lower_body.idle,
            MovementState::Walking => animations.lower_body.walk,
            MovementState::Running => animations.lower_body.run,
            MovementState::CrouchIdle => animations.lower_body.crouch_idle,
            MovementState::CrouchWalking => animations.lower_body.crouch_walk,
        }
    }
}

/// Upper body action states (weapon layer)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UpperBodyAction {
    #[default]
    Idle,
    Reloading,
    Firing,
}

// ============================================================================
// ANIMATION INDICES
// ============================================================================

/// Path to the standard animation model containing skeleton and all animations.
pub const ANIMATION_MODEL_PATH: &str = "models/player_animations.glb";

/// Animation clip indices in the GLB file.
/// Lower body animations (universal across all weapons)
pub mod lower_body_indices {
    pub const IDLE: usize = 0;
    pub const WALK: usize = 1;
    pub const RUN: usize = 2;
    pub const CROUCH_IDLE: usize = 3;
    pub const CROUCH_WALK: usize = 4;
}

/// Upper body animation indices per weapon type
pub mod upper_body_indices {
    pub mod rifle {
        pub const IDLE: usize = 5;
        pub const RELOAD: usize = 6;
        pub const FIRE: usize = 7;
    }
    pub mod pistol {
        pub const IDLE: usize = 8;
        pub const RELOAD: usize = 9;
        pub const FIRE: usize = 10;
    }
    pub mod sniper {
        pub const IDLE: usize = 11;
        pub const RELOAD: usize = 12;
        pub const FIRE: usize = 13;
    }
    pub mod knife {
        pub const IDLE: usize = 14;
        pub const ATTACK: usize = 15;
    }
}

/// Total number of animation clips expected
pub const TOTAL_ANIMATION_COUNT: usize = 16;

// ============================================================================
// ANIMATION NODES
// ============================================================================

/// Animation nodes for lower body locomotion
#[derive(Debug, Clone)]
pub struct LowerBodyNodes {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub run: AnimationNodeIndex,
    pub crouch_idle: AnimationNodeIndex,
    pub crouch_walk: AnimationNodeIndex,
}

/// Animation nodes for a single weapon's upper body animations
#[derive(Debug, Clone)]
pub struct WeaponAnimNodes {
    pub idle: AnimationNodeIndex,
    pub reload: AnimationNodeIndex,
    pub fire: AnimationNodeIndex,
}

/// All weapon animation nodes
#[derive(Debug, Clone)]
pub struct UpperBodyNodes {
    pub rifle: WeaponAnimNodes,
    pub pistol: WeaponAnimNodes,
    pub sniper: WeaponAnimNodes,
    pub knife: WeaponAnimNodes,
}

impl UpperBodyNodes {
    /// Get animation nodes for a specific weapon type
    pub fn for_weapon(&self, weapon: WeaponType) -> &WeaponAnimNodes {
        match weapon {
            WeaponType::Rifle => &self.rifle,
            WeaponType::Pistol => &self.pistol,
            WeaponType::Sniper => &self.sniper,
            WeaponType::Knife => &self.knife,
        }
    }
}

// ============================================================================
// SHARED ANIMATIONS RESOURCE
// ============================================================================

/// Shared animation data - loaded once, used by all players.
#[derive(Resource)]
pub struct SharedAnimations {
    /// The animation graph handle
    pub graph: Handle<AnimationGraph>,
    /// Lower body (locomotion) animation nodes
    pub lower_body: LowerBodyNodes,
    /// Upper body (weapon) animation nodes
    pub upper_body: UpperBodyNodes,
}

// ============================================================================
// PLAYER ANIMATION CONTROLLER
// ============================================================================

/// Per-player animation controller component.
/// Tracks both lower body (movement) and upper body (weapon) animation states.
#[derive(Component)]
pub struct PlayerAnimationController {
    // Lower body state (locomotion)
    pub movement: MovementState,
    pub prev_movement: MovementState,
    pub is_crouching: bool,
    
    // Upper body state (weapon)
    pub weapon: WeaponType,
    pub prev_weapon: WeaponType,
    pub upper_action: UpperBodyAction,
    pub prev_upper_action: UpperBodyAction,
    
    // Action timers
    pub reload_timer: f32,
    pub fire_timer: f32,
    
    // Animation player entity reference (set after scene loads)
    pub animation_player_entity: Option<Entity>,
}

impl Default for PlayerAnimationController {
    fn default() -> Self {
        Self {
            movement: MovementState::Idle,
            prev_movement: MovementState::Idle,
            is_crouching: false,
            weapon: WeaponType::Rifle,
            prev_weapon: WeaponType::Rifle,
            upper_action: UpperBodyAction::Idle,
            prev_upper_action: UpperBodyAction::Idle,
            reload_timer: 0.0,
            fire_timer: 0.0,
            animation_player_entity: None,
        }
    }
}

impl PlayerAnimationController {
    /// Check if lower body state changed
    pub fn movement_changed(&self) -> bool {
        self.movement != self.prev_movement
    }
    
    /// Check if upper body state changed (action or weapon)
    pub fn upper_body_changed(&self) -> bool {
        self.upper_action != self.prev_upper_action || self.weapon != self.prev_weapon
    }
    
    /// Get the current upper body animation node
    pub fn upper_body_node(&self, animations: &SharedAnimations) -> AnimationNodeIndex {
        let weapon_nodes = animations.upper_body.for_weapon(self.weapon);
        match self.upper_action {
            UpperBodyAction::Idle => weapon_nodes.idle,
            UpperBodyAction::Reloading => weapon_nodes.reload,
            UpperBodyAction::Firing => weapon_nodes.fire,
        }
    }
}

// ============================================================================
// CONSTANTS
// ============================================================================

/// Velocity thresholds for movement state detection
const IDLE_THRESHOLD: f32 = 0.5;
const WALK_THRESHOLD: f32 = 4.0;

/// Blend duration for animation transitions
const BLEND_DURATION: f32 = 0.2;

// ============================================================================
// SYSTEMS
// ============================================================================

/// Load shared animations from the standard animation model.
/// Called once at startup.
pub fn load_shared_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Load all animation clips from the standard model
    let clips: Vec<Handle<AnimationClip>> = (0..TOTAL_ANIMATION_COUNT)
        .map(|i| asset_server.load(GltfAssetLabel::Animation(i).from_asset(ANIMATION_MODEL_PATH)))
        .collect();
    
    // Build animation graph from clips
    let (graph, indices) = AnimationGraph::from_clips(clips);
    let graph_handle = graphs.add(graph);
    
    // Map indices to named nodes
    let lower_body = LowerBodyNodes {
        idle: indices[lower_body_indices::IDLE],
        walk: indices[lower_body_indices::WALK],
        run: indices[lower_body_indices::RUN],
        crouch_idle: indices[lower_body_indices::CROUCH_IDLE],
        crouch_walk: indices[lower_body_indices::CROUCH_WALK],
    };
    
    let upper_body = UpperBodyNodes {
        rifle: WeaponAnimNodes {
            idle: indices[upper_body_indices::rifle::IDLE],
            reload: indices[upper_body_indices::rifle::RELOAD],
            fire: indices[upper_body_indices::rifle::FIRE],
        },
        pistol: WeaponAnimNodes {
            idle: indices[upper_body_indices::pistol::IDLE],
            reload: indices[upper_body_indices::pistol::RELOAD],
            fire: indices[upper_body_indices::pistol::FIRE],
        },
        sniper: WeaponAnimNodes {
            idle: indices[upper_body_indices::sniper::IDLE],
            reload: indices[upper_body_indices::sniper::RELOAD],
            fire: indices[upper_body_indices::sniper::FIRE],
        },
        knife: WeaponAnimNodes {
            idle: indices[upper_body_indices::knife::IDLE],
            reload: indices[upper_body_indices::knife::ATTACK], // Knife uses attack instead of reload
            fire: indices[upper_body_indices::knife::ATTACK],   // Knife attack for both
        },
    };
    
    commands.insert_resource(SharedAnimations {
        graph: graph_handle,
        lower_body,
        upper_body,
    });
}

/// Detect animation state based on player velocity and input.
pub fn detect_animation_state(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut controller_query: Query<(&mut PlayerAnimationController, &PlayerModel)>,
    velocity_query: Query<&Velocity, With<LogicalPlayer>>,
) {
    for (mut controller, player_model) in &mut controller_query {
        let delta = time.delta_secs();
        
        // Store previous states for change detection
        controller.prev_movement = controller.movement;
        controller.prev_upper_action = controller.upper_action;
        controller.prev_weapon = controller.weapon;
        
        // --- Crouch Detection ---
        controller.is_crouching = keyboard.pressed(KeyCode::ControlLeft) 
            || keyboard.pressed(KeyCode::ControlRight);
        
        // --- Weapon Switching ---
        if keyboard.just_pressed(KeyCode::Digit1) {
            controller.weapon = WeaponType::Rifle;
        } else if keyboard.just_pressed(KeyCode::Digit2) {
            controller.weapon = WeaponType::Pistol;
        } else if keyboard.just_pressed(KeyCode::Digit3) {
            controller.weapon = WeaponType::Sniper;
        } else if keyboard.just_pressed(KeyCode::Digit4) {
            controller.weapon = WeaponType::Knife;
        }
        
        // --- Upper Body Action State ---
        // Update timers
        if controller.reload_timer > 0.0 {
            controller.reload_timer -= delta;
        }
        if controller.fire_timer > 0.0 {
            controller.fire_timer -= delta;
        }
        
        // Reload input (R key)
        if keyboard.just_pressed(KeyCode::KeyR) 
            && controller.upper_action != UpperBodyAction::Reloading 
            && controller.weapon != WeaponType::Knife 
        {
            controller.upper_action = UpperBodyAction::Reloading;
            controller.reload_timer = controller.weapon.reload_duration();
        }
        
        // Fire input (Left mouse)
        if mouse.just_pressed(MouseButton::Left) && controller.fire_timer <= 0.0 {
            controller.upper_action = UpperBodyAction::Firing;
            controller.fire_timer = controller.weapon.fire_cooldown();
        }
        
        // Return to idle when action completes
        if controller.upper_action == UpperBodyAction::Reloading && controller.reload_timer <= 0.0 {
            controller.upper_action = UpperBodyAction::Idle;
        }
        if controller.upper_action == UpperBodyAction::Firing && controller.fire_timer <= 0.0 {
            controller.upper_action = UpperBodyAction::Idle;
        }
        
        // --- Lower Body Movement State ---
        let Ok(velocity) = velocity_query.get(player_model.logical_entity) else {
            continue;
        };
        
        let horizontal_velocity = Vec2::new(velocity.linvel.x, velocity.linvel.z);
        let speed = horizontal_velocity.length();
        
        controller.movement = if controller.is_crouching {
            if speed < IDLE_THRESHOLD {
                MovementState::CrouchIdle
            } else {
                MovementState::CrouchWalking
            }
        } else if speed < IDLE_THRESHOLD {
            MovementState::Idle
        } else if speed < WALK_THRESHOLD {
            MovementState::Walking
        } else {
            MovementState::Running
        };
    }
}

/// Update animation players based on state changes.
/// 
/// Note: In a full implementation, this would use bone masks to blend
/// upper and lower body separately. For now, we prioritize upper body
/// actions (reload/fire) over locomotion when they're active.
pub fn update_player_animations(
    shared_animations: Option<Res<SharedAnimations>>,
    controller_query: Query<&PlayerAnimationController, Changed<PlayerAnimationController>>,
    mut animation_player_query: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    let Some(animations) = shared_animations else {
        return;
    };

    for controller in &controller_query {
        let Some(anim_entity) = controller.animation_player_entity else {
            continue;
        };

        let Ok((mut player, mut transitions)) = animation_player_query.get_mut(anim_entity) else {
            continue;
        };

        // Determine which animation to play
        // Priority: Firing > Reloading > Movement
        let (target_node, should_loop) = match controller.upper_action {
            UpperBodyAction::Firing => {
                (controller.upper_body_node(&animations), false)
            }
            UpperBodyAction::Reloading => {
                (controller.upper_body_node(&animations), false)
            }
            UpperBodyAction::Idle => {
                // When idle, blend based on movement
                // For now, just play movement animation
                // TODO: Implement proper bone masking for simultaneous upper/lower
                if controller.movement_changed() || controller.upper_body_changed() {
                    (controller.movement.node_index(&animations), true)
                } else {
                    continue;
                }
            }
        };

        // Apply the animation
        let transition = transitions.play(
            &mut player,
            target_node,
            Duration::from_secs_f32(BLEND_DURATION),
        );
        
        if should_loop {
            transition.repeat();
        }
    }
}

/// Setup animation player for newly spawned player models.
pub fn setup_animation_player(
    shared_animations: Option<Res<SharedAnimations>>,
    mut commands: Commands,
    mut animation_player_query: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    parent_query: Query<&ChildOf>,
    mut player_model_query: Query<(Entity, &mut PlayerAnimationController), With<PlayerModel>>,
) {
    let Some(animations) = shared_animations else {
        return;
    };

    for (anim_entity, mut player) in &mut animation_player_query {
        // Walk up hierarchy to find PlayerModel
        let mut current = anim_entity;
        let mut found_player_model: Option<Entity> = None;
        
        while let Ok(parent) = parent_query.get(current) {
            let parent_entity = parent.parent();
            for (pm_entity, _) in &player_model_query {
                if pm_entity == parent_entity {
                    found_player_model = Some(pm_entity);
                    break;
                }
            }
            if found_player_model.is_some() {
                break;
            }
            current = parent_entity;
        }

        let Some(player_model_entity) = found_player_model else {
            continue;
        };

        // Store animation player reference
        if let Ok((_, mut controller)) = player_model_query.get_mut(player_model_entity) {
            controller.animation_player_entity = Some(anim_entity);
        }

        // Initialize with idle animation
        let mut transitions = AnimationTransitions::new();
        transitions
            .play(&mut player, animations.lower_body.idle, Duration::from_secs_f32(BLEND_DURATION))
            .repeat();

        commands.entity(anim_entity).insert((
            AnimationGraphHandle(animations.graph.clone()),
            transitions,
        ));
    }
}
