use macroquad::prelude::*;
/*
mod ai;
mod board;*/
mod config;
//mod game;
//mod menu;
mod state;
mod utils;

use config::*;
use state::GameState;
use utils::*;

#[macroquad::main("Tic Tac Toe")]
async fn main() {
    let mut game_state = GameState::Menu;

    loop {
        let scale = calculate_scale();

        clear_background(BG_COLOR);

        // Draw virtual bounds (debug)
        draw_rectangle_lines(
            scale.offset_x,
            scale.offset_y,
            VIRTUAL_WIDTH * scale.scale,
            VIRTUAL_HEIGHT * scale.scale,
            2.0,
            RED,
        );

        // Example text in virtual space
        draw_text_ex(
            "Virtual Resolution: 800 x 600",
            scale.offset_x + 20.0 * scale.scale,
            scale.offset_y + 40.0 * scale.scale,
            TextParams {
                font_size: (32.0 * scale.scale) as u16,
                color: TEXT_COLOR,
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
