use bevy::prelude::*;

use crate::game::GameState;

/// Marker for crosshair UI elements
#[derive(Component)]
pub struct CrosshairRoot;

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_crosshair)
            .add_systems(OnExit(GameState::Playing), cleanup_crosshair);
    }
}

fn spawn_crosshair(mut commands: Commands) {
    let crosshair_size = 4.0;

    commands
        .spawn((
            CrosshairRoot,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            GlobalZIndex(50),
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode::solid_color(Color::srgb(0., 1., 0.)),
                Node {
                    width: Val::Px(crosshair_size),
                    height: Val::Px(crosshair_size),
                    ..default()
                },
            ));
        });
}

fn cleanup_crosshair(mut commands: Commands, query: Query<Entity, With<CrosshairRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
