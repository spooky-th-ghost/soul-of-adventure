use crate::Animated;
use crate::CharacterId;
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationCharacterMap::default())
            .add_event::<AnimationTransitionEvent>()
            .add_systems(
                Update,
                store_animation_relationships.run_if(in_state(GameState::Gameplay)),
            );
    }
}

#[derive(Resource, Default)]
pub struct AnimationCharacterMap(HashMap<Entity, Entity>);

impl AnimationCharacterMap {
    pub fn get(&self, key_entity: Entity) -> Option<Entity> {
        self.0.get(&key_entity).copied()
    }

    pub fn insert(&mut self, key_entity: Entity, value_entity: Entity) {
        self.0.insert(key_entity, value_entity);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Default)]
pub enum AnimationState {
    #[default]
    Idle,
    Walk,
    Run,
    Attacking,
    Rising,
    Falling,
    Hurt,
    Knockdown,
}

pub enum AnimationTransition {
    ToIdle,
    ToRun,
    ToPickup,
    ToKick,
    ToInteract,
    ToJump,
    ToThrow,
    ToAttack(u8),
}

#[derive(Event)]
pub struct AnimationTransitionEvent {
    pub character_id: CharacterId,
    pub transition: AnimationTransition,
    pub parent_entity: Entity,
}

pub fn store_animation_relationships(
    mut commands: Commands,
    mut animation_character_map: ResMut<AnimationCharacterMap>,
    child_query: Query<(Entity, &Parent), Added<AnimationPlayer>>,
    grandparent_query: Query<(Entity, &Children), With<Animated>>,
) {
    for (grandchild_entity, grandchild_parent) in &child_query {
        for (grandparent_entity, grandparent_children) in &grandparent_query {
            if grandparent_children
                .into_iter()
                .any(|entity| *entity == grandchild_parent.get())
            {
                animation_character_map.insert(grandparent_entity, grandchild_entity);
                commands.entity(grandparent_entity).remove::<Animated>();
            }
        }
    }
}
