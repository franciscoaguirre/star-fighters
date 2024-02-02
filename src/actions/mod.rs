use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::actions::game_control::{get_control, get_movement, GameControl};
use crate::player::Player;
use crate::GameState;

mod game_control;

pub const FOLLOW_EPSILON: f32 = 5.;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            set_movement_actions.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_rotation: Option<f32>,
    pub player_thrust: bool,
    pub fire: bool,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    touch_input: Res<Touches>,
    player: Query<&Transform, With<Player>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let player_rotation = get_movement(GameControl::Left, &keyboard_input)
        - get_movement(GameControl::Right, &keyboard_input);

    // if let Some(touch_position) = touch_input.first_pressed_position() {
    //     let (camera, camera_transform) = camera.single();
    //     if let Some(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position)
    //     {
    //         let diff = touch_position - player.single().translation.xy();
    //         if diff.length() > FOLLOW_EPSILON {
    //             player_movement = diff.normalize();
    //         }
    //     }
    // }

    if player_rotation != 0. {
        actions.player_rotation = Some(player_rotation);
    } else {
        actions.player_rotation = None;
    }

    actions.player_thrust = get_control(GameControl::Up, &keyboard_input);
    actions.fire = get_control(GameControl::Fire, &keyboard_input);
}
