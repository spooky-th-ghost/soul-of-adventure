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

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::default()))
        .add_plugins((room_builder::RoomBuilderPlugin, physics::PhysicsPlugin))
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::PreLoad).continue_to_state(GameState::Gameplay),
        )
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
        .add_systems(OnEnter(GameState::Gameplay), startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, 2.0, 15.6)
                .with_rotation(Quat::from_axis_angle(Vec3::X, -90.0_f32.to_radians())),
            ..default()
        },
        Name::from("Camera"),
    ));
}
