use bracket_bevy::BracketContext;
use bracket_color::{
    prelude::{BLACK, WHITE},
    rgb::RGB,
};

pub fn draw_ui(ctx: &BracketContext) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(WHITE), RGB::named(BLACK));
}
