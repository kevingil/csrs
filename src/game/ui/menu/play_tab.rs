use super::{scene_definition::menu_scene, style::*, LocalMatchOption, MenuPage, MenuTab};
use crate::game::{
    config::{GameConfig, MapId},
    GameState,
};
use bevy::prelude::*;
pub struct PlayTabPlugin;
#[derive(Component)]
pub(super) struct StartGameButton;
#[derive(Component)]
struct MapCard;
#[derive(Component)]
struct MatchDetails;
#[derive(Resource, Default)]
struct StartPending(bool);
impl Plugin for PlayTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StartPending>()
            .add_systems(Startup, setup)
            .add_systems(
                OnEnter(GameState::MainMenu),
                |mut pending: ResMut<StartPending>| pending.0 = false,
            )
            .add_systems(
                Update,
                (interact, details)
                    .run_if(in_state(GameState::MainMenu).and(in_state(MenuTab::Play))),
            );
    }
}
fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands
        .spawn((
            MenuPage(MenuTab::Play),
            page(),
            BackgroundColor(Color::srgba(0.04, 0.05, 0.06, 0.76)),
            GlobalZIndex(150),
        ))
        .with_children(|root| {
            root.spawn(label("PLAY", 30., WHITE));
            root.spawn((
                Node {
                    align_self: AlignSelf::Start,
                    ..button()
                },
                BackgroundColor(SELECTED),
                BorderColor(ACCENT),
            ))
            .with_child(label(LocalMatchOption::LABEL, 19., WHITE));
            root.spawn(Node {
                flex_grow: 1.,
                column_gap: Val::Vw(4.),
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    MapCard,
                    Button,
                    Node {
                        width: Val::VMin(30.),
                        height: Val::VMin(49.),
                        min_width: Val::Px(180.),
                        border: UiRect::all(Val::Px(2.)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    BackgroundColor(PANEL),
                    BorderColor(ACCENT),
                ))
                .with_children(|card| {
                    card.spawn((
                        ImageNode::new(server.load(menu_scene(&MapId::Dust2).thumbnail)),
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                    ));
                    card.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(10.),
                            right: Val::Px(10.),
                            padding: UiRect::all(Val::Px(8.)),
                            ..default()
                        },
                        BackgroundColor(SELECTED),
                    ))
                    .with_child(label("SELECTED", 11., WHITE));
                    card.spawn((
                        Node {
                            padding: UiRect::all(Val::Px(18.)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.),
                            ..default()
                        },
                        BackgroundColor(INK),
                    ))
                    .with_children(|name| {
                        name.spawn(label("DUST 2", 28., WHITE));
                        name.spawn(label("DEATHMATCH", 12., ACCENT));
                    });
                });
                row.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(18.),
                    max_width: Val::Vw(40.),
                    ..default()
                })
                .with_children(|info| {
                    info.spawn(label("Deathmatch", 32., WHITE));
                    info.spawn((MatchDetails, label("", 17., WHITE)));
                    info.spawn(label(
                        "Eliminate the opposing team.\nRespawn and rejoin the fight.",
                        16.,
                        MUTED,
                    ));
                });
            });
            root.spawn((
                StartGameButton,
                Button,
                Node {
                    align_self: AlignSelf::End,
                    ..button()
                },
                BackgroundColor(SELECTED),
                BorderColor(ACCENT),
            ))
            .with_child(label("START GAME", 21., WHITE));
        });
}
fn details(config: Res<GameConfig>, mut labels: Query<&mut Text, With<MatchDetails>>) {
    if !config.is_changed() {
        return;
    }
    let time = config
        .match_settings
        .time_limit
        .map(|v| format!("{} min", v.as_secs() / 60))
        .unwrap_or("Unlimited time".into());
    let score = config
        .match_settings
        .score_limit
        .map(|v| format!("First to {v} kills"))
        .unwrap_or("No score limit".into());
    for mut text in &mut labels {
        **text = format!("Local bots · 3v3\n{time} · {score}");
    }
}
fn interact(
    mut config: ResMut<GameConfig>,
    mut pending: ResMut<StartPending>,
    mut next: ResMut<NextState<GameState>>,
    cards: Query<&Interaction, (Changed<Interaction>, With<MapCard>)>,
    buttons: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
) {
    if cards.iter().any(|i| *i == Interaction::Pressed) {
        LocalMatchOption::select(&mut config);
    }
    if !pending.0
        && LocalMatchOption::selected(&config)
        && buttons.iter().any(|i| *i == Interaction::Pressed)
    {
        pending.0 = true;
        next.set(GameState::Loading);
    }
}
