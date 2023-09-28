use crate::CharacterId;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct AnimationPlugin;

#[derive(Resource, Default)]
pub struct ResourceAnimationCharacterMap(HashMap<Entity, Entity>);

impl ResourceAnimationCharacterMap {
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

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationTransitionEvent>();
    }
}
