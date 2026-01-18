use macroquad::prelude::*;
use std::sync::OnceLock;

// =====================
// Font
// =====================
const FONT_DATA: &[u8] = include_bytes!("../assets/font/Kenney Future.ttf");
const FONT_INTER_DATA: &[u8] = include_bytes!("../assets/font/inter18.ttf");

static GAME_FONT: OnceLock<Font> = OnceLock::new();
static INTER_FONT: OnceLock<Font> = OnceLock::new();

pub fn load_font() -> Font {
    load_ttf_font_from_bytes(FONT_DATA).expect("Failed to load embedded font")
}

pub fn load_inter_font() -> Font {
    load_ttf_font_from_bytes(FONT_INTER_DATA).expect("Failed to load embedded inter font")
}

pub fn get_font() -> Option<&'static Font> {
    GAME_FONT.get()
}

pub fn get_inter_font() -> Option<&'static Font> {
    INTER_FONT.get()
}

pub fn set_font(font: Font) {
    let _ = GAME_FONT.set(font);
}

pub fn set_inter_font(font: Font) {
    let _ = INTER_FONT.set(font);
}

// =====================
// Textures (Buttons)
// =====================
const BUTTON_BLUE_DATA: &[u8] =
    include_bytes!("../assets/things/Blue/Default/button_rectangle_depth_gradient.png");
const BUTTON_GREEN_DATA: &[u8] =
    include_bytes!("../assets/things/Green/Default/button_rectangle_depth_gradient.png");

static BUTTON_BLUE_TEX: OnceLock<Texture2D> = OnceLock::new();
static BUTTON_GREEN_TEX: OnceLock<Texture2D> = OnceLock::new();

// =====================
// Sounds
// =====================
const SOUND_CLICK_DATA: &[u8] = include_bytes!("../assets/Sounds/click-a.ogg");
const SOUND_MOVE_DATA: &[u8] = include_bytes!("../assets/Sounds/tap-b.ogg");
const SOUND_WIN_DATA: &[u8] = include_bytes!("../assets/Sounds/switch-a.ogg");

static SOUND_CLICK: OnceLock<macroquad::audio::Sound> = OnceLock::new();
static SOUND_MOVE: OnceLock<macroquad::audio::Sound> = OnceLock::new();
static SOUND_WIN: OnceLock<macroquad::audio::Sound> = OnceLock::new();

pub fn load_assets() {
    let blue_tex = Texture2D::from_file_with_format(BUTTON_BLUE_DATA, None);
    let green_tex = Texture2D::from_file_with_format(BUTTON_GREEN_DATA, None);

    blue_tex.set_filter(FilterMode::Linear);
    green_tex.set_filter(FilterMode::Linear);

    let _ = BUTTON_BLUE_TEX.set(blue_tex);
    let _ = BUTTON_GREEN_TEX.set(green_tex);
}

pub async fn load_sounds() {
    if let Ok(sound) = macroquad::audio::load_sound_from_bytes(SOUND_CLICK_DATA).await {
        let _ = SOUND_CLICK.set(sound);
    }
    if let Ok(sound) = macroquad::audio::load_sound_from_bytes(SOUND_MOVE_DATA).await {
        let _ = SOUND_MOVE.set(sound);
    }
    if let Ok(sound) = macroquad::audio::load_sound_from_bytes(SOUND_WIN_DATA).await {
        let _ = SOUND_WIN.set(sound);
    }
}

pub fn play_click() {
    if let Some(sound) = SOUND_CLICK.get() {
        macroquad::audio::play_sound(
            sound,
            macroquad::audio::PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }
}

pub fn play_move() {
    if let Some(sound) = SOUND_MOVE.get() {
        macroquad::audio::play_sound(
            sound,
            macroquad::audio::PlaySoundParams {
                looped: false,
                volume: 0.8,
            },
        );
    }
}

pub fn play_win() {
    if let Some(sound) = SOUND_WIN.get() {
        macroquad::audio::play_sound(
            sound,
            macroquad::audio::PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }
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
pub const WINDOW_TITLE: &str = "Tic Tac Toe Rust Edition";
pub const WINDOW_RESIZABLE: bool = false;
pub const WINDOW_FULLSCREEN: bool = false;

// =====================
// Colors
// =====================
pub const BG_COLOR: Color = Color::from_rgba(245, 245, 245, 255);
pub const TEXT_COLOR: Color = Color::from_rgba(20, 20, 20, 255);

pub const PRIMARY_BLUE: Color = Color::from_rgba(52, 120, 246, 255);
pub const SUCCESS_GREEN: Color = Color::from_rgba(46, 204, 113, 255);
pub const MUTED_GREY: Color = Color::from_rgba(120, 120, 120, 255);
pub const DARK_GREY: Color = Color::from_rgba(50, 50, 50, 255);
