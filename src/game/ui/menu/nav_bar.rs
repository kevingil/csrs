use bevy::prelude::*;

use super::MenuTab;
use crate::game::GameState;

pub struct NavBarPlugin;

impl Plugin for NavBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_nav_bar)
            .add_systems(
                Update,
                (
                    handle_nav_buttons,
                    update_button_styles,
                    toggle_nav_visibility,
                ),
            );
    }
}

#[derive(Component)]
pub struct NavBarRoot;

#[derive(Component)]
struct HomeButton;

#[derive(Component)]
struct InventoryButton;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct SettingsButton;

// Colors
const NAV_BG: Color = Color::srgba(0.05, 0.05, 0.08, 0.95);
const BUTTON_NORMAL: Color = Color::srgba(0.15, 0.15, 0.2, 1.0);
const BUTTON_HOVER: Color = Color::srgba(0.25, 0.25, 0.35, 1.0);
const BUTTON_ACTIVE: Color = Color::srgba(0.2, 0.5, 0.3, 1.0);
const PLAY_BUTTON_NORMAL: Color = Color::srgb(0.2, 0.6, 0.3);
const PLAY_BUTTON_HOVER: Color = Color::srgb(0.25, 0.7, 0.35);
const PLAY_BUTTON_ACTIVE: Color = Color::srgb(0.15, 0.5, 0.25);

fn setup_nav_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Navigation bar container
    commands
        .spawn((
            NavBarRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(70.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                padding: UiRect::horizontal(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(NAV_BG),
            GlobalZIndex(200),
        ))
        .with_children(|parent| {
            // Home button (left side) with icon
            parent
                .spawn((
                    HomeButton,
                    Button,
                    Node {
                        width: Val::Px(50.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(BUTTON_NORMAL),
                    BorderRadius::all(Val::Px(8.0)),
                ))
                .with_child((
                    ImageNode {
                        image: asset_server.load("models/images/home.png"),
                        color: Color::WHITE, // Tints black image to white
                        ..default()
                    },
                    Node {
                        width: Val::Px(28.0),
                        height: Val::Px(28.0),
                        ..default()
                    },
                ));

            // Spacer to push play to center
            parent.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });

            // Inventory button
            parent
                .spawn((
                    InventoryButton,
                    Button,
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(45.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(BUTTON_NORMAL),
                    BorderRadius::all(Val::Px(8.0)),
                ))
                .with_child((
                    Text::new("Inventory"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

            // Play button (center, large, green)
            parent
                .spawn((
                    PlayButton,
                    Button,
                    Node {
                        width: Val::Px(160.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(PLAY_BUTTON_NORMAL),
                    BorderRadius::all(Val::Px(8.0)),
                ))
                .with_child((
                    Text::new("PLAY"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

            // Settings button
            parent
                .spawn((
                    SettingsButton,
                    Button,
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(45.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(BUTTON_NORMAL),
                    BorderRadius::all(Val::Px(8.0)),
                ))
                .with_child((
                    Text::new("Settings"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

            // Spacer to balance
            parent.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });
        });
}

fn handle_nav_buttons(
    home_query: Query<&Interaction, (Changed<Interaction>, With<HomeButton>)>,
    inventory_query: Query<&Interaction, (Changed<Interaction>, With<InventoryButton>)>,
    play_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    settings_query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    mut next_tab: ResMut<NextState<MenuTab>>,
) {
    for interaction in &home_query {
        if *interaction == Interaction::Pressed {
            next_tab.set(MenuTab::Home);
        }
    }
    for interaction in &inventory_query {
        if *interaction == Interaction::Pressed {
            next_tab.set(MenuTab::Inventory);
        }
    }
    for interaction in &play_query {
        if *interaction == Interaction::Pressed {
            next_tab.set(MenuTab::Play);
        }
    }
    for interaction in &settings_query {
        if *interaction == Interaction::Pressed {
            next_tab.set(MenuTab::Settings);
        }
    }
}

fn update_button_styles(
    current_tab: Res<State<MenuTab>>,
    mut home_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<HomeButton>, Without<PlayButton>),
    >,
    mut inventory_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<InventoryButton>, Without<PlayButton>, Without<HomeButton>),
    >,
    mut play_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<PlayButton>, Without<HomeButton>, Without<InventoryButton>),
    >,
    mut settings_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            With<SettingsButton>,
            Without<PlayButton>,
            Without<HomeButton>,
            Without<InventoryButton>,
        ),
    >,
) {
    // Update Home button
    for (interaction, mut bg) in &mut home_query {
        let is_active = *current_tab.get() == MenuTab::Home;
        *bg = match (*interaction, is_active) {
            (_, true) => BackgroundColor(BUTTON_ACTIVE),
            (Interaction::Hovered, false) => BackgroundColor(BUTTON_HOVER),
            _ => BackgroundColor(BUTTON_NORMAL),
        };
    }

    // Update Inventory button
    for (interaction, mut bg) in &mut inventory_query {
        let is_active = *current_tab.get() == MenuTab::Inventory;
        *bg = match (*interaction, is_active) {
            (_, true) => BackgroundColor(BUTTON_ACTIVE),
            (Interaction::Hovered, false) => BackgroundColor(BUTTON_HOVER),
            _ => BackgroundColor(BUTTON_NORMAL),
        };
    }

    // Update Play button
    for (interaction, mut bg) in &mut play_query {
        let is_active = *current_tab.get() == MenuTab::Play;
        *bg = match (*interaction, is_active) {
            (_, true) => BackgroundColor(PLAY_BUTTON_ACTIVE),
            (Interaction::Hovered, false) => BackgroundColor(PLAY_BUTTON_HOVER),
            _ => BackgroundColor(PLAY_BUTTON_NORMAL),
        };
    }

    // Update Settings button
    for (interaction, mut bg) in &mut settings_query {
        let is_active = *current_tab.get() == MenuTab::Settings;
        *bg = match (*interaction, is_active) {
            (_, true) => BackgroundColor(BUTTON_ACTIVE),
            (Interaction::Hovered, false) => BackgroundColor(BUTTON_HOVER),
            _ => BackgroundColor(BUTTON_NORMAL),
        };
    }
}

fn toggle_nav_visibility(
    game_state: Res<State<GameState>>,
    mut nav_query: Query<&mut Visibility, With<NavBarRoot>>,
) {
    let Ok(mut visibility) = nav_query.single_mut() else {
        return;
    };

    *visibility = match game_state.get() {
        GameState::MainMenu => Visibility::Visible,
        _ => Visibility::Hidden,
    };
}
