use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

pub struct VelocityPlugin;
impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity);
    }
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}
