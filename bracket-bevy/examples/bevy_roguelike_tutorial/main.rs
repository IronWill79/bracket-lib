use bevy::prelude::*;
use bracket_bevy::{prelude::*, FontCharType};
use std::cmp::{max, min};

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::*;
mod visibility_system;
pub use visibility_system::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple_80x50().with_random_number_generator(true))
        .add_startup_system(setup)
        .add_system(tick)
        .run();
}

fn setup(mut commands: Commands) {
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    commands
        .spawn_empty()
        .insert(Position {
            x: player_x,
            y: player_y,
        })
        .insert(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .insert(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .insert(Player {});

    let rng = RandomNumbers::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();

        let glyph: FontCharType;
        let roll = rng.range(1, 3);
        match roll {
            1 => glyph = to_cp437('g'),
            _ => glyph = to_cp437('o'),
        }

        commands
            .spawn_empty()
            .insert(Position { x, y })
            .insert(Renderable {
                glyph,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .insert(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            });
    }

    commands.insert_resource(map);
}

fn tick(
    ctx: Res<BracketContext>,
    mut map: ResMut<Map>,
    keyboard: Res<Input<KeyCode>>,
    mut queries: ParamSet<(
        Query<(&mut Position, &mut Viewshed), With<Player>>,
        Query<(&Position, &Renderable)>,
        Query<(&mut Viewshed, &Position, Option<&Player>)>,
    )>,
) {
    ctx.cls();

    let delta = player_input(&keyboard);
    if delta != (0, 0) {
        let mut player_query = queries.p0();
        let (mut pos, mut viewshed) = player_query.single_mut();
        let destination_idx = map.xy_idx(pos.x + delta.0, pos.y + delta.1);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta.0));
            pos.y = min(49, max(0, pos.y + delta.1));

            viewshed.dirty = true;
        }
    }
    visibility_system(&mut map, queries.p2());

    draw_map(&map, &ctx);
    for (pos, render) in queries.p1().iter() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible_tiles[idx] {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
