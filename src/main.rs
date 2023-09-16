pub mod box_data;
pub mod constants;
pub mod enums;
pub mod event;
pub mod render;

use crate::render::*;
use bracket_lib::prelude::*;
use constants::*;

embedded_resource!(TILE_FONT, "../resources/terminal_10x16.png");

fn main() -> BError {
    link_resource!(TILE_FONT, "resources/terminal_10x16.png");

    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_title("Klotski Game")
        .with_font("terminal_10x16.png", FONT_WIDTH, FONT_HEIGHT)
        .build()?;

    main_loop(context, State::new(DEFAULT_BLOCK_SIZE))
}
