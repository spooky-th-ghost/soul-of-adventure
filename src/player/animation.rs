use crate::{GameState, PlayerAnimationCache};
use bevy::prelude::*;

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), play_idle_animation);
    }
}
fn play_idle_animation(
    player_animations: Res<PlayerAnimationCache>,
    mut player_query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut player_query {
        player.play(player_animations.idle.clone_weak()).repeat();
    }
}
