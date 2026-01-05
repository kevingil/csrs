//! Full-body animation system for player models using Mixamo animations.
//!
//! Uses complete body animations (not split upper/lower) for smooth transitions.
//! All player skins share the same Mixamo skeleton and animations.

use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::LogicalPlayer;

use super::player_model::PlayerModel;

// ============================================================================
// ANIMATION STATE
// ============================================================================

/// Player animation state - determines which full-body animation to play
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum AnimationState {
    #[default]
    Idle,
    Walking,
    WalkingBackward,
    Running,
    RunningBackward,
    StrafeLeft,
    StrafeRight,
    Firing,
    Jumping,
    JumpingBackward,
    Dying,
}

impl AnimationState {
    /// Whether this animation should loop
    pub fn should_loop(&self) -> bool {
        match self {
            AnimationState::Idle => true,
            AnimationState::Walking => true,
            AnimationState::WalkingBackward => true,
            AnimationState::Running => true,
            AnimationState::RunningBackward => true,
            AnimationState::StrafeLeft => true,
            AnimationState::StrafeRight => true,
            AnimationState::Firing => false,
            AnimationState::Jumping => false,
            AnimationState::JumpingBackward => false,
            AnimationState::Dying => false,
        }
    }
}

// ============================================================================
// ANIMATION INDICES
// ============================================================================

/// Path to the animation model containing all animations.
pub const ANIMATION_MODEL_PATH: &str = "models/player_animations.glb";

/// Animation clip indices matching NLA track order in player_animations.glb
/// Order: top to bottom in NLA Editor = index 0 to N
pub mod animation_indices {
    pub const RIFLE_FIRING: usize = 0;
    pub const RIFLE_JUMP_BACKWARD: usize = 1;
    pub const RIFLE_JUMP: usize = 2;
    pub const RIFLE_AIM_IDLE: usize = 3;
    pub const RIFLE_RUN: usize = 4;
    pub const RIFLE_RUN_BACKWARD: usize = 5;
    pub const RIFLE_START_WALKING: usize = 6;
    pub const RIFLE_START_WALKING_BACKWARD: usize = 7;
    pub const RIFLE_STOP_WALKING: usize = 8;
    pub const RIFLE_STRAFE_LEFT: usize = 9;
    pub const RIFLE_STRAFE_RIGHT: usize = 10;
    pub const RIFLE_STOP_WALKING_BACKWARD: usize = 11;
    pub const RIFLE_WALKING: usize = 12;
    pub const RIFLE_WALK_BACKWARD: usize = 13;
    pub const RIFLE_DIE_FORWARD: usize = 14;
    pub const RIFLE_HOME_IDLE: usize = 15;
}

/// Total number of animation clips
pub const TOTAL_ANIMATION_COUNT: usize = 16;

// ============================================================================
// SHARED ANIMATIONS RESOURCE
// ============================================================================

/// Animation node indices for each state
#[derive(Debug, Clone)]
pub struct AnimationNodes {
    pub idle: AnimationNodeIndex,
    pub walking: AnimationNodeIndex,
    pub walking_backward: AnimationNodeIndex,
    pub running: AnimationNodeIndex,
    pub running_backward: AnimationNodeIndex,
    pub strafe_left: AnimationNodeIndex,
    pub strafe_right: AnimationNodeIndex,
    pub firing: AnimationNodeIndex,
    pub jumping: AnimationNodeIndex,
    pub jumping_backward: AnimationNodeIndex,
    pub dying: AnimationNodeIndex,
    pub home_idle: AnimationNodeIndex,
}

/// Shared animation data - loaded once, used by all players.
#[derive(Resource)]
pub struct SharedAnimations {
    /// The animation graph handle
    pub graph: Handle<AnimationGraph>,
    /// Animation nodes for each state
    pub nodes: AnimationNodes,
}

impl SharedAnimations {
    /// Get the animation node for a given state
    pub fn get_node(&self, state: AnimationState) -> AnimationNodeIndex {
        match state {
            AnimationState::Idle => self.nodes.idle,
            AnimationState::Walking => self.nodes.walking,
            AnimationState::WalkingBackward => self.nodes.walking_backward,
            AnimationState::Running => self.nodes.running,
            AnimationState::RunningBackward => self.nodes.running_backward,
            AnimationState::StrafeLeft => self.nodes.strafe_left,
            AnimationState::StrafeRight => self.nodes.strafe_right,
            AnimationState::Firing => self.nodes.firing,
            AnimationState::Jumping => self.nodes.jumping,
            AnimationState::JumpingBackward => self.nodes.jumping_backward,
            AnimationState::Dying => self.nodes.dying,
        }
    }
}

// ============================================================================
// PLAYER ANIMATION CONTROLLER
// ============================================================================

/// Per-player animation controller component.
#[derive(Component)]
pub struct PlayerAnimationController {
    /// Current animation state
    pub state: AnimationState,
    /// Previous state for change detection
    pub prev_state: AnimationState,
    /// Animation player entity reference
    pub animation_player_entity: Option<Entity>,
    /// Timer for one-shot animations (firing, etc.)
    pub action_timer: f32,
}

impl Default for PlayerAnimationController {
    fn default() -> Self {
        Self {
            state: AnimationState::Idle,
            prev_state: AnimationState::Idle,
            animation_player_entity: None,
            action_timer: 0.0,
        }
    }
}

impl PlayerAnimationController {
    /// Check if state changed
    pub fn state_changed(&self) -> bool {
        self.state != self.prev_state
    }
}

