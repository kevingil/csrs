use super::{style::*, MenuPage, MenuTab, PlayerLoadout};
use crate::game::player::skins::{SkinId, SkinRegistry};
use bevy::prelude::*;
pub struct InventoryTabPlugin;
#[derive(Component)]
struct Equipped(SkinId);
impl Plugin for InventoryTabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, equipped);
    }
}
fn setup(mut commands: Commands, server: Res<AssetServer>, skins: Res<SkinRegistry>) {
    commands
        .spawn((
            MenuPage(MenuTab::Inventory),
            page(),
            BackgroundColor(PANEL),
            GlobalZIndex(150),
        ))
        .with_children(|root| {
            root.spawn(label("INVENTORY", 30., WHITE));
            root.spawn(label("Your available equipment", 16., MUTED));
            root.spawn((
                Node {
                    width: Val::Px(280.),
                    padding: UiRect::all(Val::Px(24.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(18.),
                    ..default()
                },
                BackgroundColor(INK),
            ))
            .with_children(|card| {
                card.spawn((
                    ImageNode::new(server.load("generated/ui/ak47.png")),
                    Node {
                        width: Val::Px(230.),
                        aspect_ratio: Some(3.),
                        ..default()
                    },
                ));
                card.spawn(label("AK-47 · Equipped", 18., WHITE));
            });
            root.spawn((
                ImageNode::new(server.load("generated/ui/knife.png")),
                Node {
                    width: Val::Px(150.0),
                    aspect_ratio: Some(3.0),
                    ..default()
                },
            ));
            root.spawn(label("Default Knife · Equipped", 18.0, WHITE));
            root.spawn(label("CHARACTERS", 17., MUTED));
            root.spawn(Node {
                column_gap: Val::Px(20.),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            })
            .with_children(|row| {
                for skin in &skins.skins {
                    row.spawn((
                        Node {
                            min_width: Val::Px(220.),
                            padding: UiRect::all(Val::Px(24.)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(14.),
                            ..default()
                        },
                        BackgroundColor(INK),
                    ))
                    .with_children(|card| {
                        card.spawn(label(skin.name, 21., WHITE));
                        card.spawn(label(skin.side.name(), 14., skin.side.color()));
                        card.spawn((Equipped(skin.id), label("", 13., ACCENT)));
                    });
                }
            });
            root.spawn(label("Open Load Out to equip a character.", 14., MUTED));
        });
}
fn equipped(loadout: Res<PlayerLoadout>, mut labels: Query<(&Equipped, &mut Text)>) {
    if !loadout.is_changed() {
        return;
    }
    for (skin, mut text) in &mut labels {
        **text = if skin.0 == loadout.selected_skin {
            "Equipped"
        } else {
            "Available"
        }
        .into();
    }
}
