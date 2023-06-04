use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameLog {
    pub entries: Vec<String>,
}
