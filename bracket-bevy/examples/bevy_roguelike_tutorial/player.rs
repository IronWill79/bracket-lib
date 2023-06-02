use std::cmp::{max, min};

use bevy::prelude::*;

use crate::{CombatStats, GameState, Map, Player, PlayerPosition, Position, RunState, Viewshed};

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
    } else if keyboard.just_pressed(KeyCode::Numpad9) || keyboard.just_pressed(KeyCode::Y) {
        result.0 = 1;
        result.1 = -1;
    } else if keyboard.just_pressed(KeyCode::Numpad7) || keyboard.just_pressed(KeyCode::U) {
        result.0 = -1;
        result.1 = -1;
    } else if keyboard.just_pressed(KeyCode::Numpad3) || keyboard.just_pressed(KeyCode::N) {
        result.0 = 1;
        result.1 = 1;
    } else if keyboard.just_pressed(KeyCode::Numpad1) || keyboard.just_pressed(KeyCode::B) {
        result.0 = -1;
        result.1 = 1;
    } else {
        result.2 = RunState::Paused;
    }
    result
}

pub fn player_movement_system(
    keyboard: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut player_position: ResMut<PlayerPosition>,
    mut player_query: Query<(&mut Position, &mut Viewshed), With<Player>>,
    mut state: ResMut<GameState>,
    target_query: Query<(Entity, &CombatStats)>,
) {
    if state.0 == RunState::Paused {
        let (delta_x, delta_y, temp_state) = player_input(&keyboard);
        let delta = (delta_x, delta_y);
        if delta != (0, 0) {
            let (mut pos, mut viewshed) = player_query.single_mut();
            let destination_idx = map.xy_idx(pos.x + delta.0, pos.y + delta.1);

            for potential_target in map.tile_content[destination_idx].iter() {
                if let Ok((_target, _target_stats)) = target_query.get(*potential_target) {
                    println!("From Hell's heart, I stab at thee!!");
                    return;
                }
            }
            if !map.blocked[destination_idx] {
                pos.x = min(79, max(0, pos.x + delta.0));
                pos.y = min(49, max(0, pos.y + delta.1));
                player_position.0.x = pos.x;
                player_position.0.y = pos.y;

                viewshed.dirty = true;
            }
        }
        state.0 = temp_state;
    }
}
