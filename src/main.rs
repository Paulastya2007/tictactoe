#![windows_subsystem = "windows"]

use macroquad::prelude::*;

mod ai;
mod background;
mod board;
mod config;
mod game;
mod menu;
mod particles;
mod state;
mod utils;

use config::*;
use state::GameState;
use utils::*;

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: WINDOW_RESIZABLE,
        fullscreen: WINDOW_FULLSCREEN,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Load and set the fonts (embedded in binary)
    config::set_font(config::load_font());
    config::set_inter_font(config::load_inter_font());

    // Load assets (embedded in binary)
    config::load_assets();
    config::load_sounds().await;

    let mut game_state = GameState::Menu;
    let mut bg = background::Background::new();

    loop {
        let scale = calculate_scale();
        let dt = get_frame_time();

        clear_background(BG_COLOR);
        bg.update();

        // Scale drawing
        let camera = Camera2D {
            target: vec2(VIRTUAL_WIDTH / 2.0, VIRTUAL_HEIGHT / 2.0),
            zoom: vec2(2.0 / VIRTUAL_WIDTH, 2.0 / VIRTUAL_HEIGHT) * scale.scale,
            offset: vec2(scale.offset_x, scale.offset_y),
            ..Default::default()
        };

        set_camera(&camera);

        // Draw background
        bg.draw();

        // ---- State machine ----
        let next_state = match game_state {
            GameState::Menu => menu::update(&scale),
            GameState::ChooseSymbol => menu::choose_symbol(&scale),
            GameState::PvP | GameState::PvAI => game::update(game_state, &scale),
        };

        if let Some(state) = next_state {
            game_state = state;
        }

        // Draw and update particles
        particles::update_and_draw(dt);

        set_default_camera();
        next_frame().await;
    }
}
