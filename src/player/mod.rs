use crate::input::{InputListenerBundle, PlayerAction};
use crate::physics::MovementBundle;
use crate::{CharacterCache, GameState};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(animation::PlayerAnimationPlugin)
            .add_systems(OnEnter(GameState::Gameplay), spawn_player);
    }
}

mod animation;
mod movement;

fn spawn_player(mut commands: Commands, characters: Res<CharacterCache>) {
    commands.spawn((
        Name::from("Player"),
        MovementBundle::default().with_translation(Vec3::new(5.0, 5.0, -5.0)),
        InputListenerBundle::input_map(),
        SceneBundle {
            scene: characters.player.clone_weak(),
            ..default()
        },
    ));
}
