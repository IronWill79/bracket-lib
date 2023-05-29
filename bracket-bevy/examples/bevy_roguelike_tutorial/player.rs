use bevy::prelude::*;

use crate::RunState;

pub fn player_input(keyboard: &Input<KeyCode>) -> (i32, i32, RunState) {
    let mut result = (0, 0, RunState::Running);

    if keyboard.just_pressed(KeyCode::Left)
        || keyboard.just_pressed(KeyCode::Numpad4)
        || keyboard.just_pressed(KeyCode::H)
    {
        result.0 = -1;
    } else if keyboard.just_pressed(KeyCode::Right)
        || keyboard.just_pressed(KeyCode::Numpad6)
        || keyboard.just_pressed(KeyCode::L)
    {
        result.0 = 1;
    } else if keyboard.just_pressed(KeyCode::Up)
        || keyboard.just_pressed(KeyCode::Numpad8)
        || keyboard.just_pressed(KeyCode::K)
    {
        result.1 = -1;
    } else if keyboard.just_pressed(KeyCode::Down)
        || keyboard.just_pressed(KeyCode::Numpad2)
        || keyboard.just_pressed(KeyCode::J)
    {
        result.1 = 1;
    } else {
        result.2 = RunState::Paused;
    }
    result
}
