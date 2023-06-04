use bevy::prelude::*;
use bracket_bevy::prelude::RGB;
use bracket_geometry::prelude::Point;

#[derive(Component, Reflect)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u16,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug, Reflect)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug, Reflect)]
pub struct Monster {}

#[derive(Component, Debug, Reflect)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug, Reflect)]
pub struct BlocksTile {}

#[derive(Component, Debug, Reflect)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug, Reflect)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}
