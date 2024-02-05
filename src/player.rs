use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::physics::{Collider, Forces, Mass, PhysicsBundle, Velocity};
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    number: u8,
}

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
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_player_1, spawn_player_2),
        )
        .add_systems(
            Update,
            (move_player, shoot).run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_player_1(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(create_player(&textures, Vec3::new(-600., 0., 0.), 1));
}

fn spawn_player_2(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(create_player(&textures, Vec3::new(600., 0., 0.), 2));
}

fn create_player(
    textures: &Res<TextureAssets>,
    position: Vec3,
    player_number: u8,
) -> (SpriteBundle, Player, Gun, PhysicsBundle, Collider) {
    (
        SpriteBundle {
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(position).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..Default::default()
        },
        Player {
            number: player_number,
        },
        Gun {
            cooldown_timer: Timer::from_seconds(0.25, TimerMode::Once),
        },
        PhysicsBundle::default(),
        Collider {
            dimensions: Vec2::new(51.2, 51.2),
            destroyable: true,
        },
    )
}

/// Rotates the player and thrusts.
/// The thrust creates a force, which is then used to calculate the
/// net force in the `PhysicsPlugin`.
fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &mut Forces, &Player)>,
) {
    let rotation_speed = 2.0;
    for (mut transform, mut forces, player) in &mut player_query {
        let player_actions = &actions.player_actions[(player.number - 1) as usize];
        let thrust_force = if player_actions.thrust { 200. } else { 0. }; // Newtons
        let player_forward = transform.up().clone(); // Seems confusing but "forward" is "up" in the 2D world
        if let Some(rotation) = player_actions.rotation {
            transform.rotate_z(rotation * rotation_speed * time.delta_seconds());
        }
        forces
            .0
            .insert("thrust".to_string(), player_forward * thrust_force);
    }
}

fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Transform, &mut Gun, &Player)>,
    actions: Res<Actions>,
    time: Res<Time>,
) {
    for (transform, mut gun, player) in query.iter_mut() {
        if gun.cooldown_timer.tick(time.delta()).finished() {
            let player_actions = &actions.player_actions[(player.number - 1) as usize];
            if player_actions.fire {
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
    PhysicsBundle,
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
        PhysicsBundle {
            mass: Mass(0.1), // 100 grams
            velocity: Velocity(initial_velocity),
            ..default()
        },
        Collider {
            dimensions: Vec2::new(5., 5.),
            destroyable: true,
        },
    )
}
