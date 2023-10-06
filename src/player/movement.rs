use super::{Player, PlayerAction};
use crate::camera::MainCamera;
use crate::physics::{Direction, Grounded};
use crate::GameState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            set_player_direction.run_if(in_state(GameState::Gameplay)),
        );
    }
}

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

    let right_vec: Vec3 = -x * right;
    let forward_vec: Vec3 = -z * forward;

    (right_vec + forward_vec).normalize_or_zero()
}
