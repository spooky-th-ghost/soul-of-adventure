use super::{Character, Direction, Grounded, Landing, Speed};
use crate::camera::MainCamera;
use crate::input::PlayerAction;
use crate::player::Player;
use crate::GameState;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct LateralMovementPlugin;

impl Plugin for LateralMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (rotate_to_direction).run_if(in_state(GameState::Gameplay)),
        );
    }
}

pub fn rotate_to_direction(
    time: Res<Time>,
    mut query: Query<
        (&mut Transform, &Direction, Option<&Landing>),
        (With<Grounded>, With<Character>),
    >,
    mut rotation_target: Local<Transform>,
) {
    for (mut transform, direction, is_landing) in &mut query {
        rotation_target.translation = transform.translation;
        let flat_velo_direction = Vec3::new(direction.0.x, 0.0, direction.0.z).normalize_or_zero();
        if flat_velo_direction != Vec3::ZERO {
            let target_position = rotation_target.translation + flat_velo_direction;

            rotation_target.look_at(target_position, Vec3::Y);
            let turn_speed = if is_landing.is_some() { 20.0 } else { 10.0 };

            transform.rotation = transform
                .rotation
                .slerp(rotation_target.rotation, time.delta_seconds() * turn_speed);
        }
    }
}
