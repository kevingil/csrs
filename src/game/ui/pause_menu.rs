use bevy::{app::AppExit, prelude::*};

use crate::game::GameState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_pause_menu)
            .add_systems(Update, (
                toggle_pause_menu_visibility,
                handle_resume_button,
                handle_quit_to_menu_button,
                handle_exit_button,
            ));
    }
}

#[derive(Component)]
struct PauseMenuRoot;

#[derive(Component)]
struct ResumeButton;

#[derive(Component)]
struct QuitToMenuButton;

#[derive(Component)]
struct ExitButton;

fn setup_pause_menu(mut commands: Commands) {
    // Root container - full screen overlay
    commands
        .spawn((
            PauseMenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            Visibility::Hidden,
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Resume button
            parent
                .spawn((
                    ResumeButton,
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Resume"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Quit to Menu button
            parent
                .spawn((
                    QuitToMenuButton,
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.4, 0.2)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Main Menu"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Exit button
            parent
                .spawn((
                    ExitButton,
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.2, 0.2)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Exit Game"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn toggle_pause_menu_visibility(
    game_state: Res<State<GameState>>,
    mut menu_query: Query<&mut Visibility, With<PauseMenuRoot>>,
) {
    let Ok(mut visibility) = menu_query.single_mut() else {
        return;
    };
    
    *visibility = match game_state.get() {
        GameState::Paused => Visibility::Visible,
        _ => Visibility::Hidden,
    };
}

fn handle_resume_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

fn handle_quit_to_menu_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitToMenuButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }
}

fn handle_exit_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut exit_events: EventWriter<AppExit>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            exit_events.write(AppExit::Success);
        }
    }
}

