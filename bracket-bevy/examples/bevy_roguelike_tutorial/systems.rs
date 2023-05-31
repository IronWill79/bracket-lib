use bevy::prelude::{Entity, Query, With};
use bracket_geometry::prelude::{DistanceAlg, Point};
use bracket_pathfinding::prelude::{a_star_search, field_of_view};

use crate::{components::Name, BlocksTile, Map, Monster, Player, Position, Viewshed};

pub fn visibility_system(
    map: &mut Map,
    mut viewshed_query: Query<(&mut Viewshed, &Position, Option<&Player>)>,
) {
    for (mut viewshed, position, player) in viewshed_query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles =
                field_of_view(Point::new(position.x, position.y), viewshed.range, map);
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

pub fn monster_ai_system(
    map: &mut Map,
    mut monster_query: Query<(&mut Viewshed, &mut Position, &Name), With<Monster>>,
    player_position: Point,
) {
    for (mut viewshed, mut position, name) in monster_query.iter_mut() {
        if viewshed.visible_tiles.contains(&player_position) {
            let distance = DistanceAlg::Pythagoras
                .distance2d(Point::new(position.x, position.y), player_position);
            if distance < 1.5 {
                // Attack goes here
                println!("{} shouts insults", name.name);
                return;
            }
            let path = a_star_search(
                map.xy_idx(position.x, position.y) as i32,
                map.xy_idx(player_position.x, player_position.y) as i32,
                map,
            );
            if path.success && path.steps.len() > 1 {
                println!("{} steps towards you", name.name);
                position.x = path.steps[1] as i32 % map.width;
                position.y = path.steps[1] as i32 / map.width;
                viewshed.dirty = true;
            }
        }
    }
}

/// This system iterates through all entities with a Position and a BlocksTile component,
/// updating the map's blocked vector with the current state
pub fn map_indexing_system(
    map: &mut Map,
    blocked_query: Query<(Entity, &Position, Option<&BlocksTile>)>,
) {
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
