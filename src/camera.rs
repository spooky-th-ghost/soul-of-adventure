use crate::GameState;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), spawn_camera)
            .add_systems(
                Update,
                (update_camera_position).run_if(in_state(GameState::Gameplay)),
            );
    }
}

#[derive(Component)]
pub struct MainCamera {
    pub target_transform: Transform,
}

#[derive(Event)]
pub struct RoomTransitionEvent {
    pub target_transform: Transform,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        MainCamera {
            target_transform: Transform::from_xyz(0.0, 7.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        },
    ));
}

fn update_camera_position(time: Res<Time>, mut camera_query: Query<(&mut Transform, &MainCamera)>) {
    if let Ok((mut transform, camera)) = camera_query.get_single_mut() {
        if transform
            .translation
            .distance(camera.target_transform.translation)
            > 0.02
        {
            transform.translation = transform.translation.lerp(
                camera.target_transform.translation,
                time.delta_seconds() * 20.0,
            );
        }

        if transform
            .rotation
            .angle_between(camera.target_transform.rotation)
            > 0.02
        {
            transform.rotation = transform.rotation.slerp(
                camera.target_transform.rotation,
                time.delta_seconds() * 20.0,
            );
        }
    }
}
