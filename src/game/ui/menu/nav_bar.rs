use super::{style::*, MenuTab};
use crate::game::GameState;
use bevy::prelude::*;
pub struct NavBarPlugin;
#[derive(Component)]
pub struct NavBarRoot;
#[derive(Component)]
pub(super) struct NavButton(pub MenuTab);
impl Plugin for NavBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, visibility)
            .add_systems(Update, interact.run_if(in_state(GameState::MainMenu)));
    }
}
fn setup(mut commands: Commands) {
    commands
        .spawn((
            NavBarRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Px(HEADER),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(INK),
            GlobalZIndex(220),
        ))
        .with_children(|bar| {
            for (tab, name) in [
                (MenuTab::Home, "HOME"),
                (MenuTab::Inventory, "INVENTORY"),
                (MenuTab::Play, "PLAY"),
                (MenuTab::LoadOut, "LOAD OUT"),
                (MenuTab::Settings, "SETTINGS"),
            ] {
                let size = if tab == MenuTab::Play { 26. } else { 17. };
                bar.spawn((
                    NavButton(tab),
                    Button,
                    Node {
                        width: Val::Vw(11.),
                        min_width: Val::Px(92.),
                        max_width: Val::Px(164.),
                        height: Val::Px(HEADER),
                        border: UiRect::bottom(Val::Px(2.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderColor(Color::NONE),
                ))
                .with_child(label(name, size, WHITE));
            }
        });
}
fn interact(
    tab: Res<State<MenuTab>>,
    mut next: ResMut<NextState<MenuTab>>,
    mut buttons: Query<(
        &NavButton,
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
) {
    for (button, interaction, mut bg, mut border) in &mut buttons {
        if *interaction == Interaction::Pressed && button.0 != *tab.get() {
            next.set(button.0.clone());
        }
        let active = button.0 == *tab.get();
        bg.0 = if active {
            SELECTED
        } else if *interaction == Interaction::Hovered {
            Color::srgba(1., 1., 1., 0.08)
        } else {
            Color::NONE
        };
        border.0 = if active { ACCENT } else { Color::NONE };
    }
}
fn visibility(state: Res<State<GameState>>, mut roots: Query<&mut Node, With<NavBarRoot>>) {
    for mut node in &mut roots {
        node.display = if *state.get() == GameState::MainMenu {
            Display::Flex
        } else {
            Display::None
        };
    }
}
