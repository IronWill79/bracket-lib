use std::cmp::{max, min};

use bevy::prelude::*;

use crate::{
    CombatStats, GameState, Map, Player, PlayerPosition, Position, RunState, Viewshed, WantsToMelee,
};

pub fn player_input(keyboard: &Input<KeyCode>) -> (i32, i32, RunState) {
    let mut result = (0, 0, RunState::PlayerTurn);

    let left = keyboard.any_just_pressed([KeyCode::Left, KeyCode::Numpad4, KeyCode::H]);
    let right = keyboard.any_just_pressed([KeyCode::Right, KeyCode::Numpad6, KeyCode::L]);
    let up = keyboard.any_just_pressed([KeyCode::Up, KeyCode::Numpad8, KeyCode::K]);
    let down = keyboard.any_just_pressed([KeyCode::Down, KeyCode::Numpad2, KeyCode::J]);
    let up_left = keyboard.any_just_pressed([KeyCode::Numpad7, KeyCode::U]);
    let up_right = keyboard.any_just_pressed([KeyCode::Numpad9, KeyCode::Y]);
    let down_left = keyboard.any_just_pressed([KeyCode::Numpad1, KeyCode::B]);
    let down_right = keyboard.any_just_pressed([KeyCode::Numpad3, KeyCode::N]);

    if left {
        result.0 = -1;
    } else if right {
        result.0 = 1;
    } else if up {
        result.1 = -1;
    } else if down {
        result.1 = 1;
    } else if up_right {
        result.0 = 1;
        result.1 = -1;
    } else if up_left {
        result.0 = -1;
        result.1 = -1;
    } else if down_right {
        result.0 = 1;
        result.1 = 1;
    } else if down_left {
        result.0 = -1;
        result.1 = 1;
    } else {
        result.2 = RunState::AwaitingInput;
    }
    result
}

pub fn player_movement_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut player_position: ResMut<PlayerPosition>,
    mut player_query: Query<(Entity, &mut Position, &mut Viewshed), With<Player>>,
    mut state: ResMut<GameState>,
    target_query: Query<(Entity, &CombatStats)>,
) {
    if state.0 == RunState::AwaitingInput {
        let (delta_x, delta_y, temp_state) = player_input(&keyboard);
        let delta = (delta_x, delta_y);
        if delta != (0, 0) {
            let (player, mut pos, mut viewshed) = player_query.single_mut();
            let destination_idx = map.xy_idx(pos.x + delta.0, pos.y + delta.1);

            for potential_target in map.tile_content[destination_idx].iter() {
                if let Ok((target, _target_stats)) = target_query.get(*potential_target) {
                    println!("From Hell's heart, I stab at thee!!");
                    commands.entity(player).insert(WantsToMelee { target });
                    state.0 = temp_state;
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
