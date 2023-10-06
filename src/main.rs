use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::*;
use bevy_rapier3d::prelude::*;

mod animation;
mod camera;
mod input;
mod physics;
mod player;
mod room_builder;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
    #[default]
    PreLoad,
    Load,
    MainMenu,
    Gameplay,
    Transition,
}

#[derive(Default)]
pub enum CharacterId {
    #[default]
    Player,
    SkeletonWarrior,
    SkeletonMage,
}

#[derive(Component)]
pub struct Animated;

#[derive(Resource, AssetCollection)]
pub struct PlayerAnimationCache {
    #[asset(key = "idle")]
    idle: Handle<AnimationClip>,
    #[asset(key = "interact")]
    interact: Handle<AnimationClip>,
    #[asset(key = "run")]
    run: Handle<AnimationClip>,
}

#[derive(Resource, AssetCollection)]
pub struct StructureCache {
    #[asset(key = "wall")]
    wall: Handle<Scene>,
    #[asset(key = "wall_corner")]
    wall_corner: Handle<Scene>,
    #[asset(key = "door")]
    door: Handle<Scene>,
    #[asset(key = "multi_corner")]
    multi_corner: Handle<Scene>,
    #[asset(key = "t_split")]
    t_split: Handle<Scene>,
}

#[derive(Resource, AssetCollection)]
pub struct CharacterCache {
    #[asset(key = "player")]
    player: Handle<Scene>,
}

#[derive(Default, Reflect)]
pub struct CameraBounds {
    pub max_x: f32,
    pub min_x: f32,
    pub max_z: f32,
    pub min_z: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CameraController {
    pub x_angle: f32,
    pub offset: Vec3,
    pub target_position: Vec3,
    pub bounds: CameraBounds,
}

fn rotate_camera(mut camera_query: Query<(&mut Transform, &CameraController)>) {
    for (mut transform, controller) in &mut camera_query {
        transform.rotation.x = -1.0 * controller.x_angle.to_radians();
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::default()))
        .add_plugins((
            animation::AnimationPlugin,
            room_builder::RoomBuilderPlugin,
            physics::PhysicsPlugin,
            player::PlayerPlugin,
            input::InputPlugin,
        ))
        .add_state::<GameState>()
        .register_type::<CameraController>()
        .register_type::<CameraBounds>()
        .add_loading_state(LoadingState::new(GameState::PreLoad).continue_to_state(GameState::Load))
        .add_collection_to_loading_state::<_, PlayerAnimationCache>(GameState::PreLoad)
        .add_collection_to_loading_state::<_, StructureCache>(GameState::PreLoad)
        .add_collection_to_loading_state::<_, CharacterCache>(GameState::PreLoad)
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::PreLoad,
            "manifests/static_models.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::PreLoad,
            "manifests/player_animations.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::PreLoad,
            "manifests/character_models.assets.ron",
        )
        .add_systems(OnEnter(GameState::Load), startup)
        .add_systems(Update, move_to_gameplay.run_if(in_state(GameState::Load)))
        .add_systems(Update, rotate_camera.run_if(in_state(GameState::Gameplay)))
        .run();
}

fn move_to_gameplay(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Gameplay);
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(14.0, 16.5, 21.5),
            ..default()
        },
        CameraController {
            x_angle: 24.5,
            ..default()
        },
        Name::from("Camera"),
        camera::MainCamera {
            target_transform: Transform::default(),
        },
    ));
}
