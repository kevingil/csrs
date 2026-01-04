use bevy::prelude::*;

use super::{MenuTab, SelectedMap};
use crate::game::{config::GameConfig, GameState};

pub struct HomeTabPlugin;

impl Plugin for HomeTabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_home_tab)
            .add_systems(
                Update,
                (
                    toggle_home_tab_visibility,
                    update_selected_info,
                    handle_start_button,
                ),
            );
    }
}

#[derive(Component)]
struct HomeTabRoot;

#[derive(Component)]
struct SelectedMapText;

#[derive(Component)]
struct SelectedModeText;

#[derive(Component)]
struct StartGameButton;

#[derive(Component)]
struct StartGameContainer;

fn setup_home_tab(mut commands: Commands) {
    // Home tab UI overlay (transparent, shows 3D scene behind)
    commands
        .spawn((
            HomeTabRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                top: Val::Px(70.0), // Below nav bar
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("CSRS"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.9)),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(40.0),
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn((
                Text::new("Aim Trainer"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgba(0.7, 0.7, 0.7, 0.8)),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(120.0),
                    ..default()
                },
            ));

            // Bottom container for selected info and start button
            parent
                .spawn((
                    StartGameContainer,
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(20.0),
                        padding: UiRect::all(Val::Px(30.0)),
                        margin: UiRect::bottom(Val::Px(50.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                    BorderRadius::all(Val::Px(12.0)),
                    Visibility::Hidden,
                ))
                .with_children(|parent| {
                    // Selected mode
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Mode:"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                            ));
                            parent.spawn((
                                SelectedModeText,
                                Text::new("---"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Selected map
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Map:"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                            ));
                            parent.spawn((
                                SelectedMapText,
                                Text::new("---"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Start Game button
                    parent
                        .spawn((
                            StartGameButton,
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.7, 0.3)),
                            BorderRadius::all(Val::Px(8.0)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("▶ Start Game"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });

            // Hint when no map selected
            parent.spawn((
                Text::new("Press PLAY to select a game mode and map"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgba(0.5, 0.5, 0.5, 0.8)),
                Node {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                },
            ));
        });
}

fn toggle_home_tab_visibility(
    game_state: Res<State<GameState>>,
    menu_tab: Res<State<MenuTab>>,
    mut home_query: Query<&mut Visibility, With<HomeTabRoot>>,
) {
    let Ok(mut visibility) = home_query.single_mut() else {
        return;
    };

    let should_show =
        *game_state.get() == GameState::MainMenu && *menu_tab.get() == MenuTab::Home;

    *visibility = if should_show {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

fn update_selected_info(
    game_state: Res<State<GameState>>,
    menu_tab: Res<State<MenuTab>>,
    selected_map: Res<SelectedMap>,
    config: Res<GameConfig>,
    mut map_text_query: Query<&mut Text, (With<SelectedMapText>, Without<SelectedModeText>)>,
    mut mode_text_query: Query<&mut Text, (With<SelectedModeText>, Without<SelectedMapText>)>,
    mut container_query: Query<&mut Visibility, With<StartGameContainer>>,
) {
    let Ok(mut container_vis) = container_query.single_mut() else {
        return;
    };

    // Only show container in MainMenu Home tab with a map selected
    let in_home_menu = *game_state.get() == GameState::MainMenu && *menu_tab.get() == MenuTab::Home;
    
    if in_home_menu && selected_map.map.is_some() {
        // Show the container
        *container_vis = Visibility::Visible;

        // Update map text
        if let Ok(mut text) = map_text_query.single_mut() {
            if let Some(map) = &selected_map.map {
                **text = map.name().to_string();
            }
        }

        // Update mode text
        if let Ok(mut text) = mode_text_query.single_mut() {
            **text = config.mode.name().to_string();
        }
    } else {
        *container_vis = Visibility::Hidden;
    }
}

fn handle_start_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    selected_map: Res<SelectedMap>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut config: ResMut<GameConfig>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(map) = &selected_map.map {
                // Set the map in config
                config.map = map.clone();
                // Start the game
                next_game_state.set(GameState::Playing);
            }
        }
    }
}

