use crate::animation::store_animation_relationships;
use crate::input::{InputListenerBundle, PlayerAction};
use crate::physics::MovementBundle;
use crate::{Animated, CharacterCache, GameState};
use bevy::prelude::*;

mod animation;
mod movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(animation::PlayerAnimationPlugin)
            .add_systems(OnEnter(GameState::Load), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, characters: Res<CharacterCache>) {
    commands.spawn((
        Name::from("Player"),
        Player,
        Animated,
        MovementBundle::default(),
        InputListenerBundle::input_map(),
        SceneBundle {
            scene: characters.player.clone_weak(),
            transform: Transform::from_translation(Vec3::new(5.0, 5.0, 5.0)),
            ..default()
        },
    ));
}
