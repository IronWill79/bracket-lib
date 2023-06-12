use bevy::prelude::{Query, Res, With};
use bracket_bevy::BracketContext;
use bracket_color::{
    prelude::{BLACK, GREY, MAGENTA, RED, WHITE, YELLOW},
    rgb::RGB,
};
use bracket_geometry::prelude::Point;

use crate::{CombatStats, GameLog, Map, Player, Position, Renderable};

pub fn draw_ui(
    ctx: &BracketContext,
    player_stats_query: Query<&CombatStats, With<Player>>,
    log: Res<GameLog>,
    map: Res<Map>,
    renderables_query: Query<(&Position, &Renderable, &crate::components::Name)>,
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

    draw_tooltips(ctx, map, renderables_query);
}

pub fn draw_tooltips(
    ctx: &BracketContext,
    map: Res<Map>,
    renderables_query: Query<(&Position, &Renderable, &crate::components::Name)>,
) {
    let mouse_pos = ctx.get_mouse_position_for_current_layer();
    if mouse_pos.x >= map.width || mouse_pos.y >= map.height {
        return;
    }
    let mut tooltip: Vec<String> = Vec::new();
    for (position, _r, name) in renderables_query.iter() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.x && position.y == mouse_pos.y && map.visible_tiles[idx] {
            tooltip.push(name.name.to_string());
        }
    }

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 {
                width = s.len() as i32;
            }
        }
        width += 3;

        if mouse_pos.x > 40 {
            let arrow_pos = Point::new(mouse_pos.x - 2, mouse_pos.y);
            let left_x = mouse_pos.x - width;
            let mut y = mouse_pos.y;
            for s in tooltip.iter() {
                ctx.print_color(left_x, y, s, RGB::named(WHITE), RGB::named(GREY));
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x - i,
                        y,
                        &" ".to_string(),
                        RGB::named(WHITE),
                        RGB::named(GREY),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                &"->".to_string(),
                RGB::named(WHITE),
                RGB::named(GREY),
            );
        } else {
            let arrow_pos = Point::new(mouse_pos.x + 1, mouse_pos.y);
            let left_x = mouse_pos.x + 3;
            let mut y = mouse_pos.y;
            for s in tooltip.iter() {
                ctx.print_color(left_x + 1, y, s, RGB::named(WHITE), RGB::named(GREY));
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x + 1 + i,
                        y,
                        &" ".to_string(),
                        RGB::named(WHITE),
                        RGB::named(GREY),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                &"<-".to_string(),
                RGB::named(WHITE),
                RGB::named(GREY),
            );
        }
    }
}
