use bevy::{app::AppExit, prelude::*};

use super::{style, MenuTab, PlayerSettings};
use crate::game::GameState;

pub struct SettingsTabPlugin;

impl Plugin for SettingsTabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_settings_tab).add_systems(
            Update,
            (
                toggle_settings_tab_visibility,
                handle_slider_interactions
                    .run_if(in_state(GameState::MainMenu).and(in_state(MenuTab::Settings))),
                update_slider_displays,
                handle_quit_button
                    .run_if(in_state(GameState::MainMenu).and(in_state(MenuTab::Settings))),
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
enum SliderFill {
    Sensitivity,
    Fov,
    Volume,
}

#[derive(Component)]
struct QuitGameButton;

// Colors
const OVERLAY_BG: Color = style::PANEL;
const SLIDER_BG: Color = Color::srgba(0.1, 0.1, 0.15, 1.0);
const SLIDER_FILL: Color = style::ACCENT;

fn setup_settings_tab(
    mut commands: Commands,
    settings: Res<PlayerSettings>,
    server: Res<AssetServer>,
) {
    // Settings tab overlay
    commands
        .spawn((
            SettingsTabRoot,
            Node {
                right: Val::Px(56.0),
                bottom: Val::Px(0.0),
                position_type: PositionType::Absolute,
                top: Val::Px(style::HEADER),
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
                    font: server.load("fonts/RobotoCondensed.ttf"),
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
                                    TextFont {
                                        font: server.load("fonts/RobotoCondensed.ttf"),
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                                label_row.spawn((
                                    SensitivityValue,
                                    Text::new(format!("{:.2}", settings.sensitivity)),
                                    TextFont {
                                        font: server.load("fonts/RobotoCondensed.ttf"),
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });

                            row.spawn((
                                SensitivitySlider,
                                bevy::ui::RelativeCursorPosition::default(),
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
                                SliderFill::Sensitivity,
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
                                    TextFont {
                                        font: server.load("fonts/RobotoCondensed.ttf"),
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                                label_row.spawn((
                                    FovValue,
                                    Text::new(format!("{:.0}°", settings.fov)),
                                    TextFont {
                                        font: server.load("fonts/RobotoCondensed.ttf"),
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });

                            row.spawn((
                                FovSlider,
                                bevy::ui::RelativeCursorPosition::default(),
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
                                SliderFill::Fov,
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
                                    TextFont {
                                        font: server.load("fonts/RobotoCondensed.ttf"),
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                                label_row.spawn((
                                    VolumeValue,
                                    Text::new(format!("{:.0}%", settings.master_volume * 100.0)),
                                    TextFont {
                                        font: server.load("fonts/RobotoCondensed.ttf"),
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });

                            row.spawn((
                                VolumeSlider,
                                bevy::ui::RelativeCursorPosition::default(),
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
                                SliderFill::Volume,
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
                    BackgroundColor(style::INK),
                    BorderRadius::all(Val::Px(6.0)),
                ))
                .with_child((
                    Text::new("Quit Game"),
                    TextFont {
                        font: server.load("fonts/RobotoCondensed.ttf"),
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
    sensitivity: Query<(&Interaction, &bevy::ui::RelativeCursorPosition), With<SensitivitySlider>>,
    fov: Query<(&Interaction, &bevy::ui::RelativeCursorPosition), With<FovSlider>>,
    volume: Query<(&Interaction, &bevy::ui::RelativeCursorPosition), With<VolumeSlider>>,
    mut settings: ResMut<PlayerSettings>,
) {
    for (interaction, cursor) in &sensitivity {
        if *interaction == Interaction::Pressed {
            if let Some(p) = cursor.normalized {
                settings.sensitivity = 0.1 + p.x.clamp(0., 1.) * 2.9;
            }
        }
    }
    for (interaction, cursor) in &fov {
        if *interaction == Interaction::Pressed {
            if let Some(p) = cursor.normalized {
                settings.fov = 60. + p.x.clamp(0., 1.) * 60.;
            }
        }
    }
    for (interaction, cursor) in &volume {
        if *interaction == Interaction::Pressed {
            if let Some(p) = cursor.normalized {
                settings.master_volume = p.x.clamp(0., 1.);
            }
        }
    }
}

fn update_slider_displays(
    settings: Res<PlayerSettings>,
    mut fills: Query<(&SliderFill, &mut Node)>,
    mut sens_query: Query<
        &mut Text,
        (
            With<SensitivityValue>,
            Without<FovValue>,
            Without<VolumeValue>,
        ),
    >,
    mut fov_query: Query<
        &mut Text,
        (
            With<FovValue>,
            Without<SensitivityValue>,
            Without<VolumeValue>,
        ),
    >,
    mut vol_query: Query<
        &mut Text,
        (
            With<VolumeValue>,
            Without<SensitivityValue>,
            Without<FovValue>,
        ),
    >,
) {
    if !settings.is_changed() {
        return;
    }

    for (kind, mut node) in &mut fills {
        node.width = Val::Percent(
            match kind {
                SliderFill::Sensitivity => (settings.sensitivity - 0.1) / 2.9,
                SliderFill::Fov => (settings.fov - 60.) / 60.,
                SliderFill::Volume => settings.master_volume,
            } * 100.,
        );
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
