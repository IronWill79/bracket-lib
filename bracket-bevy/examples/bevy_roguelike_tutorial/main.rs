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
mod systems;
pub use systems::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

#[derive(Resource)]
pub struct GameState(RunState);

#[derive(Resource)]
pub struct PlayerPosition(Point);

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
        .insert(crate::components::Name {
            name: "Player".to_string(),
        })
        .insert(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .insert(Player {});

    let rng = RandomNumbers::new();
    for (idx, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: FontCharType;
        let name: String;
        let roll = rng.range(1, 3);
        match roll {
            1 => {
                glyph = to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = to_cp437('o');
                name = "Orc".to_string();
            }
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
            })
            .insert(crate::components::Name {
                name: format!("{} #{}", &name, idx).into(),
            })
            .insert(BlocksTile {})
            .insert(CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4,
            })
            .insert(Monster {});
    }

    commands.insert_resource(map);
    commands.insert_resource(GameState(RunState::Running));
    commands.insert_resource(PlayerPosition(Point {
        x: player_x,
        y: player_y,
    }));
}

fn tick(
    ctx: Res<BracketContext>,
    keyboard: Res<Input<KeyCode>>,
    mut map: ResMut<Map>,
    mut player_position: ResMut<PlayerPosition>,
    mut state: ResMut<GameState>,
    mut queries: ParamSet<(
        Query<(&mut Position, &mut Viewshed), With<Player>>,
        Query<(&Position, &Renderable)>,
        Query<(&mut Viewshed, &Position, Option<&Player>)>,
        Query<(&mut Viewshed, &mut Position, &crate::components::Name), With<Monster>>,
        Query<&Position, With<BlocksTile>>,
    )>,
) {
    ctx.cls();

    if state.0 == RunState::Running {
        visibility_system(&mut map, queries.p2());
        monster_ai_system(&mut map, queries.p3(), player_position.0);
        map_indexing_system(&mut map, queries.p4());
        state.0 = RunState::Paused;
    } else {
        let (delta_x, delta_y, temp_state) = player_input(&keyboard);
        let delta = (delta_x, delta_y);
        if delta != (0, 0) {
            let mut player_query = queries.p0();
            let (mut pos, mut viewshed) = player_query.single_mut();
            let destination_idx = map.xy_idx(pos.x + delta.0, pos.y + delta.1);
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

    draw_map(&map, &ctx);
    for (pos, render) in queries.p1().iter() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible_tiles[idx] {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
