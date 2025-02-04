use bevy::prelude::*;
use bracket_bevy::{prelude::*, FontCharType};

mod components;
pub use components::*;
mod gamelog;
pub use gamelog::*;
mod gui;
pub use gui::*;
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
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
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
        .add_systems(
            (
                visibility_system,
                monster_ai_system,
                apply_system_buffers,
                map_indexing_system,
                player_movement_system,
                apply_system_buffers,
                melee_combat_system,
                apply_system_buffers,
                damage_system,
                apply_system_buffers,
                corpse_cleanup_system,
                apply_system_buffers,
                render_system,
            )
                .chain(),
        )
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
    commands.insert_resource(GameState(RunState::PreRun));
    commands.insert_resource(PlayerPosition(Point {
        x: player_x,
        y: player_y,
    }));
    commands.insert_resource(GameLog {
        entries: vec!["Welcome to Rusty Roguelike".to_string()],
    })
}
