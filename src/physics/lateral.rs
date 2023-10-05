use super::{Character, Direction, Grounded, Landing, Speed};
use crate::camera::MainCamera;
use crate::input::PlayerAction;
use crate::player::Player;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub struct LateralMovementPlugin;

pub fn set_player_direction(
    mut player_query: Query<
        (
            &mut Direction,
            Option<&Grounded>,
            &ActionState<PlayerAction>,
        ),
        With<Player>,
    >,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let camera_transform = camera_query.single();
    for (mut direction, grounded, action) in &mut player_query {
        if grounded.is_some() {
            direction.set(get_direction_in_camera_space(camera_transform, action));
        } else {
            if direction.is_any() {
                direction.clear();
            }
        }
    }
}

pub fn get_direction_in_camera_space(
    camera_transform: &Transform,
    action: &ActionState<PlayerAction>,
) -> Vec3 {
    let mut x = 0.0;
    let mut z = 0.0;

    let mut forward = camera_transform.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut right = camera_transform.right();
    right.y = 0.0;
    right = right.normalize();

    if action.pressed(PlayerAction::Move) {
        let axis_pair = action.clamped_axis_pair(PlayerAction::Move).unwrap();
        x = axis_pair.x();
        z = axis_pair.y();
    }

    let right_vec: Vec3 = x * right;
    let forward_vec: Vec3 = z * forward;

    (right_vec + forward_vec).normalize_or_zero()
}

pub fn rotate_to_direction(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Direction, Option<&Landing>), With<Grounded>>,
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
