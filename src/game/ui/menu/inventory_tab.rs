use bevy::prelude::*;

use super::{MenuTab, PlayerLoadout, WeaponId};
use crate::game::player::skins::{SkinId, SkinRegistry};
use crate::game::GameState;

pub struct InventoryTabPlugin;

impl Plugin for InventoryTabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_inventory_tab)
            .add_systems(
                Update,
                (
                    toggle_inventory_tab_visibility,
                    handle_weapon_buttons,
                    update_weapon_button_styles,
                    handle_skin_buttons,
                    update_skin_button_styles,
                ),
            );
    }
}

#[derive(Component)]
struct InventoryTabRoot;

#[derive(Component)]
struct WeaponButton(WeaponId);

#[derive(Component)]
struct SkinButton(SkinId);

// Colors
const OVERLAY_BG: Color = Color::srgba(0.02, 0.02, 0.05, 0.92);
const BUTTON_NORMAL: Color = Color::srgba(0.12, 0.12, 0.18, 1.0);
const BUTTON_HOVER: Color = Color::srgba(0.2, 0.2, 0.3, 1.0);
const BUTTON_SELECTED: Color = Color::srgb(0.2, 0.5, 0.3);

fn setup_inventory_tab(
    mut commands: Commands,
    loadout: Res<PlayerLoadout>,
    skin_registry: Res<SkinRegistry>,
) {
    // Inventory tab overlay
    commands
        .spawn((
            InventoryTabRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                top: Val::Px(70.0),
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(40.0)),
                row_gap: Val::Px(30.0),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(OVERLAY_BG),
            GlobalZIndex(150),
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("LOADOUT"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Section: Primary Weapon
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    ..default()
                })
                .with_children(|section| {
                    section.spawn((
                        Text::new("Primary Weapon"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));

                    // Weapon list
                    section
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(15.0),
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        })
                        .with_children(|weapons| {
                            for weapon in WeaponId::all() {
                                let is_selected = weapon == loadout.primary_weapon;
                                let bg_color = if is_selected {
                                    BUTTON_SELECTED
                                } else {
                                    BUTTON_NORMAL
                                };

                                weapons
                                    .spawn((
                                        WeaponButton(weapon.clone()),
                                        Button,
                                        Node {
                                            width: Val::Px(150.0),
                                            height: Val::Px(120.0),
                                            flex_direction: FlexDirection::Column,
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            row_gap: Val::Px(10.0),
                                            ..default()
                                        },
                                        BackgroundColor(bg_color),
                                        BorderRadius::all(Val::Px(8.0)),
                                    ))
                                    .with_children(|card| {
                                        // Weapon icon placeholder
                                        card.spawn((
                                            Node {
                                                width: Val::Px(80.0),
                                                height: Val::Px(50.0),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
                                            BorderRadius::all(Val::Px(4.0)),
                                        ));

                                        // Weapon name
                                        card.spawn((
                                            Text::new(weapon.name()),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });
                            }
                        });
                });

            // Section: Character Skin
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    ..default()
                })
                .with_children(|section| {
                    section.spawn((
                        Text::new("Character Skin"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));

                    // Skin list
                    section
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(15.0),
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        })
                        .with_children(|skins| {
                            for skin_def in &skin_registry.skins {
                                let is_selected = skin_def.id == loadout.selected_skin;
                                let bg_color = if is_selected {
                                    BUTTON_SELECTED
                                } else {
                                    BUTTON_NORMAL
                                };

                                skins
                                    .spawn((
                                        SkinButton(skin_def.id),
                                        Button,
                                        Node {
                                            width: Val::Px(150.0),
                                            height: Val::Px(140.0),
                                            flex_direction: FlexDirection::Column,
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            row_gap: Val::Px(8.0),
                                            ..default()
                                        },
                                        BackgroundColor(bg_color),
                                        BorderRadius::all(Val::Px(8.0)),
                                    ))
                                    .with_children(|card| {
                                        // Skin icon placeholder
                                        card.spawn((
                                            Node {
                                                width: Val::Px(60.0),
                                                height: Val::Px(60.0),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
                                            BorderRadius::all(Val::Px(4.0)),
                                        ));

                                        // Skin name
                                        card.spawn((
                                            Text::new(skin_def.name),
                                            TextFont {
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));

                                        // Side indicator
                                        card.spawn((
                                            Text::new(skin_def.side.name()),
                                            TextFont {
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(skin_def.side.color()),
                                        ));
                                    });
                            }
                        });
                });

            // Spacer
            parent.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });

            // Hint
            parent.spawn((
                Text::new("Select your weapon and character skin"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

fn toggle_inventory_tab_visibility(
    game_state: Res<State<GameState>>,
    menu_tab: Res<State<MenuTab>>,
    mut inventory_query: Query<&mut Visibility, With<InventoryTabRoot>>,
) {
    let Ok(mut visibility) = inventory_query.single_mut() else {
        return;
    };

    let should_show =
        *game_state.get() == GameState::MainMenu && *menu_tab.get() == MenuTab::Inventory;

    *visibility = if should_show {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

fn handle_weapon_buttons(
    interaction_query: Query<(&Interaction, &WeaponButton), Changed<Interaction>>,
    mut loadout: ResMut<PlayerLoadout>,
) {
    for (interaction, weapon_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            loadout.primary_weapon = weapon_button.0.clone();
        }
    }
}

fn update_weapon_button_styles(
    loadout: Res<PlayerLoadout>,
    mut query: Query<(&WeaponButton, &Interaction, &mut BackgroundColor)>,
) {
    for (weapon_button, interaction, mut bg) in &mut query {
        let is_selected = weapon_button.0 == loadout.primary_weapon;
        *bg = match (*interaction, is_selected) {
            (_, true) => BackgroundColor(BUTTON_SELECTED),
            (Interaction::Hovered, false) => BackgroundColor(BUTTON_HOVER),
            _ => BackgroundColor(BUTTON_NORMAL),
        };
    }
}

fn handle_skin_buttons(
    interaction_query: Query<(&Interaction, &SkinButton), Changed<Interaction>>,
    mut loadout: ResMut<PlayerLoadout>,
) {
    for (interaction, skin_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            loadout.selected_skin = skin_button.0;
        }
    }
}

fn update_skin_button_styles(
    loadout: Res<PlayerLoadout>,
    mut query: Query<(&SkinButton, &Interaction, &mut BackgroundColor)>,
) {
    for (skin_button, interaction, mut bg) in &mut query {
        let is_selected = skin_button.0 == loadout.selected_skin;
        *bg = match (*interaction, is_selected) {
            (_, true) => BackgroundColor(BUTTON_SELECTED),
            (Interaction::Hovered, false) => BackgroundColor(BUTTON_HOVER),
            _ => BackgroundColor(BUTTON_NORMAL),
        };
    }
}
