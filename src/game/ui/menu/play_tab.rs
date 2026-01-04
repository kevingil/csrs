use bevy::prelude::*;

use super::{MenuTab, SelectedMap};
use crate::game::{
    config::{GameConfig, GameMode, MapId},
    GameState,
};

pub struct PlayTabPlugin;

impl Plugin for PlayTabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_play_tab)
            .add_systems(
                Update,
                (
                    toggle_play_tab_visibility,
                    handle_gamemode_buttons,
                    handle_map_buttons,
                    update_gamemode_button_styles,
                ),
            );
    }
}

#[derive(Component)]
struct PlayTabRoot;

#[derive(Component)]
struct GameModeButton(GameMode);

#[derive(Component)]
struct MapButton(MapId);

// Colors
const OVERLAY_BG: Color = Color::srgba(0.02, 0.02, 0.05, 0.92);
const MODE_BUTTON_NORMAL: Color = Color::srgba(0.15, 0.15, 0.2, 1.0);
const MODE_BUTTON_ACTIVE: Color = Color::srgb(0.2, 0.5, 0.3);
const MAP_CARD_BG: Color = Color::srgba(0.12, 0.12, 0.18, 1.0);
const MAP_CARD_HOVER: Color = Color::srgba(0.2, 0.2, 0.3, 1.0);

/// Get all available maps for a game mode
fn get_maps_for_mode(mode: &GameMode) -> Vec<(MapId, &'static str, Color)> {
    match mode {
        GameMode::Freemode => vec![
            (MapId::Default, "Default Arena", Color::srgb(0.3, 0.5, 0.7)),
        ],
    }
}

/// Get all game modes
fn get_all_modes() -> Vec<GameMode> {
    vec![GameMode::Freemode]
}

fn setup_play_tab(mut commands: Commands, config: Res<GameConfig>) {
    // Play tab overlay
    commands
        .spawn((
            PlayTabRoot,
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
                Text::new("SELECT GAME MODE & MAP"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Game mode tabs
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(15.0),
                    ..default()
                })
                .with_children(|mode_row| {
                    for mode in get_all_modes() {
                        let is_active = mode == config.mode;
                        let bg_color = if is_active {
                            MODE_BUTTON_ACTIVE
                        } else {
                            MODE_BUTTON_NORMAL
                        };

                        mode_row
                            .spawn((
                                GameModeButton(mode.clone()),
                                Button,
                                Node {
                                    padding: UiRect::axes(Val::Px(25.0), Val::Px(12.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(bg_color),
                                BorderRadius::all(Val::Px(6.0)),
                            ))
                            .with_child((
                                Text::new(mode.name()),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                    }
                });

            // Map grid container
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: Val::Px(20.0),
                    row_gap: Val::Px(20.0),
                    padding: UiRect::top(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|grid| {
                    for (map_id, map_name, preview_color) in get_maps_for_mode(&config.mode) {
                        grid.spawn((
                            MapButton(map_id),
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(160.0),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            BackgroundColor(MAP_CARD_BG),
                            BorderRadius::all(Val::Px(8.0)),
                        ))
                        .with_children(|card| {
                            // Preview image placeholder
                            card.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(100.0),
                                    ..default()
                                },
                                BackgroundColor(preview_color),
                                BorderRadius::top(Val::Px(8.0)),
                            ));

                            // Map name
                            card.spawn((
                                Text::new(map_name),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Node {
                                    padding: UiRect::all(Val::Px(12.0)),
                                    ..default()
                                },
                            ));
                        });
                    }
                });

            // Hint
            parent.spawn((
                Text::new("Click a map to select and return to home"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

fn toggle_play_tab_visibility(
    game_state: Res<State<GameState>>,
    menu_tab: Res<State<MenuTab>>,
    mut play_query: Query<&mut Visibility, With<PlayTabRoot>>,
) {
    let Ok(mut visibility) = play_query.single_mut() else {
        return;
    };

    let should_show =
        *game_state.get() == GameState::MainMenu && *menu_tab.get() == MenuTab::Play;

    *visibility = if should_show {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

fn handle_gamemode_buttons(
    interaction_query: Query<(&Interaction, &GameModeButton), Changed<Interaction>>,
    mut config: ResMut<GameConfig>,
) {
    for (interaction, mode_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            config.mode = mode_button.0.clone();
        }
    }
}

fn handle_map_buttons(
    interaction_query: Query<(&Interaction, &MapButton), Changed<Interaction>>,
    mut selected_map: ResMut<SelectedMap>,
    mut next_tab: ResMut<NextState<MenuTab>>,
) {
    for (interaction, map_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            selected_map.map = Some(map_button.0.clone());
            selected_map.ready_to_start = true;
            next_tab.set(MenuTab::Home);
        }
    }
}

fn update_gamemode_button_styles(
    config: Res<GameConfig>,
    mut query: Query<(&GameModeButton, &Interaction, &mut BackgroundColor)>,
) {
    for (mode_button, interaction, mut bg) in &mut query {
        let is_active = mode_button.0 == config.mode;
        *bg = match (*interaction, is_active) {
            (_, true) => BackgroundColor(MODE_BUTTON_ACTIVE),
            (Interaction::Hovered, false) => BackgroundColor(MAP_CARD_HOVER),
            _ => BackgroundColor(MODE_BUTTON_NORMAL),
        };
    }
}
