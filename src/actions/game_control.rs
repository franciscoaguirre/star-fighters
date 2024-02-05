use bevy::prelude::{Input, KeyCode, Res};

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Fire,
}

impl GameControl {
    pub fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>, player_number: u8) -> bool {
        match self {
            GameControl::Up => {
                if player_number == 1 {
                    keyboard_input.pressed(KeyCode::W)
                } else {
                    keyboard_input.pressed(KeyCode::Up)
                }
            }
            GameControl::Down => {
                if player_number == 1 {
                    keyboard_input.pressed(KeyCode::S)
                } else {
                    keyboard_input.pressed(KeyCode::Down)
                }
            }
            GameControl::Left => {
                if player_number == 1 {
                    keyboard_input.pressed(KeyCode::A)
                } else {
                    keyboard_input.pressed(KeyCode::Left)
                }
            }
            GameControl::Right => {
                if player_number == 1 {
                    keyboard_input.pressed(KeyCode::D)
                } else {
                    keyboard_input.pressed(KeyCode::Right)
                }
            }
            GameControl::Fire => {
                if player_number == 1 {
                    keyboard_input.pressed(KeyCode::Space)
                } else {
                    keyboard_input.pressed(KeyCode::ControlRight)
                }
            }
        }
    }
}

pub fn get_movement(control: GameControl, input: &Res<Input<KeyCode>>, player_number: u8) -> f32 {
    if control.pressed(input, player_number) {
        1.0
    } else {
        0.0
    }
}

pub fn get_control(control: GameControl, input: &Res<Input<KeyCode>>, player_number: u8) -> bool {
    control.pressed(input, player_number)
}
