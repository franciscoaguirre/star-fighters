use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::utils::HashMap;

use crate::loading::TextureAssets;
use crate::GameState;

#[derive(Component)]
pub struct Collider {
    pub dimensions: Vec2,
    pub destroyable: bool,
}

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub forces: Forces,
    pub acceleration: Acceleration,
    pub velocity: Velocity,
    pub mass: Mass,
}

#[derive(Component, Debug, Default)]
pub struct Forces(pub HashMap<String, Vec3>);

#[derive(Component, Default)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.0)
    }
}

// Objects so massive that they attract other objects with their gravity.
#[derive(Component)]
pub struct Star;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_star)
            .add_systems(
                Update,
                (
                    apply_gravity,
                    apply_forces,
                    apply_acceleration,
                    apply_velocity,
                    check_for_collisions,
                )
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
        Mass(10_000.),
        Collider {
            // 128 is half the image used because of scale. We also give some 32px of extra space.
            dimensions: Vec2::new(96., 96.),
            destroyable: false, // Should never destroy a star
        },
        Star,
    ));
}

/// Sum all forces being applied to entities, in order to get the net force.
/// Then, modify the acceleration according to Newton's 2nd law.
fn apply_forces(mut query: Query<(&Forces, &Mass, &mut Acceleration)>) {
    for (forces, mass, mut acceleration) in query.iter_mut() {
        // F = m * a -> a = F / m
        acceleration.0 = forces.0.values().sum::<Vec3>() / mass.0;
    }
}

/// Change entities' velocity according to their acceleration.
fn apply_acceleration(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

/// Change entities' positions according to their velocity.
fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

/// Apply a gravitational force from entities with the `Star` component
/// to those that don't have it.
fn apply_gravity(
    mut objects_query: Query<(&Transform, &mut Forces), Without<Star>>,
    stars_query: Query<(&Mass, &Transform), With<Star>>,
) {
    for (transform, mut forces) in objects_query.iter_mut() {
        for (star_mass, star_transform) in stars_query.iter() {
            // F = G * m1 * m2 / r^2
            // G is too small to account for. m2 as well.
            let direction_to_star = star_transform.translation - transform.translation;
            let distance_to_star = direction_to_star.length();
            let gravitational_force = star_mass.0 / distance_to_star.powi(2);
            forces.0.insert(
                "gravity".to_string(),
                direction_to_star * gravitational_force,
            );
        }
    }
}

fn check_for_collisions(mut commands: Commands, query: Query<(Entity, &Transform, &Collider)>) {
    let mut iter = query.iter_combinations();

    while let Some([(entity1, transform1, collider1), (entity2, transform2, collider2)]) =
        iter.fetch_next()
    {
        let collision = collide(
            transform1.translation,
            collider1.dimensions,
            transform2.translation,
            collider2.dimensions,
        );
        if collision.is_some() {
            if collider1.destroyable {
                commands.entity(entity1).despawn();
            }
            if collider2.destroyable {
                commands.entity(entity2).despawn();
            }
        }
    }
}
