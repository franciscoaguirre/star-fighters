use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::physics::{Acceleration, Forces, Mass, Velocity};
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
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
        Forces(HashMap::new()),
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
    let thrust_force = if actions.player_thrust { 100. } else { 0. }; // Newtons
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
