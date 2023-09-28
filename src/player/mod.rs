use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(animation::PlayerAnimationPlugin);
    }
}

mod animation;
mod movement;
