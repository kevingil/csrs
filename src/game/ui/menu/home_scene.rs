use bevy::{prelude::*, render::camera::Exposure};
use std::fs::OpenOptions;
use std::io::Write;

use crate::game::player::skins::{SkinId, SkinRegistry};
use crate::game::ui::menu::PlayerLoadout;
use crate::game::GameState;

pub struct HomeScenePlugin;

impl Plugin for HomeScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugTarget>()
            .init_resource::<DebugPanelState>()
            .add_systems(OnEnter(GameState::MainMenu), (setup_home_scene, setup_debug_ui))
            .add_systems(OnExit(GameState::MainMenu), cleanup_home_scene)
            .add_systems(
                Update,
                (
                    handle_debug_buttons,
                    update_debug_display,
                    keyboard_debug_controls,
                    handle_debug_toggle,
                    handle_debug_drag,
                    update_debug_panel_position,
                )
                    .run_if(in_state(GameState::MainMenu)),
            );
    }
}

/// Marker for home scene entities
#[derive(Component)]
pub struct HomeSceneEntity;

/// Marker for the home scene 3D camera (separate from game camera)
#[derive(Component)]
pub struct HomeSceneCamera;

/// Marker for the rotating player model
#[derive(Component)]
struct HomePlayerModel;

/// Marker for the armory scene
#[derive(Component)]
struct ArmoryScene;

/// Debug UI root
#[derive(Component)]
struct DebugUiRoot;

/// Debug panel content (collapsible part)
#[derive(Component)]
struct DebugPanelContent;

/// Debug panel header (draggable)
#[derive(Component)]
struct DebugPanelHeader;

/// Toggle button for collapse/expand
#[derive(Component)]
struct DebugToggleButton;

/// Debug position display text
#[derive(Component)]
struct DebugDisplayText;

/// State for the debug panel
#[derive(Resource)]
struct DebugPanelState {
    expanded: bool,
    position: Vec2,
    dragging: bool,
    drag_offset: Vec2,
}

impl Default for DebugPanelState {
    fn default() -> Self {
        Self {
            expanded: true,
            position: Vec2::new(10.0, 80.0),
            dragging: false,
            drag_offset: Vec2::ZERO,
        }
    }
}

/// Which entity is being controlled
#[derive(Resource, Default, PartialEq, Clone, Copy)]
enum DebugTarget {
    #[default]
    Character,
    Scene,
    Camera,
}

/// Debug button actions
#[derive(Component, Clone, Copy)]
enum DebugButton {
    // Target selection
    SelectCharacter,
    SelectScene,
    SelectCamera,
    // Position adjustments
    PosXPlus,
    PosXMinus,
    PosYPlus,
    PosYMinus,
    PosZPlus,
    PosZMinus,
    // Scale adjustments
    ScalePlus,
    ScaleMinus,
    // Rotation adjustments
    RotYPlus,
    RotYMinus,
    // Actions
    SavePositions,
    ResetPositions,
}

// Button colors
const BTN_NORMAL: Color = Color::srgba(0.2, 0.2, 0.3, 0.9);
const BTN_HOVER: Color = Color::srgba(0.3, 0.3, 0.5, 0.9);
const BTN_ACTIVE: Color = Color::srgba(0.2, 0.6, 0.3, 0.9);

macro_rules! spawn_debug_button {
    ($parent:expr, $label:expr, $action:expr) => {
        $parent
            .spawn((
                $action,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(BTN_NORMAL),
                BorderRadius::all(Val::Px(4.0)),
            ))
            .with_child((
                Text::new($label),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::WHITE),
            ))
    };
}

