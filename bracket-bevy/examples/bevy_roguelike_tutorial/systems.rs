use bevy::prelude::{Query, With};
use bracket_geometry::prelude::Point;
use bracket_pathfinding::prelude::field_of_view;

use crate::{Map, Monster, Player, Position, Viewshed};

pub fn visibility_system(
    map: &mut Map,
    mut viewshed_query: Query<(&mut Viewshed, &Position, Option<&Player>)>,
) {
    for (mut viewshed, position, player) in viewshed_query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles =
                field_of_view(Point::new(position.x, position.y), viewshed.range, &*map);
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

pub fn monster_ai_system(mut monster_query: Query<(&Viewshed, &Position), With<Monster>>) {
    for (viewshed, position) in monster_query.iter_mut() {
        println!("Monster considers its own existence");
    }
}
