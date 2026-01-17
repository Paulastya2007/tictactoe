use crate::config::*;
use crate::state::GameState;
use crate::utils::*;
use macroquad::prelude::*;

pub fn update(mode: GameState, scale: &ScreenScale) -> Option<GameState> {
    let _mouse = mouse_to_virtual(scale);

    let font = crate::config::get_font();

    draw_text_ex(
        match mode {
            GameState::PvP => "PvP Mode",
            GameState::PvAI => "PvAI Mode",
            _ => "",
        },
        20.0,
        40.0,
        TextParams {
            font: font,
            font_size: 24,
            color: TEXT_COLOR,
            ..Default::default()
        },
    );

    draw_text_ex(
        "Press ESC to return to Menu",
        20.0,
        80.0,
        TextParams {
            font: font,
            font_size: 18,
            color: MUTED_GREY,
            ..Default::default()
        },
    );

    if is_key_pressed(KeyCode::Escape) {
        return Some(GameState::Menu);
    }

    None
}
