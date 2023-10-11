use super::{Character, Direction, Grounded, Landing, Momentum, Speed};
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
            (rotate_to_direction, handle_speed, apply_momentum)
                .run_if(in_state(GameState::Gameplay)),
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

fn handle_speed(
    time: Res<Time>,
    mut character_query: Query<(&Direction, &mut Momentum, &mut Speed), With<Grounded>>,
) {
    for (direction, mut momentum, mut speed) in &mut character_query {
        if direction.is_any() {
            speed.accelerate(&time);
            momentum.set(speed.current());
        } else {
            speed.reset();
            momentum.reset();
        }
    }
}

fn apply_momentum(mut query: Query<(&mut Velocity, &Transform, &Momentum)>) {
    for (mut velocity, transform, momentum) in &mut query {
        let mut speed_to_apply = Vec3::ZERO;
        let mut should_change_velocity: bool = false;

        if momentum.is_any() {
            should_change_velocity = true;
            let forward = transform.forward();
            speed_to_apply += forward * momentum.get();
        }

        if should_change_velocity {
            velocity.linvel.x = speed_to_apply.x;
            velocity.linvel.z = speed_to_apply.z;
        }
    }
}
