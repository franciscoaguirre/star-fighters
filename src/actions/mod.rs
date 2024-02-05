use bevy::prelude::*;

use crate::actions::game_control::{get_control, get_movement, GameControl};
use crate::GameState;

mod game_control;

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
pub struct PlayerActions {
    pub rotation: Option<f32>,
    pub thrust: bool,
    pub fire: bool,
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_actions: [PlayerActions; 2],
}

pub fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    for player_number in 0..=1 {
        let player_rotation = get_movement(GameControl::Left, &keyboard_input, player_number + 1)
            - get_movement(GameControl::Right, &keyboard_input, player_number + 1);

        if player_rotation != 0. {
            actions.player_actions[player_number as usize].rotation = Some(player_rotation);
        } else {
            actions.player_actions[player_number as usize].rotation = None;
        }

        actions.player_actions[player_number as usize].thrust =
            get_control(GameControl::Up, &keyboard_input, player_number + 1);
        actions.player_actions[player_number as usize].fire =
            get_control(GameControl::Fire, &keyboard_input, player_number + 1);
    }
}
