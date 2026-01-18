use crate::config::*;
use crate::state::GameState;
use crate::utils::*;
use macroquad::prelude::*;

pub fn update(scale: &ScreenScale) -> Option<GameState> {
    let button_width = 280.0;
    let button_height = 70.0;

    let center_x = VIRTUAL_WIDTH / 2.0 - button_width / 2.0;

    // Draw Background Decorations (X and O icons using Font for sharpness)
    let decoration_color = Color::new(0.0, 0.0, 0.0, 0.05); // Very faint grey
    let font = crate::config::get_font();

    if let Some(f) = font {
        // Top Left X
        draw_text_ex(
            "X",
            50.0,
            180.0,
            TextParams {
                font: Some(f),
                font_size: 150,
                color: decoration_color,
                ..Default::default()
            },
        );
        // Bottom Right X
        draw_text_ex(
            "X",
            VIRTUAL_WIDTH - 200.0,
            VIRTUAL_HEIGHT - 50.0,
            TextParams {
                font: Some(f),
                font_size: 180,
                color: decoration_color,
                ..Default::default()
            },
        );

        // Top Right O
        draw_text_ex(
            "O",
            VIRTUAL_WIDTH - 250.0,
            200.0,
            TextParams {
                font: Some(f),
                font_size: 160,
                color: decoration_color,
                ..Default::default()
            },
        );
        // Bottom Left O
        draw_text_ex(
            "O",
            80.0,
            VIRTUAL_HEIGHT - 80.0,
            TextParams {
                font: Some(f),
                font_size: 140,
                color: decoration_color,
                ..Default::default()
            },
        );
    }

    // Draw Title
    let title_text = "TIC TAC TOE";
    let title_size = 64;
    let title_dim = measure_text(title_text, font, title_size, 1.0);
    draw_text_ex(
        title_text,
        VIRTUAL_WIDTH / 2.0 - title_dim.width / 2.0,
        120.0,
        TextParams {
            font,
            font_size: title_size,
            color: DARK_GREY,
            ..Default::default()
        },
    );

    let start_y = 250.0;
    let mouse = mouse_to_virtual(scale);

    // ---- PvP Button ----
    let pvp_rect = Rect::new(center_x, start_y, button_width, button_height);
    draw_button(pvp_rect, "Player vs Player", ButtonType::Blue, mouse);

    // ---- PvAI Button ----
    let ai_rect = Rect::new(center_x, start_y + 100.0, button_width, button_height);
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
