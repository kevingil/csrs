//! Opt-in native scenario: exercises menu handlers without OS input injection.
use super::{
    friends_drawer::DrawerState, home_scene::HomeSceneEntity, loadout_tab::SkinButton,
    nav_bar::NavButton, play_tab::StartGameButton, MenuTab,
};
use crate::game::{matchplay::Combatant, player::skins::SkinId, GameState};
use bevy::{
    prelude::*,
    render::view::screenshot::{save_to_disk, Screenshot},
};
#[derive(Resource, Default)]
struct Scenario {
    step: usize,
    at: f32,
}
pub fn install(app: &mut App) {
    if std::env::var_os("CSRS_MENU_SCENARIO").is_some() {
        app.init_resource::<Scenario>()
            .add_systems(PreUpdate, run.after(bevy::ui::UiSystem::Focus));
    }
}
fn run(
    mut commands: Commands,
    time: Res<Time<Real>>,
    game: Res<State<GameState>>,
    mut scenario: ResMut<Scenario>,
    mut nav: Query<(&NavButton, &mut Interaction)>,
    mut skins: Query<
        (&SkinButton, &mut Interaction),
        (Without<NavButton>, Without<StartGameButton>),
    >,
    mut starts: Query<&mut Interaction, (With<StartGameButton>, Without<NavButton>)>,
    mut drawer: ResMut<DrawerState>,
    mut next: ResMut<NextState<GameState>>,
    actors: Query<&Combatant>,
    menu_entities: Query<(), With<HomeSceneEntity>>,
    mut exit: EventWriter<AppExit>,
    server: Res<AssetServer>,
    scenes: Query<&SceneRoot>,
    menu_cameras: Query<(), With<super::home_scene::HomeSceneCamera>>,
) {
    let elapsed = time.elapsed_secs();
    let delay = if scenario.step == 0 { 7. } else { 1.2 };
    if elapsed - scenario.at < delay {
        return;
    }
    let capture = |commands: &mut Commands, name: &str| {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("/private/tmp/csrs-menu-{name}.png")));
    };
    let mut tab = |target| {
        for (button, mut interaction) in &mut nav {
            *interaction = if button.0 == target {
                Interaction::Pressed
            } else {
                Interaction::None
            };
        }
    };
    match scenario.step {
        0 => {
            assert_eq!(menu_cameras.iter().count(), 1);
            for scene in &scenes {
                if let Some(path) = server.get_path(scene.0.id()) {
                    let path = path.to_string();
                    assert!(
                        !path.contains("generated/dust2.glb")
                            && !path.contains("armory_map")
                            && !path.contains("warehouse_map"),
                        "Gameplay background loaded in menu"
                    );
                    info!("MENU_SCENE_ASSET {path}");
                }
            }
            info!("MENU_SCENARIO Home ready; pid {}", std::process::id());
            capture(&mut commands, "home");
        }
        1 => drawer.focused = true,
        2 => capture(&mut commands, "friends"),
        3 => {
            drawer.focused = false;
            tab(MenuTab::Inventory);
        }
        4 => capture(&mut commands, "inventory"),
        5 => tab(MenuTab::LoadOut),
        6 => {
            for (skin, mut interaction) in &mut skins {
                if skin.0 == SkinId::Police {
                    *interaction = Interaction::Pressed;
                }
            }
        }
        7 => capture(&mut commands, "loadout"),
        8 => {
            for (_, mut interaction) in &mut skins {
                *interaction = Interaction::None;
            }
            tab(MenuTab::Home);
        }
        9 => capture(&mut commands, "defender"),
        10 => tab(MenuTab::Settings),
        11 => capture(&mut commands, "settings"),
        12 => tab(MenuTab::Play),
        13 => capture(&mut commands, "play"),
        14 => {
            for mut interaction in &mut starts {
                *interaction = Interaction::Pressed;
            }
        }
        15 => {
            if *game.get() == GameState::Loading {
                return;
            }
            assert_eq!(
                *game.get(),
                GameState::Playing,
                "Play handler did not launch match"
            );
            assert_eq!(actors.iter().count(), 6, "Expected local 3v3");
            assert!(menu_entities.is_empty(), "Menu entities leaked into match");
            info!("MENU_SCENARIO match loaded with 6 actors and no menu entities");
            capture(&mut commands, "hud");
        }
        16 => next.set(GameState::Paused),
        17 => {
            assert_eq!(*game.get(), GameState::Paused);
            next.set(GameState::Playing);
        }
        18 => next.set(GameState::MainMenu),
        19 => {
            assert!(actors.is_empty(), "Match actors leaked into Home");
            assert!(!menu_entities.is_empty());
            capture(&mut commands, "return");
            tab(MenuTab::Play);
            for mut interaction in &mut starts {
                *interaction = Interaction::None;
            }
        }
        20 => {
            for mut interaction in &mut starts {
                *interaction = Interaction::Pressed;
            }
        }
        21 => {
            if *game.get() == GameState::Loading {
                return;
            }
            assert_eq!(*game.get(), GameState::Playing);
            assert_eq!(actors.iter().count(), 6);
            assert!(menu_entities.is_empty());
            info!("MENU_SCENARIO passed: browse, equip, Play, pause/resume, return, restart");
            exit.write(AppExit::Success);
        }
        _ => return,
    }
    scenario.step += 1;
    scenario.at = elapsed;
}
