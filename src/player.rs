use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::velocity::Velocity;
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
            transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                .with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..Default::default()
        },
        Player,
        Velocity::default(),
    ));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let rotation_speed = 2.0;
    let thrust_force = 5.;
    // Seems confusing but "forward" is "up" in the 2D world
    let player_forward = &player_query.single().0.up();
    for (mut transform, mut velocity) in &mut player_query {
        if let Some(rotation) = actions.player_rotation {
            transform.rotate_z(rotation * rotation_speed * time.delta_seconds());
        }
        if actions.player_thrust {
            velocity.0 += *player_forward * thrust_force;
        }
    }
}
