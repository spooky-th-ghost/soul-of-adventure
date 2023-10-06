use crate::animation::AnimationState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod lateral;
mod vertical;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((lateral::LateralMovementPlugin,))
        .register_type::<Speed>()
        .register_type::<Direction>()
        .register_type::<Momentum>()
        .register_type::<Landing>();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Speed {
    accel_timer: Timer,
    decel_timer: Timer,
    base_speed: f32,
    current_speed: f32,
    base_top_speed: f32,
    top_speed: f32,
    acceleration: f32,
    deceleration: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Speed {
            accel_timer: Timer::from_seconds(0.3, TimerMode::Once),
            decel_timer: Timer::from_seconds(0.5, TimerMode::Once),
            base_speed: 7.5,
            current_speed: 7.5,
            top_speed: 15.0,
            base_top_speed: 15.0,
            acceleration: 1.0,
            deceleration: 2.0,
        }
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Direction(Vec3);

impl Direction {
    pub fn is_any(&self) -> bool {
        self.0 != Vec3::ZERO
    }

    pub fn get(&self) -> Vec3 {
        self.0
    }

    pub fn set(&mut self, value: Vec3) {
        self.0 = value;
    }

    pub fn clear(&mut self) {
        self.0 = Vec3::ZERO;
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Momentum(f32);

impl Momentum {
    pub fn is_any(&self) -> bool {
        self.0 != 0.0
    }

    pub fn clear(&mut self) {
        self.0 = 0.0;
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, value: f32) {
        self.0 = value;
    }

    pub fn add(&mut self, value: f32) {
        self.0 += value;
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Landing(Timer);

impl Landing {
    pub fn new() -> Self {
        Landing(Timer::from_seconds(0.15, TimerMode::Once))
    }

    pub fn tick(&mut self, duration: std::time::Duration) {
        self.0.tick(duration);
    }

    pub fn finished(&self) -> bool {
        self.0.finished()
    }
}

#[derive(Component)]
pub struct Grounded;

#[derive(Component, Default)]
pub struct Character {
    pub state: AnimationState,
}

#[derive(Default, Bundle)]
pub struct MovementBundle {
    rigidbody: RigidBody,
    collider: Collider,
    external_impulse: ExternalImpulse,
    velocity: Velocity,
    friction: Friction,
    damping: Damping,
    gravity_scale: GravityScale,
    direction: Direction,
    speed: Speed,
    character: Character,
}

impl MovementBundle {
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

    pub fn with_speed(mut self, speed: Speed) -> Self {
        self.speed = speed;
        self
    }
}
