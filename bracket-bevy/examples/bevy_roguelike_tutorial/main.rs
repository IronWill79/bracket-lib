use bevy::prelude::*;
use bracket_bevy::{prelude::*, FontCharType};

mod components;
pub use components::*;
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
        .add_system(visibility_system.before(monster_ai_system))
        .add_system(
            monster_ai_system
                .after(visibility_system)
                .before(map_indexing_system),
        )
        .add_system(
            map_indexing_system
                .after(monster_ai_system)
                .before(player_movement_system),
        )
        .add_system(
            player_movement_system
                .after(map_indexing_system)
                .before(melee_combat_system),
        )
        .add_system(
            melee_combat_system
                .after(player_movement_system)
                .before(damage_system),
        )
        .add_system(
            damage_system
                .after(melee_combat_system)
                .before(corpse_cleanup_system),
        )
        .add_system(
            corpse_cleanup_system
                .after(damage_system)
                .before(render_system),
        )
        .add_system(render_system.after(corpse_cleanup_system))
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
}
