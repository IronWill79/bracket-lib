use bevy::prelude::{Query, Res, With};
use bracket_bevy::BracketContext;
use bracket_color::{
    prelude::{BLACK, MAGENTA, RED, WHITE, YELLOW},
    rgb::RGB,
};

use crate::{CombatStats, GameLog, Player};

pub fn draw_ui(
    ctx: &BracketContext,
    player_stats_query: Query<&CombatStats, With<Player>>,
    log: Res<GameLog>,
) {
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
    );

    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, s);
        }
        y += 1;
    }

    let mouse_pos = ctx.get_mouse_position_for_current_layer();
    ctx.set_bg(mouse_pos.x, mouse_pos.y, RGB::named(MAGENTA));
}
