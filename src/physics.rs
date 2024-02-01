use bevy::prelude::*;

use crate::loading::TextureAssets;
use crate::GameState;

#[derive(Component, Default)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Mass(pub f32);

// Objects so massive that they attract other objects with their gravity.
#[derive(Component)]
pub struct Star;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_star)
            .add_systems(
                Update,
                (apply_acceleration, apply_velocity, apply_gravity)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_star(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                .with_scale(Vec3::new(0.5, 0.5, 0.5)),
            ..default()
        },
        Mass(1000.),
        Star,
    ));
}

fn apply_acceleration(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn apply_gravity(
    mut objects_query: Query<(&Mass, &Transform, &mut Acceleration), Without<Star>>,
    stars_query: Query<(&Mass, &Transform), With<Star>>,
) {
    for (mass, transform, mut acceleration) in objects_query.iter_mut() {
        for (star_mass, star_transform) in stars_query.iter() {
            // F = G * m1 * m2 / r^2
            // G is too small to account for. m2 as well.
            let direction_to_star = star_transform.translation - transform.translation;
            let distance_to_star = direction_to_star.length();
            let gravitational_force = star_mass.0 / distance_to_star.powi(2);
            // F = m * a -> a = F / m
            acceleration.0 = direction_to_star * gravitational_force / mass.0;
        }
    }
}