fn setup_home_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    loadout: Res<PlayerLoadout>,
    skin_registry: Res<SkinRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Get selected skin (falls back to default if not found)
    let skin_def = skin_registry
        .get(loadout.selected_skin)
        .unwrap_or_else(|| skin_registry.get(SkinId::default()).unwrap());

    // Custom FOV for menu scene
    let menu_fov = 60.0_f32.to_radians();

    // 3D Camera for the home scene
    commands.spawn((
        HomeSceneEntity,
        HomeSceneCamera,
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: menu_fov,
            near: 0.1,
            far: 1000.0,
            ..default()
        }),
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(0.0, 1.5, 4.0).with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 0.0), 0.1244)),
        Exposure::INDOOR,
    ));

    // Armory background scene
    commands.spawn((
        HomeSceneEntity,
        ArmoryScene,
        SceneRoot(asset_server.load("models/armory.glb#Scene0")),
        Transform::from_xyz(2.8618, 0.0, 2.4632)
            .with_scale(Vec3::splat(1.0))
            .with_rotation(Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 0.6154)),
    ));

    // Platform for player to stand on
    let platform_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.2),
        metallic: 0.8,
        perceptual_roughness: 0.3,
        ..default()
    });

    commands.spawn((
        HomeSceneEntity,
        Mesh3d(meshes.add(Cylinder::new(2.5, 0.2))),
        MeshMaterial3d(platform_material),
        Transform::from_xyz(0.0, -0.1, 0.0),
    ));

    // Player skin model (rotating showcase)
    commands.spawn((
        HomeSceneEntity,
        HomePlayerModel,
        SceneRoot(asset_server.load(skin_def.model_path)),
        Transform::from_xyz(0.0, 0.0, 1.6615)
            .with_scale(Vec3::splat(1.855458))
            .with_rotation(Quat::from_axis_angle(Vec3::new(0.0, -1.0, 0.0), 0.0332)),
        Visibility::Visible,
    ));

    // Dark room with single lightbulb effect
    // Very dim ambient for darkness
    commands.spawn((
        HomeSceneEntity,
        AmbientLight {
            color: Color::srgb(0.05, 0.05, 0.08),
            brightness: 20.0,
            affects_lightmapped_meshes: true,
        },
    ));

    // Single point light (lightbulb) - warm color, positioned above character
    commands.spawn((
        HomeSceneEntity,
        PointLight {
            color: Color::srgb(1.0, 0.9, 0.7), // Warm incandescent color
            intensity: 800_000.0,
            radius: 0.1,
            range: 15.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 3.5, 1.0),
    ));
}

