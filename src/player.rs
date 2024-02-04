use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::physics::{Acceleration, Collider, Forces, Mass, Velocity};
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Gun {
    cooldown_timer: Timer,
}

#[derive(Component)]
pub struct Projectile;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (move_player, shoot).run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(Vec3::new(600., 0., 0.))
                .with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..Default::default()
        },
        Player,
        Acceleration::default(),
        Velocity::default(),
        Mass(1.0), // 1 kilogram
        Forces::default(),
        Gun {
            cooldown_timer: Timer::from_seconds(0.25, TimerMode::Once),
        },
        Collider {
            dimensions: Vec2::new(51.2, 51.2),
            should_destroy: true,
        },
    ));
}

/// Rotates the player and thrusts.
/// The thrust creates a force, which is then used to calculate the
/// net force in the `PhysicsPlugin`.
fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &mut Forces), With<Player>>,
) {
    let rotation_speed = 2.0;
    let thrust_force = if actions.player_thrust { 200. } else { 0. }; // Newtons
    if let Ok(player) = player_query.get_single() {
        let player_forward = &player.0.up(); // Seems confusing but "forward" is "up" in the 2D world
        for (mut transform, mut forces) in &mut player_query {
            if let Some(rotation) = actions.player_rotation {
                transform.rotate_z(rotation * rotation_speed * time.delta_seconds());
            }
            forces
                .0
                .insert("thrust".to_string(), *player_forward * thrust_force);
        }
    }
}

fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Transform, &mut Gun), With<Player>>,
    actions: Res<Actions>,
    time: Res<Time>,
) {
    for (transform, mut gun) in query.iter_mut() {
        if gun.cooldown_timer.tick(time.delta()).finished() {
            if actions.fire {
                commands.spawn(create_projectile(
                    transform.translation.clone() + transform.up() * 50.,
                    &transform.up(),
                    &mut meshes,
                    &mut materials,
                ));
                gun.cooldown_timer.reset();
            }
        }
    }
}

fn create_projectile(
    position: Vec3,
    up: &Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> (
    MaterialMesh2dBundle<ColorMaterial>,
    Projectile,
    Forces,
    Acceleration,
    Velocity,
    Mass,
    Collider,
) {
    let initial_velocity = *up * 500.;
    (
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_translation(position),
            ..default()
        },
        Projectile,
        Forces::default(),
        Acceleration::default(),
        Velocity(initial_velocity),
        Mass(0.1), // 100 grams
        Collider {
            dimensions: Vec2::new(5., 5.),
            should_destroy: true,
        },
    )
}
