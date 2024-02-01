use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::physics::{Acceleration, Mass, Velocity};
use crate::GameState;
use bevy::prelude::*;

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
    ));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &mut Acceleration, &Mass), With<Player>>,
) {
    let rotation_speed = 2.0;
    let thrust_force = if actions.player_thrust { 200. } else { 0. }; // Newtons
    let player_forward = &player_query.single().0.up(); // Seems confusing but "forward" is "up" in the 2D world
    for (mut transform, mut acceleration, mass) in &mut player_query {
        if let Some(rotation) = actions.player_rotation {
            transform.rotate_z(rotation * rotation_speed * time.delta_seconds());
        }
        // F = m * a -> a = F / m
        acceleration.0 = *player_forward * thrust_force / mass.0;
    }
}