fn setup_debug_ui(mut commands: Commands, panel_state: Res<DebugPanelState>) {
    // Debug UI panel - draggable window
    commands
        .spawn((
            HomeSceneEntity,
            DebugUiRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(panel_state.position.x),
                top: Val::Px(panel_state.position.y),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            GlobalZIndex(250),
        ))
        .with_children(|parent| {
            // Draggable header bar
            parent
                .spawn((
                    DebugPanelHeader,
                    Button,
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
                    BorderRadius::top(Val::Px(8.0)),
                ))
                .with_children(|header| {
                    // Title
                    header.spawn((
                        Text::new("DEBUG"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgb(1.0, 1.0, 0.0)),
                    ));

                    // Toggle button (collapse/expand)
                    header
                        .spawn((
                            DebugToggleButton,
                            Button,
                            Node {
                                padding: UiRect::axes(Val::Px(6.0), Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.3, 0.3, 0.4, 0.9)),
                            BorderRadius::all(Val::Px(4.0)),
                        ))
                        .with_child((
                            Text::new("▼"),
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(Color::WHITE),
                        ));
                });

            // Collapsible content panel
            parent
                .spawn((
                    DebugPanelContent,
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.0)),
                        row_gap: Val::Px(6.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
                    BorderRadius::bottom(Val::Px(8.0)),
                ))
                .with_children(|content| {
                    // Target selection row
                    content
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(4.0),
                            ..default()
                        })
                        .with_children(|row| {
                            spawn_debug_button!(row, "Char", DebugButton::SelectCharacter);
                            spawn_debug_button!(row, "Scene", DebugButton::SelectScene);
                            spawn_debug_button!(row, "Cam", DebugButton::SelectCamera);
                        });

                    // Position display
                    content.spawn((
                        DebugDisplayText,
                        Text::new("Loading..."),
                        TextFont { font_size: 11.0, ..default() },
                        TextColor(Color::WHITE),
                    ));

                    // Position X row
                    content
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(4.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                Text::new("X:"),
                                TextFont { font_size: 11.0, ..default() },
                                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                Node { width: Val::Px(35.0), ..default() },
                            ));
                            spawn_debug_button!(row, "-", DebugButton::PosXMinus);
                            spawn_debug_button!(row, "+", DebugButton::PosXPlus);
                        });

                    // Position Y row
                    content
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(4.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                Text::new("Y:"),
                                TextFont { font_size: 11.0, ..default() },
                                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                Node { width: Val::Px(35.0), ..default() },
                            ));
                            spawn_debug_button!(row, "-", DebugButton::PosYMinus);
                            spawn_debug_button!(row, "+", DebugButton::PosYPlus);
                        });

                    // Position Z row
                    content
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(4.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                Text::new("Z:"),
                                TextFont { font_size: 11.0, ..default() },
                                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                Node { width: Val::Px(35.0), ..default() },
                            ));
                            spawn_debug_button!(row, "-", DebugButton::PosZMinus);
                            spawn_debug_button!(row, "+", DebugButton::PosZPlus);
                        });

                    // Scale row
                    content
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(4.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                Text::new("Scale:"),
                                TextFont { font_size: 11.0, ..default() },
                                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                Node { width: Val::Px(35.0), ..default() },
                            ));
                            spawn_debug_button!(row, "-", DebugButton::ScaleMinus);
                            spawn_debug_button!(row, "+", DebugButton::ScalePlus);
                        });

                    // Rotation row
                    content
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(4.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                Text::new("Rot:"),
                                TextFont { font_size: 11.0, ..default() },
                                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                Node { width: Val::Px(35.0), ..default() },
                            ));
                            spawn_debug_button!(row, "-", DebugButton::RotYMinus);
                            spawn_debug_button!(row, "+", DebugButton::RotYPlus);
                        });

                    // Action buttons
                    content
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(4.0),
                            margin: UiRect::top(Val::Px(6.0)),
                            ..default()
                        })
                        .with_children(|row| {
                            spawn_debug_button!(row, "Save", DebugButton::SavePositions);
                            spawn_debug_button!(row, "Reset", DebugButton::ResetPositions);
                        });

                    // Keyboard help
                    content.spawn((
                        Text::new("WASD QE RF +/-"),
                        TextFont { font_size: 9.0, ..default() },
                        TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    ));
                });
        });
}

