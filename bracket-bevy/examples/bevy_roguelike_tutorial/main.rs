use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
mod states;
pub use states::*;
mod systems;
pub use systems::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    PreRun,
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
        .add_plugin(WorldInspectorPlugin)
        .register_type::<CombatStats>()
        .register_type::<crate::components::Name>()
        .register_type::<Position>()
        .register_type::<Player>()
        .register_type::<Monster>()
        .register_type::<BlocksTile>()
        .register_type::<WantsToMelee>()
        .register_type::<SufferDamage>()
        .add_startup_system(setup)
        .add_stage_after(
            CoreStage::Update,
            MONSTER_AI,
            SystemStage::single_threaded(),
        )
        .add_stage_after(MONSTER_AI, MAP_INDEXING, SystemStage::single_threaded())
        .add_stage_after(
            MAP_INDEXING,
            PLAYER_MOVEMENT,
            SystemStage::single_threaded(),
        )
        .add_stage_after(
            PLAYER_MOVEMENT,
            MELEE_COMBAT,
            SystemStage::single_threaded(),
        )
        .add_stage_after(MELEE_COMBAT, DAMAGE, SystemStage::single_threaded())
        .add_stage_after(DAMAGE, CORPSE_CLEANUP, SystemStage::single_threaded())
        .add_stage_after(CORPSE_CLEANUP, RENDER, SystemStage::single_threaded())
        .add_system(visibility_system)
        .add_system_to_stage(MONSTER_AI, monster_ai_system)
        .add_system_to_stage(MAP_INDEXING, map_indexing_system)
        .add_system_to_stage(PLAYER_MOVEMENT, player_movement_system)
        .add_system_to_stage(MELEE_COMBAT, melee_combat_system)
        .add_system_to_stage(DAMAGE, damage_system)
        .add_system_to_stage(CORPSE_CLEANUP, corpse_cleanup_system)
        .add_system_to_stage(RENDER, render_system)
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
