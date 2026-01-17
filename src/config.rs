use macroquad::prelude::*;
use std::sync::OnceLock;

// =====================
// Font
// =====================
const FONT_DATA: &[u8] = include_bytes!("../assets/font/Kenney Future.ttf");

static GAME_FONT: OnceLock<Font> = OnceLock::new();

pub fn load_font() -> Font {
    load_ttf_font_from_bytes(FONT_DATA).expect("Failed to load embedded font")
}

pub fn get_font() -> Option<&'static Font> {
    GAME_FONT.get()
}

pub fn set_font(font: Font) {
    let _ = GAME_FONT.set(font);
}

// =====================
// Button Textures
// =====================
const BUTTON_BLUE_DATA: &[u8] =
    include_bytes!("../assets/things/Blue/Default/button_rectangle_depth_gradient.png");
const BUTTON_GREEN_DATA: &[u8] =
    include_bytes!("../assets/things/Green/Default/button_rectangle_depth_gradient.png");

static BUTTON_BLUE_TEX: OnceLock<Texture2D> = OnceLock::new();
static BUTTON_GREEN_TEX: OnceLock<Texture2D> = OnceLock::new();

pub fn load_button_textures() {
    let blue_tex = Texture2D::from_file_with_format(BUTTON_BLUE_DATA, None);
    let green_tex = Texture2D::from_file_with_format(BUTTON_GREEN_DATA, None);

    blue_tex.set_filter(FilterMode::Linear);
    green_tex.set_filter(FilterMode::Linear);

    let _ = BUTTON_BLUE_TEX.set(blue_tex);
    let _ = BUTTON_GREEN_TEX.set(green_tex);
}

pub fn get_button_blue() -> Option<&'static Texture2D> {
    BUTTON_BLUE_TEX.get()
}

pub fn get_button_green() -> Option<&'static Texture2D> {
    BUTTON_GREEN_TEX.get()
}

// =====================
// Virtual resolution
// =====================
pub const VIRTUAL_WIDTH: f32 = 800.0;
pub const VIRTUAL_HEIGHT: f32 = 600.0;

// =====================
// Window settings
// =====================
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 600;
pub const WINDOW_TITLE: &str = "Tic Tac Toe";
pub const WINDOW_RESIZABLE: bool = false;
pub const WINDOW_FULLSCREEN: bool = false;

// =====================
// Colors
// =====================
pub const BG_COLOR: Color = Color::from_rgba(245, 245, 245, 255);
pub const TEXT_COLOR: Color = Color::from_rgba(20, 20, 20, 255);

pub const PRIMARY_BLUE: Color = Color::from_rgba(52, 120, 246, 255);
pub const SUCCESS_GREEN: Color = Color::from_rgba(46, 204, 113, 255);
pub const WARNING_RED: Color = Color::from_rgba(231, 76, 60, 255);
pub const MUTED_GREY: Color = Color::from_rgba(120, 120, 120, 255);
pub const DARK_GREY: Color = Color::from_rgba(50, 50, 50, 255);