fn handle_debug_buttons(
    mut interaction_query: Query<
        (&Interaction, &DebugButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut target: ResMut<DebugTarget>,
    mut char_query: Query<
        &mut Transform,
        (With<HomePlayerModel>, Without<ArmoryScene>, Without<HomeSceneCamera>),
    >,
    mut scene_query: Query<
        &mut Transform,
        (With<ArmoryScene>, Without<HomePlayerModel>, Without<HomeSceneCamera>),
    >,
    mut camera_query: Query<
        &mut Transform,
        (With<HomeSceneCamera>, Without<HomePlayerModel>, Without<ArmoryScene>),
    >,
) {
    let move_amount = 0.1;
    let scale_factor = 1.1;
    let rot_amount = 0.1; // radians

    for (interaction, button, mut bg) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(BTN_ACTIVE);

                match button {
                    DebugButton::SelectCharacter => *target = DebugTarget::Character,
                    DebugButton::SelectScene => *target = DebugTarget::Scene,
                    DebugButton::SelectCamera => *target = DebugTarget::Camera,
                    DebugButton::SavePositions => {
                        save_positions(&char_query, &scene_query, &camera_query);
                    }
                    DebugButton::ResetPositions => {
                        reset_positions(&mut char_query, &mut scene_query, &mut camera_query);
                    }
                    _ => {
                        // Position/scale/rotation adjustments
                        let transform = match *target {
                            DebugTarget::Character => char_query.iter_mut().next(),
                            DebugTarget::Scene => scene_query.iter_mut().next(),
                            DebugTarget::Camera => camera_query.iter_mut().next(),
                        };

                        if let Some(mut t) = transform {
                            match button {
                                DebugButton::PosXPlus => t.translation.x += move_amount,
                                DebugButton::PosXMinus => t.translation.x -= move_amount,
                                DebugButton::PosYPlus => t.translation.y += move_amount,
                                DebugButton::PosYMinus => t.translation.y -= move_amount,
                                DebugButton::PosZPlus => t.translation.z += move_amount,
                                DebugButton::PosZMinus => t.translation.z -= move_amount,
                                DebugButton::ScalePlus => t.scale *= scale_factor,
                                DebugButton::ScaleMinus => t.scale /= scale_factor,
                                DebugButton::RotYPlus => t.rotate_y(rot_amount),
                                DebugButton::RotYMinus => t.rotate_y(-rot_amount),
                                _ => {}
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(BTN_HOVER);
            }
            Interaction::None => {
                // Highlight active target button
                let is_active = matches!(
                    (button, *target),
                    (DebugButton::SelectCharacter, DebugTarget::Character)
                        | (DebugButton::SelectScene, DebugTarget::Scene)
                        | (DebugButton::SelectCamera, DebugTarget::Camera)
                );
                *bg = if is_active {
                    BackgroundColor(BTN_ACTIVE)
                } else {
                    BackgroundColor(BTN_NORMAL)
                };
            }
        }
    }
}

fn keyboard_debug_controls(
    keys: Res<ButtonInput<KeyCode>>,
    target: Res<DebugTarget>,
    mut char_query: Query<
        &mut Transform,
        (With<HomePlayerModel>, Without<ArmoryScene>, Without<HomeSceneCamera>),
    >,
    mut scene_query: Query<
        &mut Transform,
        (With<ArmoryScene>, Without<HomePlayerModel>, Without<HomeSceneCamera>),
    >,
    mut camera_query: Query<
        &mut Transform,
        (With<HomeSceneCamera>, Without<HomePlayerModel>, Without<ArmoryScene>),
    >,
    time: Res<Time>,
) {
    let move_speed = 2.0 * time.delta_secs();
    let scale_speed = 1.5 * time.delta_secs();

    let transform = match *target {
        DebugTarget::Character => char_query.iter_mut().next(),
        DebugTarget::Scene => scene_query.iter_mut().next(),
        DebugTarget::Camera => camera_query.iter_mut().next(),
    };

    let rot_speed = 2.0 * time.delta_secs();

    if let Some(mut t) = transform {
        // WASD for X/Z
        if keys.pressed(KeyCode::KeyW) {
            t.translation.z -= move_speed;
        }
        if keys.pressed(KeyCode::KeyS) {
            t.translation.z += move_speed;
        }
        if keys.pressed(KeyCode::KeyA) {
            t.translation.x -= move_speed;
        }
        if keys.pressed(KeyCode::KeyD) {
            t.translation.x += move_speed;
        }
        // Q/E for Y
        if keys.pressed(KeyCode::KeyQ) {
            t.translation.y -= move_speed;
        }
        if keys.pressed(KeyCode::KeyE) {
            t.translation.y += move_speed;
        }
        // +/- for scale
        if keys.pressed(KeyCode::Equal) || keys.pressed(KeyCode::NumpadAdd) {
            t.scale *= 1.0 + scale_speed;
        }
        if keys.pressed(KeyCode::Minus) || keys.pressed(KeyCode::NumpadSubtract) {
            t.scale *= 1.0 - scale_speed;
        }
        // R/F for rotation
        if keys.pressed(KeyCode::KeyR) {
            t.rotate_y(rot_speed);
        }
        if keys.pressed(KeyCode::KeyF) {
            t.rotate_y(-rot_speed);
        }
    }

    // Tab to cycle targets
    if keys.just_pressed(KeyCode::Tab) {
        // Note: Can't mutate target here since we borrowed it immutably
        // This would need a separate system or different approach
    }
}

fn update_debug_display(
    target: Res<DebugTarget>,
    char_query: Query<
        &Transform,
        (With<HomePlayerModel>, Without<ArmoryScene>, Without<HomeSceneCamera>),
    >,
    scene_query: Query<
        &Transform,
        (With<ArmoryScene>, Without<HomePlayerModel>, Without<HomeSceneCamera>),
    >,
    camera_query: Query<
        &Transform,
        (With<HomeSceneCamera>, Without<HomePlayerModel>, Without<ArmoryScene>),
    >,
    mut text_query: Query<&mut Text, With<DebugDisplayText>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    let target_name = match *target {
        DebugTarget::Character => "CHARACTER",
        DebugTarget::Scene => "SCENE",
        DebugTarget::Camera => "CAMERA",
    };

    let (pos, scale, rot_y) = match *target {
        DebugTarget::Character => char_query
            .iter()
            .next()
            .map(|t| {
                let (_, y, _) = t.rotation.to_euler(EulerRot::YXZ);
                (t.translation, t.scale.x, y)
            })
            .unwrap_or((Vec3::ZERO, 1.0, 0.0)),
        DebugTarget::Scene => scene_query
            .iter()
            .next()
            .map(|t| {
                let (_, y, _) = t.rotation.to_euler(EulerRot::YXZ);
                (t.translation, t.scale.x, y)
            })
            .unwrap_or((Vec3::ZERO, 1.0, 0.0)),
        DebugTarget::Camera => camera_query
            .iter()
            .next()
            .map(|t| {
                let (_, y, _) = t.rotation.to_euler(EulerRot::YXZ);
                (t.translation, t.scale.x, y)
            })
            .unwrap_or((Vec3::ZERO, 1.0, 0.0)),
    };

    **text = format!(
        "Target: {}\nPos: ({:.3}, {:.3}, {:.3})\nScale: {:.4}\nRot Y: {:.2}°",
        target_name, pos.x, pos.y, pos.z, scale, rot_y.to_degrees()
    );
}

fn save_positions(
    char_query: &Query<
        &mut Transform,
        (With<HomePlayerModel>, Without<ArmoryScene>, Without<HomeSceneCamera>),
    >,
    scene_query: &Query<
        &mut Transform,
        (With<ArmoryScene>, Without<HomePlayerModel>, Without<HomeSceneCamera>),
    >,
    camera_query: &Query<
        &mut Transform,
        (With<HomeSceneCamera>, Without<HomePlayerModel>, Without<ArmoryScene>),
    >,
) {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let mut output = format!("=== Home Scene Positions ({})\n\n", timestamp);

    if let Some(t) = char_query.iter().next() {
        let (axis, angle) = t.rotation.to_axis_angle();
        output.push_str(&format!(
            "CHARACTER:\n  position: Vec3::new({:.4}, {:.4}, {:.4})\n  scale: Vec3::splat({:.6})\n  rotation: Quat::from_axis_angle(Vec3::new({:.4}, {:.4}, {:.4}), {:.4})\n\n",
            t.translation.x, t.translation.y, t.translation.z, t.scale.x,
            axis.x, axis.y, axis.z, angle
        ));
    }

    if let Some(t) = scene_query.iter().next() {
        let (axis, angle) = t.rotation.to_axis_angle();
        output.push_str(&format!(
            "SCENE (Armory):\n  position: Vec3::new({:.4}, {:.4}, {:.4})\n  scale: Vec3::splat({:.6})\n  rotation: Quat::from_axis_angle(Vec3::new({:.4}, {:.4}, {:.4}), {:.4})\n\n",
            t.translation.x, t.translation.y, t.translation.z, t.scale.x,
            axis.x, axis.y, axis.z, angle
        ));
    }

    if let Some(t) = camera_query.iter().next() {
        let (axis, angle) = t.rotation.to_axis_angle();
        output.push_str(&format!(
            "CAMERA:\n  position: Vec3::new({:.4}, {:.4}, {:.4})\n  rotation: Quat::from_axis_angle(Vec3::new({:.4}, {:.4}, {:.4}), {:.4})\n\n",
            t.translation.x, t.translation.y, t.translation.z,
            axis.x, axis.y, axis.z, angle
        ));
    }

    // Generate copy-paste ready code
    output.push_str("// Copy-paste ready code:\n");
    if let Some(t) = char_query.iter().next() {
        let (axis, angle) = t.rotation.to_axis_angle();
        output.push_str(&format!(
            "CHARACTER: Transform::from_xyz({:.4}, {:.4}, {:.4}).with_scale(Vec3::splat({:.6})).with_rotation(Quat::from_axis_angle(Vec3::new({:.4}, {:.4}, {:.4}), {:.4}))\n",
            t.translation.x, t.translation.y, t.translation.z, t.scale.x,
            axis.x, axis.y, axis.z, angle
        ));
    }
    if let Some(t) = scene_query.iter().next() {
        let (axis, angle) = t.rotation.to_axis_angle();
        output.push_str(&format!(
            "SCENE: Transform::from_xyz({:.4}, {:.4}, {:.4}).with_scale(Vec3::splat({:.6})).with_rotation(Quat::from_axis_angle(Vec3::new({:.4}, {:.4}, {:.4}), {:.4}))\n",
            t.translation.x, t.translation.y, t.translation.z, t.scale.x,
            axis.x, axis.y, axis.z, angle
        ));
    }

    // Save to file
    let path = "home_scene_positions.log";
    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", output) {
                eprintln!("Failed to write positions: {}", e);
            } else {
                println!("Positions saved to {}", path);
                println!("{}", output);
            }
        }
        Err(e) => {
            eprintln!("Failed to open log file: {}", e);
            // Still print to console
            println!("{}", output);
        }
    }
}

