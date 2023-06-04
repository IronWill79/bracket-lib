use bevy::prelude::{Query, With};
use bracket_bevy::BracketContext;
use bracket_color::{
    prelude::{BLACK, RED, WHITE, YELLOW},
    rgb::RGB,
};

use crate::{CombatStats, Player};

pub fn draw_ui(ctx: &BracketContext, player_stats_query: Query<&CombatStats, With<Player>>) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(WHITE), RGB::named(BLACK));

    let stats = player_stats_query.single();
    let health = format!(" HP: {} / {}", stats.hp, stats.max_hp);
    ctx.print_color(12, 43, health, RGB::named(YELLOW), RGB::named(BLACK));

    ctx.draw_bar_horizontal(
        28,
        43,
        51,
        stats.hp,
        stats.max_hp,
        RGB::named(RED),
        RGB::named(BLACK),
    )
}
