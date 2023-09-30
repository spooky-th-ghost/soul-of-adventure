use super::Player;
use crate::animation::{store_animation_relationships, AnimationCharacterMap};
use crate::{GameState, PlayerAnimationCache};
use bevy::prelude::*;

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            play_idle_animation.run_if(in_state(GameState::Gameplay)),
        );
    }
}
fn play_idle_animation(
    player_animations: Res<PlayerAnimationCache>,
    animation_map: Res<AnimationCharacterMap>,
    player_query: Query<Entity, With<Player>>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
) {
    for entity in &player_query {
        if let Some(animation_entity) = animation_map.get(entity) {
            if let Ok(mut animation_player) = animation_player_query.get_mut(animation_entity) {
                animation_player
                    .play(player_animations.idle.clone_weak())
                    .repeat();
            }
        }
    }
}