// ============================================================================
// CONSTANTS
// ============================================================================

/// Velocity thresholds for movement detection
const IDLE_THRESHOLD: f32 = 0.5;
const WALK_THRESHOLD: f32 = 4.0;

/// Animation transition blend duration
const BLEND_DURATION: f32 = 0.15;

/// Fire animation duration
const FIRE_DURATION: f32 = 0.3;

// ============================================================================
// SYSTEMS
// ============================================================================

/// Load shared animations from the animation model.
pub fn load_shared_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    use animation_indices::*;
    
    // Load all animation clips
    let clips: Vec<Handle<AnimationClip>> = (0..TOTAL_ANIMATION_COUNT)
        .map(|i| asset_server.load(GltfAssetLabel::Animation(i).from_asset(ANIMATION_MODEL_PATH)))
        .collect();
    
    // Build animation graph
    let (graph, indices) = AnimationGraph::from_clips(clips);
    let graph_handle = graphs.add(graph);
    
    // Map indices to animation nodes
    let nodes = AnimationNodes {
        idle: indices[RIFLE_AIM_IDLE],
        walking: indices[RIFLE_WALKING],
        walking_backward: indices[RIFLE_WALK_BACKWARD],
        running: indices[RIFLE_RUN],
        running_backward: indices[RIFLE_RUN_BACKWARD],
        strafe_left: indices[RIFLE_STRAFE_LEFT],
        strafe_right: indices[RIFLE_STRAFE_RIGHT],
        firing: indices[RIFLE_FIRING],
        jumping: indices[RIFLE_JUMP],
        jumping_backward: indices[RIFLE_JUMP_BACKWARD],
        dying: indices[RIFLE_DIE_FORWARD],
        home_idle: indices[RIFLE_HOME_IDLE],
    };
    
    commands.insert_resource(SharedAnimations {
        graph: graph_handle,
        nodes,
    });
}

/// Detect animation state based on player input and velocity.
pub fn detect_animation_state(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut controller_query: Query<(&mut PlayerAnimationController, &PlayerModel)>,
    velocity_query: Query<&Velocity, With<LogicalPlayer>>,
) {
    for (mut controller, player_model) in &mut controller_query {
        let delta = time.delta_secs();
        
        // Store previous state
        controller.prev_state = controller.state;
        
        // Update action timer
        if controller.action_timer > 0.0 {
            controller.action_timer -= delta;
        }
        
        // Check for firing (takes priority)
        if mouse.just_pressed(MouseButton::Left) {
            controller.state = AnimationState::Firing;
            controller.action_timer = FIRE_DURATION;
            continue;
        }
        
        // If in one-shot animation, wait for it to finish
        if controller.action_timer > 0.0 {
            continue;
        }
        
        // Get velocity for movement detection
        let Ok(velocity) = velocity_query.get(player_model.logical_entity) else {
            controller.state = AnimationState::Idle;
            continue;
        };
        
        let horizontal_vel = Vec2::new(velocity.linvel.x, velocity.linvel.z);
        let speed = horizontal_vel.length();
        
        // Determine movement state based on velocity and input
        if speed < IDLE_THRESHOLD {
            controller.state = AnimationState::Idle;
        } else {
            // Check movement direction based on input
            let moving_forward = keyboard.pressed(KeyCode::KeyW);
            let moving_backward = keyboard.pressed(KeyCode::KeyS);
            let moving_left = keyboard.pressed(KeyCode::KeyA);
            let moving_right = keyboard.pressed(KeyCode::KeyD);
            let is_running = speed >= WALK_THRESHOLD;
            
            if moving_left && !moving_right {
                controller.state = AnimationState::StrafeLeft;
            } else if moving_right && !moving_left {
                controller.state = AnimationState::StrafeRight;
            } else if moving_backward && !moving_forward {
                controller.state = if is_running {
                    AnimationState::RunningBackward
                } else {
                    AnimationState::WalkingBackward
                };
            } else {
                controller.state = if is_running {
                    AnimationState::Running
                } else {
                    AnimationState::Walking
                };
            }
        }
    }
}

/// Update animation players based on state changes.
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

        // Get the target animation node
        let target_node = animations.get_node(controller.state);
        
        // Play the animation with smooth transition
        let transition = transitions.play(
            &mut player,
            target_node,
            Duration::from_secs_f32(BLEND_DURATION),
        );
        
        // Loop if appropriate
        if controller.state.should_loop() {
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
            .play(&mut player, animations.nodes.idle, Duration::from_secs_f32(BLEND_DURATION))
            .repeat();

        commands.entity(anim_entity).insert((
            AnimationGraphHandle(animations.graph.clone()),
            transitions,
        ));
    }
}

// ============================================================================
// LEGACY COMPATIBILITY (for other modules that may reference old types)
// ============================================================================

/// Weapon types (kept for compatibility with other systems)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum WeaponType {
    #[default]
    Rifle,
    Pistol,
    Sniper,
    Knife,
}

impl WeaponType {
    pub fn reload_duration(&self) -> f32 {
        match self {
            WeaponType::Rifle => 2.5,
            WeaponType::Pistol => 1.8,
            WeaponType::Sniper => 3.5,
            WeaponType::Knife => 0.0,
        }
    }

    pub fn fire_cooldown(&self) -> f32 {
        match self {
            WeaponType::Rifle => 0.1,
            WeaponType::Pistol => 0.15,
            WeaponType::Sniper => 1.5,
            WeaponType::Knife => 0.5,
        }
    }
}
