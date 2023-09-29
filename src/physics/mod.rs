use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ));
    }
}

#[derive(Default, Bundle)]
pub struct MovementBundle {
    transform: Transform,
    rigidbody: RigidBody,
    collider: Collider,
    external_impulse: ExternalImpulse,
    velocity: Velocity,
    friction: Friction,
    damping: Damping,
    gravity_scale: GravityScale,
}

impl MovementBundle {
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.transform.translation = translation;
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.transform.rotation = rotation;
        self
    }

    pub fn looking_at(mut self, target: Vec3, up: Vec3) -> Self {
        self.transform.look_at(target, up);
        self
    }

    pub fn with_rigidbody(mut self, rigidbody: RigidBody) -> Self {
        self.rigidbody = rigidbody;
        self
    }

    pub fn with_collider(mut self, collider: Collider) -> Self {
        self.collider = collider;
        self
    }

    pub fn with_impulse(mut self, external_impulse: ExternalImpulse) -> Self {
        self.external_impulse = external_impulse;
        self
    }

    pub fn with_velocity(mut self, velocity: Velocity) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_damping(mut self, damping: Damping) -> Self {
        self.damping = damping;
        self
    }

    pub fn with_friction(mut self, friction: Friction) -> Self {
        self.friction = friction;
        self
    }

    pub fn with_gravity_scale(mut self, gravity_scale: f32) -> Self {
        self.gravity_scale = GravityScale(gravity_scale);
        self
    }
}
