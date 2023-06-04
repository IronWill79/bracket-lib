use bevy::prelude::{Commands, Entity, Query, Res, ResMut, With};
use bracket_bevy::BracketContext;
use bracket_geometry::prelude::{DistanceAlg, Point};
use bracket_pathfinding::prelude::{a_star_search, field_of_view};

use crate::{
    components::Name, draw_map, draw_ui, BlocksTile, CombatStats, GameState, Map, Monster, Player,
    PlayerPosition, Position, Renderable, RunState, SufferDamage, Viewshed, WantsToMelee,
};

pub fn visibility_system(
    state: Res<GameState>,
    mut map: ResMut<Map>,
    mut viewshed_query: Query<(&mut Viewshed, &Position, Option<&Player>)>,
) {
    if state.0 != RunState::AwaitingInput {
        for (mut viewshed, position, player) in viewshed_query.iter_mut() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(
                    Point::new(position.x, position.y),
                    viewshed.range,
                    map.as_ref(),
                );
                viewshed
                    .visible_tiles
                    .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

                if let Some(_player) = player {
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}

pub fn monster_ai_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut monster_query: Query<(Entity, &mut Viewshed, &mut Position, &Name), With<Monster>>,
    player_position: Res<PlayerPosition>,
    player_query: Query<Entity, With<Player>>,
    state: Res<GameState>,
) {
    if state.0 == RunState::MonsterTurn {
        for (entity, mut viewshed, mut position, name) in monster_query.iter_mut() {
            if viewshed.visible_tiles.contains(&player_position.0) {
                let distance = DistanceAlg::Pythagoras
                    .distance2d(Point::new(position.x, position.y), player_position.0);
                if distance < 1.5 {
                    // Attack goes here
                    println!("{} shouts insults", name.name);
                    let player = player_query.single();
                    commands
                        .entity(entity)
                        .insert(WantsToMelee { target: player });
                    return;
                }
                let path = a_star_search(
                    map.xy_idx(position.x, position.y) as i32,
                    map.xy_idx(player_position.0.x, player_position.0.y) as i32,
                    map.as_ref(),
                );
                if path.success && path.steps.len() > 1 {
                    let mut idx = map.xy_idx(position.x, position.y);
                    map.blocked[idx] = false;
                    println!("{} steps towards you", name.name);
                    position.x = path.steps[1] as i32 % map.width;
                    position.y = path.steps[1] as i32 / map.width;
                    idx = map.xy_idx(position.x, position.y);
                    map.blocked[idx] = true;
                    viewshed.dirty = true;
                }
            }
        }
    }
}

/// This system iterates through all entities with a Position and a BlocksTile component,
/// updating the map's blocked vector with the current state and the tile_content vector with any
/// entities on the tile
pub fn map_indexing_system(
    blocked_query: Query<(Entity, &Position, Option<&BlocksTile>)>,
    mut map: ResMut<Map>,
    state: Res<GameState>,
) {
    if state.0 != RunState::AwaitingInput {
        map.populate_blocked();
        map.clear_content_index();
        for (entity, position, _blocks_tile) in blocked_query.iter() {
            let index = map.xy_idx(position.x, position.y);

            if let Some(_blocks_tile) = _blocks_tile {
                map.blocked[index] = true;
            }

            map.tile_content[index].push(entity);
        }
    }
}

pub fn melee_combat_system(
    attackers_query: Query<(
        Entity,
        &WantsToMelee,
        &crate::components::Name,
        &CombatStats,
    )>,
    mut commands: Commands,
    state: Res<GameState>,
    mut targets_query: Query<(
        Entity,
        &crate::components::Name,
        &CombatStats,
        Option<&mut SufferDamage>,
    )>,
) {
    if state.0 != RunState::AwaitingInput {
        for (attacker, wants_melee, name, stats) in attackers_query.iter() {
            if stats.hp > 0 {
                if let Ok((target, target_name, target_stats, suffering)) =
                    targets_query.get_mut(wants_melee.target)
                {
                    if target_stats.hp > 0 {
                        let damage = i32::max(0, stats.power - target_stats.defense);

                        if damage == 0 {
                            println!("{} is unable to hurt {}", name.name, target_name.name);
                        } else {
                            println!("{} hits {} for {} hp", name.name, target_name.name, damage);
                            if let Some(mut suffering) = suffering {
                                suffering.amount.push(damage);
                            } else {
                                commands.entity(target).insert(SufferDamage {
                                    amount: vec![damage],
                                });
                            }
                        }
                    }
                }
            }
            commands.entity(attacker).remove::<WantsToMelee>();
        }
    }
}

pub fn damage_system(
    mut attacked_query: Query<(Entity, &mut CombatStats, &SufferDamage)>,
    mut commands: Commands,
    mut state: ResMut<GameState>,
) {
    if state.0 != RunState::AwaitingInput {
        for (entity, mut stats, damage) in attacked_query.iter_mut() {
            stats.hp -= damage.amount.iter().sum::<i32>();
            commands.entity(entity).remove::<SufferDamage>();
        }
        match state.0 {
            RunState::AwaitingInput => {}
            RunState::PreRun => state.0 = RunState::AwaitingInput,
            RunState::PlayerTurn => state.0 = RunState::MonsterTurn,
            RunState::MonsterTurn => state.0 = RunState::AwaitingInput,
        }
    }
}

pub fn corpse_cleanup_system(
    mut commands: Commands,
    corpse_query: Query<(Entity, &CombatStats, Option<&Player>)>,
) {
    for (entity, stats, player) in corpse_query.iter() {
        if stats.hp < 1 {
            if let Some(_player) = player {
                println!("You are dead");
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn render_system(
    ctx: Res<BracketContext>,
    map: Res<Map>,
    renderable_query: Query<(&Position, &Renderable)>,
) {
    ctx.cls();

    draw_map(&map, &ctx);

    for (pos, render) in renderable_query.iter() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible_tiles[idx] {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }

    draw_ui(&ctx, player_stats_query);
}
