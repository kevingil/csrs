use bevy::{app::AppExit, prelude::*};

use super::{MenuTab, PlayerSettings};
use crate::game::GameState;

pub struct SettingsTabPlugin;

impl Plugin for SettingsTabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_settings_tab)
            .add_systems(
                Update,
                (
                    toggle_settings_tab_visibility,
                    handle_slider_interactions,
                    update_slider_displays,
                    handle_quit_button,
                ),
            );
    }
}

#[derive(Component)]
struct SettingsTabRoot;

#[derive(Component)]
struct SensitivitySlider;

#[derive(Component)]
struct SensitivityValue;

#[derive(Component)]
struct FovSlider;

#[derive(Component)]
struct FovValue;

#[derive(Component)]
struct VolumeSlider;

#[derive(Component)]
struct VolumeValue;

#[derive(Component)]
struct QuitGameButton;

// Colors
const OVERLAY_BG: Color = Color::srgba(0.02, 0.02, 0.05, 0.92);
const SLIDER_BG: Color = Color::srgba(0.1, 0.1, 0.15, 1.0);
const SLIDER_FILL: Color = Color::srgb(0.2, 0.5, 0.7);

fn setup_settings_tab(mut commands: Commands, settings: Res<PlayerSettings>) {
    // Settings tab overlay
    commands
        .spawn((
            SettingsTabRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                top: Val::Px(70.0),
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(40.0)),
                row_gap: Val::Px(30.0),
                ..default()
            },
            BackgroundColor(OVERLAY_BG),
            GlobalZIndex(150),
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("SETTINGS"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Settings container
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(25.0),
                    max_width: Val::Px(500.0),
                    ..default()
                })
                .with_children(|container| {
                    // Sensitivity slider
                    let sens_normalized = (settings.sensitivity - 0.1) / (3.0 - 0.1);
                    container
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            })
                            .with_children(|label_row| {
                                label_row.spawn((
                                    Text::new("Mouse Sensitivity"),
                                    TextFont { font_size: 18.0, ..default() },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                                label_row.spawn((
                                    SensitivityValue,
                                    Text::new(format!("{:.2}", settings.sensitivity)),
                                    TextFont { font_size: 18.0, ..default() },
                                    TextColor(Color::WHITE),
                                ));
                            });

                            row.spawn((
                                SensitivitySlider,
                                Button,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(20.0),
                                    ..default()
                                },
                                BackgroundColor(SLIDER_BG),
                                BorderRadius::all(Val::Px(4.0)),
                            ))
                            .with_child((
                                Node {
                                    width: Val::Percent(sens_normalized * 100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(SLIDER_FILL),
                                BorderRadius::left(Val::Px(4.0)),
                            ));
                        });

                    // FOV slider
                    let fov_normalized = (settings.fov - 60.0) / (120.0 - 60.0);
                    container
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            })
                            .with_children(|label_row| {
                                label_row.spawn((
                                    Text::new("Field of View"),
                                    TextFont { font_size: 18.0, ..default() },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                                label_row.spawn((
                                    FovValue,
                                    Text::new(format!("{:.0}°", settings.fov)),
                                    TextFont { font_size: 18.0, ..default() },
                                    TextColor(Color::WHITE),
                                ));
                            });

                            row.spawn((
                                FovSlider,
                                Button,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(20.0),
                                    ..default()
                                },
                                BackgroundColor(SLIDER_BG),
                                BorderRadius::all(Val::Px(4.0)),
                            ))
                            .with_child((
                                Node {
                                    width: Val::Percent(fov_normalized * 100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(SLIDER_FILL),
                                BorderRadius::left(Val::Px(4.0)),
                            ));
                        });

                    // Volume slider
                    let vol_normalized = settings.master_volume;
                    container
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            })
                            .with_children(|label_row| {
                                label_row.spawn((
                                    Text::new("Master Volume"),
                                    TextFont { font_size: 18.0, ..default() },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                                label_row.spawn((
                                    VolumeValue,
                                    Text::new(format!("{:.0}%", settings.master_volume * 100.0)),
                                    TextFont { font_size: 18.0, ..default() },
                                    TextColor(Color::WHITE),
                                ));
                            });

                            row.spawn((
                                VolumeSlider,
                                Button,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(20.0),
                                    ..default()
                                },
                                BackgroundColor(SLIDER_BG),
                                BorderRadius::all(Val::Px(4.0)),
                            ))
                            .with_child((
                                Node {
                                    width: Val::Percent(vol_normalized * 100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(SLIDER_FILL),
                                BorderRadius::left(Val::Px(4.0)),
                            ));
                        });
                });

            // Spacer
            parent.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });

            // Quit game button
            parent
                .spawn((
                    QuitGameButton,
                    Button,
                    Node {
                        width: Val::Px(180.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.2, 0.2)),
                    BorderRadius::all(Val::Px(6.0)),
                ))
                .with_child((
                    Text::new("Quit Game"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
        });
}

