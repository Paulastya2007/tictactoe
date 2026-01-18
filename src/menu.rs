use crate::config::*;
use crate::state::GameState;
use crate::utils::*;
use macroquad::prelude::*;

pub fn update(scale: &ScreenScale) -> Option<GameState> {
    draw_decorations();

    let button_width = 280.0;
    let button_height = 70.0;
    let center_x = VIRTUAL_WIDTH / 2.0 - button_width / 2.0;
    let start_y = 250.0;
    let mouse = mouse_to_virtual(scale);
    let font = crate::config::get_font();
    let theme = crate::theme::get_current_theme();

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
            color: theme.text,
            ..Default::default()
        },
    );

    // Theme Instructions
    let theme_instr = format!("Theme: {} (Press T)", theme.name);
    let instr_size = 20;
    let instr_dim = measure_text(&theme_instr, font, instr_size, 1.0);
    draw_text_ex(
        &theme_instr,
        VIRTUAL_WIDTH / 2.0 - instr_dim.width / 2.0,
        160.0,
        TextParams {
            font,
            font_size: instr_size,
            color: theme.muted,
            ..Default::default()
        },
    );

    // ---- PvP Button ----
    let pvp_rect = Rect::new(center_x, start_y, button_width, button_height);
    draw_button(pvp_rect, "Player vs Player", ButtonType::Blue, mouse, None);

    // ---- PvAI Button ----
    let ai_rect = Rect::new(center_x, start_y + 100.0, button_width, button_height);
    draw_button(ai_rect, "Play vs AI", ButtonType::Green, mouse, None);

    if is_mouse_button_pressed(MouseButton::Left) {
        if pvp_rect.contains(mouse) {
            crate::config::play_click();
            return Some(GameState::PvP);
        }
        if ai_rect.contains(mouse) {
            crate::config::play_click();
            return Some(GameState::ChooseSymbol);
        }
    }

    if is_key_pressed(KeyCode::T) {
        crate::theme::cycle_theme();
    }

    None
}

pub fn choose_symbol(scale: &ScreenScale) -> Option<GameState> {
    draw_decorations();

    let button_width = 180.0;
    let button_height = 180.0;
    let center_y = VIRTUAL_HEIGHT / 2.0 - button_height / 2.0 + 50.0;
    let spacing = 100.0;
    let start_x = VIRTUAL_WIDTH / 2.0 - button_width - spacing / 2.0;

    let mouse = mouse_to_virtual(scale);
    let font = crate::config::get_font();
    let inter_font = crate::config::get_inter_font();
    let theme = crate::theme::get_current_theme();

    // Draw Title
    let title_text = "CHOOSE YOUR SIDE";
    let title_size = 48;
    let title_dim = measure_text(title_text, font, title_size, 1.0);
    draw_text_ex(
        title_text,
        VIRTUAL_WIDTH / 2.0 - title_dim.width / 2.0,
        150.0,
        TextParams {
            font,
            font_size: title_size,
            color: theme.text,
            ..Default::default()
        },
    );

    // ---- Choose X Button ----
    let x_rect = Rect::new(start_x, center_y, button_width, button_height);
    draw_button(x_rect, "X", ButtonType::Blue, mouse, inter_font);

    // ---- Choose O Button ----
    let o_rect = Rect::new(
        start_x + button_width + spacing,
        center_y,
        button_width,
        button_height,
    );
    draw_button(o_rect, "O", ButtonType::Green, mouse, inter_font);

    if is_mouse_button_pressed(MouseButton::Left) {
        if x_rect.contains(mouse) {
            crate::config::play_click();
            crate::game::set_player_symbol(crate::board::CellState::X);
            return Some(GameState::PvAI);
        }
        if o_rect.contains(mouse) {
            crate::config::play_click();
            crate::game::set_player_symbol(crate::board::CellState::O);
            return Some(GameState::PvAI);
        }
    }

    if is_key_pressed(KeyCode::Escape) {
        return Some(GameState::Menu);
    }

    None
}

fn draw_decorations() {
    // Draw Background Decorations (X and O icons using Font for sharpness)
    let theme = crate::theme::get_current_theme();
    let mut decoration_color = theme.muted;
    decoration_color.a = 0.05; // Very faint

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
}

enum ButtonType {
    Blue,
    Green,
}

fn draw_button(
    rect: Rect,
    text: &str,
    button_type: ButtonType,
    mouse: Vec2,
    override_font: Option<&Font>,
) {
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
    let font = override_font.or(crate::config::get_font());
    let font_size = if override_font.is_some() { 100.0 } else { 22.0 };

    let text_params = TextParams {
        font,
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
