use crate::config::*;
use crate::state::GameState;
use crate::utils::*;
use macroquad::prelude::*;

pub fn update(scale: &ScreenScale) -> Option<GameState> {
    let button_width = 260.0;
    let button_height = 60.0;

    let center_x = VIRTUAL_WIDTH / 2.0 - button_width / 2.0;
    let start_y = 220.0;

    let mouse = mouse_to_virtual(scale);

    // ---- PvP Button ----
    let pvp_rect = Rect::new(center_x, start_y, button_width, button_height);
    draw_button(pvp_rect, "Play vs Player", ButtonType::Blue, mouse);

    // ---- PvAI Button ----
    let ai_rect = Rect::new(center_x, start_y + 90.0, button_width, button_height);
    draw_button(ai_rect, "Play vs AI", ButtonType::Green, mouse);

    if is_mouse_button_pressed(MouseButton::Left) {
        if pvp_rect.contains(mouse) {
            return Some(GameState::PvP);
        }
        if ai_rect.contains(mouse) {
            return Some(GameState::PvAI);
        }
    }

    None
}

enum ButtonType {
    Blue,
    Green,
}

fn draw_button(rect: Rect, text: &str, button_type: ButtonType, mouse: Vec2) {
    let hover = rect.contains(mouse);

    // Get the appropriate texture
    let texture = match button_type {
        ButtonType::Blue => crate::config::get_button_blue(),
        ButtonType::Green => crate::config::get_button_green(),
    };

    if let Some(tex) = texture {
        // Draw the button texture with hover effect
        let alpha = if hover { 0.85 } else { 1.0 };
        draw_texture_ex(
            tex,
            rect.x,
            rect.y,
            Color::new(1.0, 1.0, 1.0, alpha),
            DrawTextureParams {
                dest_size: Some(vec2(rect.w, rect.h)),
                ..Default::default()
            },
        );
    }

    // Draw text
    let font_size = 22.0;
    let font = crate::config::get_font();
    let text_params = TextParams {
        font: font,
        font_size: font_size as u16,
        color: WHITE,
        ..Default::default()
    };
    let text_dim = measure_text(text, font, font_size as u16, 1.0);

    draw_text_ex(
        text,
        rect.x + rect.w / 2.0 - text_dim.width / 2.0,
        rect.y + rect.h / 2.0 + text_dim.height / 2.0 - 4.0,
        text_params,
    );
}