fn toggle_settings_tab_visibility(
    game_state: Res<State<GameState>>,
    menu_tab: Res<State<MenuTab>>,
    mut settings_query: Query<&mut Visibility, With<SettingsTabRoot>>,
) {
    let Ok(mut visibility) = settings_query.single_mut() else {
        return;
    };

    let should_show =
        *game_state.get() == GameState::MainMenu && *menu_tab.get() == MenuTab::Settings;

    *visibility = if should_show {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

fn handle_slider_interactions(
    sensitivity_query: Query<(&Interaction, &Node, &GlobalTransform), With<SensitivitySlider>>,
    fov_query: Query<
        (&Interaction, &Node, &GlobalTransform),
        (With<FovSlider>, Without<SensitivitySlider>),
    >,
    volume_query: Query<
        (&Interaction, &Node, &GlobalTransform),
        (With<VolumeSlider>, Without<SensitivitySlider>, Without<FovSlider>),
    >,
    windows: Query<&Window>,
    mut settings: ResMut<PlayerSettings>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Handle sensitivity slider
    for (interaction, node, transform) in &sensitivity_query {
        if *interaction == Interaction::Pressed {
            if let Some(new_value) = calculate_slider_value(cursor_pos, node, transform, 0.1, 3.0) {
                settings.sensitivity = new_value;
            }
        }
    }

    // Handle FOV slider
    for (interaction, node, transform) in &fov_query {
        if *interaction == Interaction::Pressed {
            if let Some(new_value) = calculate_slider_value(cursor_pos, node, transform, 60.0, 120.0) {
                settings.fov = new_value;
            }
        }
    }

    // Handle volume slider
    for (interaction, node, transform) in &volume_query {
        if *interaction == Interaction::Pressed {
            if let Some(new_value) = calculate_slider_value(cursor_pos, node, transform, 0.0, 1.0) {
                settings.master_volume = new_value;
            }
        }
    }
}

fn calculate_slider_value(
    cursor_pos: Vec2,
    node: &Node,
    transform: &GlobalTransform,
    min: f32,
    max: f32,
) -> Option<f32> {
    let width = match node.width {
        Val::Px(w) => w,
        Val::Percent(p) => p * 5.0,
        _ => return None,
    };

    let left = transform.translation().x - width / 2.0;
    let normalized = ((cursor_pos.x - left) / width).clamp(0.0, 1.0);
    Some(min + normalized * (max - min))
}

fn update_slider_displays(
    settings: Res<PlayerSettings>,
    mut sens_query: Query<&mut Text, (With<SensitivityValue>, Without<FovValue>, Without<VolumeValue>)>,
    mut fov_query: Query<&mut Text, (With<FovValue>, Without<SensitivityValue>, Without<VolumeValue>)>,
    mut vol_query: Query<&mut Text, (With<VolumeValue>, Without<SensitivityValue>, Without<FovValue>)>,
) {
    if !settings.is_changed() {
        return;
    }

    if let Ok(mut text) = sens_query.single_mut() {
        **text = format!("{:.2}", settings.sensitivity);
    }
    if let Ok(mut text) = fov_query.single_mut() {
        **text = format!("{:.0}°", settings.fov);
    }
    if let Ok(mut text) = vol_query.single_mut() {
        **text = format!("{:.0}%", settings.master_volume * 100.0);
    }
}

fn handle_quit_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitGameButton>)>,
    mut exit_events: EventWriter<AppExit>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            exit_events.write(AppExit::Success);
        }
    }
}
