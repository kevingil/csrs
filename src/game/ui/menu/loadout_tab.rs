use super::{style::*, MenuPage, MenuTab, PlayerLoadout};
use crate::game::{
    player::skins::{SkinId, SkinRegistry},
    GameState,
};
use bevy::prelude::*;
pub struct LoadoutTabPlugin;
#[derive(Component)]
pub(super) struct SkinButton(pub SkinId);
#[derive(Component)]
struct EquippedLabel(SkinId);
impl Plugin for LoadoutTabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            interact.run_if(in_state(GameState::MainMenu).and(in_state(MenuTab::LoadOut))),
        );
    }
}
fn setup(mut commands: Commands, skins: Res<SkinRegistry>, server: Res<AssetServer>) {
    commands
        .spawn((
            MenuPage(MenuTab::LoadOut),
            Node {
                right: Val::Auto,
                width: Val::Percent(38.),
                min_width: Val::Px(340.),
                ..page()
            },
            BackgroundColor(PANEL),
            GlobalZIndex(150),
        ))
        .with_children(|root| {
            root.spawn(label("LOAD OUT", 30., WHITE));
            root.spawn(label("PRIMARY WEAPON", 15., MUTED));
            root.spawn((
                ImageNode::new(server.load("generated/ui/ak47.png")),
                Node {
                    width: Val::Px(230.),
                    aspect_ratio: Some(3.),
                    ..default()
                },
            ));
            root.spawn(label("AK-47 · Equipped", 18., WHITE));
            root.spawn((
                ImageNode::new(server.load("generated/ui/knife.png")),
                Node {
                    width: Val::Px(150.0),
                    aspect_ratio: Some(3.0),
                    ..default()
                },
            ));
            root.spawn(label("Default Knife · Equipped", 18.0, WHITE));
            root.spawn(label("CHARACTER", 15., MUTED));
            for skin in &skins.skins {
                root.spawn((
                    SkinButton(skin.id),
                    Button,
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Start,
                        row_gap: Val::Px(8.),
                        ..button()
                    },
                    BackgroundColor(INK),
                    BorderColor(Color::NONE),
                ))
                .with_children(|card| {
                    card.spawn(label(skin.name, 20., WHITE));
                    card.spawn(label(skin.side.name(), 13., skin.side.color()));
                    card.spawn((EquippedLabel(skin.id), label("", 13., ACCENT)));
                });
            }
        });
}
fn interact(
    mut loadout: ResMut<PlayerLoadout>,
    mut cards: Query<(
        &SkinButton,
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
    mut labels: Query<(&EquippedLabel, &mut Text)>,
) {
    for (skin, interaction, _, _) in &mut cards {
        if *interaction == Interaction::Pressed && skin.0 != loadout.selected_skin {
            loadout.selected_skin = skin.0;
        }
    }
    for (skin, interaction, mut bg, mut border) in &mut cards {
        let selected = skin.0 == loadout.selected_skin;
        bg.0 = if selected || *interaction == Interaction::Hovered {
            SELECTED
        } else {
            INK
        };
        border.0 = if selected { ACCENT } else { Color::NONE };
    }
    for (skin, mut text) in &mut labels {
        **text = if skin.0 == loadout.selected_skin {
            "Equipped"
        } else {
            "Click to equip"
        }
        .into();
    }
}