fn reset_positions(
    char_query: &mut Query<
        &mut Transform,
        (With<HomePlayerModel>, Without<ArmoryScene>, Without<HomeSceneCamera>),
    >,
    scene_query: &mut Query<
        &mut Transform,
        (With<ArmoryScene>, Without<HomePlayerModel>, Without<HomeSceneCamera>),
    >,
    camera_query: &mut Query<
        &mut Transform,
        (With<HomeSceneCamera>, Without<HomePlayerModel>, Without<ArmoryScene>),
    >,
) {
    for mut t in char_query.iter_mut() {
        t.translation = Vec3::new(0.0, 0.0, 1.6615);
        t.scale = Vec3::splat(1.855458);
        t.rotation = Quat::from_axis_angle(Vec3::new(0.0, -1.0, 0.0), 0.0332);
    }
    for mut t in scene_query.iter_mut() {
        t.translation = Vec3::new(2.8618, 0.0, 2.4632);
        t.scale = Vec3::splat(1.0);
        t.rotation = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 0.6154);
    }
    for mut t in camera_query.iter_mut() {
        *t = Transform::from_xyz(0.0, 1.5, 4.0)
            .with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 0.0), 0.1244));
    }
}

fn handle_debug_toggle(
    mut panel_state: ResMut<DebugPanelState>,
    toggle_query: Query<&Interaction, (Changed<Interaction>, With<DebugToggleButton>)>,
    mut content_query: Query<&mut Visibility, With<DebugPanelContent>>,
) {
    for interaction in &toggle_query {
        if *interaction == Interaction::Pressed {
            panel_state.expanded = !panel_state.expanded;

            // Update content visibility
            if let Ok(mut visibility) = content_query.single_mut() {
                *visibility = if panel_state.expanded {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}

fn handle_debug_drag(
    mut panel_state: ResMut<DebugPanelState>,
    header_query: Query<&Interaction, With<DebugPanelHeader>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    for interaction in &header_query {
        match *interaction {
            Interaction::Pressed => {
                if !panel_state.dragging {
                    panel_state.dragging = true;
                    panel_state.drag_offset = cursor_pos - panel_state.position;
                }
            }
            _ => {}
        }
    }

    if panel_state.dragging {
        if mouse_button.pressed(MouseButton::Left) {
            panel_state.position = cursor_pos - panel_state.drag_offset;
            // Clamp to window bounds
            panel_state.position.x = panel_state.position.x.max(0.0);
            panel_state.position.y = panel_state.position.y.max(0.0);
        } else {
            panel_state.dragging = false;
        }
    }
}

fn update_debug_panel_position(
    panel_state: Res<DebugPanelState>,
    mut root_query: Query<&mut Node, With<DebugUiRoot>>,
) {
    if !panel_state.is_changed() {
        return;
    }

    if let Ok(mut node) = root_query.single_mut() {
        node.left = Val::Px(panel_state.position.x);
        node.top = Val::Px(panel_state.position.y);
    }
}

fn cleanup_home_scene(mut commands: Commands, query: Query<Entity, With<HomeSceneEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
