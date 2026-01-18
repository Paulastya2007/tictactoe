use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ThemeType {
    Modern,
    Neon,
    Royal,
    Terminal,
}

pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub text: Color,
    pub primary: Color,   // X
    pub secondary: Color, // O
    pub muted: Color,
    pub accent: Color, // Grid
    pub glow_colors: [Color; 2],
}

impl Theme {
    pub fn get(t: ThemeType) -> Self {
        match t {
            ThemeType::Modern => Self {
                name: "Modern",
                bg: Color::from_rgba(245, 245, 245, 255),
                text: Color::from_rgba(20, 20, 20, 255),
                primary: Color::from_rgba(52, 120, 246, 255), // Blue
                secondary: Color::from_rgba(46, 204, 113, 255), // Green
                muted: Color::from_rgba(120, 120, 120, 255),
                accent: Color::from_rgba(50, 50, 50, 255), // Dark Grey
                glow_colors: [
                    Color::new(0.2, 0.5, 1.0, 0.03),
                    Color::new(0.2, 1.0, 0.5, 0.02),
                ],
            },
            ThemeType::Neon => Self {
                name: "Neon Night",
                bg: Color::from_rgba(10, 10, 20, 255),
                text: Color::from_rgba(240, 240, 255, 255),
                primary: Color::from_rgba(255, 0, 255, 255), // Pink
                secondary: Color::from_rgba(0, 255, 255, 255), // Cyan
                muted: Color::from_rgba(100, 100, 150, 255),
                accent: Color::from_rgba(100, 100, 255, 150),
                glow_colors: [
                    Color::new(1.0, 0.0, 1.0, 0.04),
                    Color::new(0.0, 1.0, 1.0, 0.04),
                ],
            },
            ThemeType::Royal => Self {
                name: "Royal Gold",
                bg: Color::from_rgba(20, 30, 60, 255), // Dark Blue
                text: Color::from_rgba(255, 240, 200, 255),
                primary: Color::from_rgba(255, 215, 0, 255), // Gold
                secondary: Color::from_rgba(220, 50, 50, 255), // Crimson
                muted: Color::from_rgba(150, 130, 100, 255),
                accent: Color::from_rgba(200, 180, 120, 100),
                glow_colors: [
                    Color::new(1.0, 0.84, 0.0, 0.05),
                    Color::new(0.86, 0.08, 0.24, 0.04),
                ],
            },
            ThemeType::Terminal => Self {
                name: "Terminal",
                bg: Color::from_rgba(5, 15, 5, 255),
                text: Color::from_rgba(0, 255, 0, 255),
                primary: Color::from_rgba(50, 255, 50, 255),
                secondary: Color::from_rgba(0, 200, 0, 255),
                muted: Color::from_rgba(20, 100, 20, 255),
                accent: Color::from_rgba(0, 120, 0, 80),
                glow_colors: [
                    Color::new(0.0, 1.0, 0.0, 0.02),
                    Color::new(0.0, 0.8, 0.0, 0.01),
                ],
            },
        }
    }
}

static mut CURRENT_THEME_TYPE: ThemeType = ThemeType::Modern;

pub fn get_current_theme() -> Theme {
    unsafe { Theme::get(CURRENT_THEME_TYPE) }
}

pub fn cycle_theme() {
    unsafe {
        CURRENT_THEME_TYPE = match CURRENT_THEME_TYPE {
            ThemeType::Modern => ThemeType::Neon,
            ThemeType::Neon => ThemeType::Royal,
            ThemeType::Royal => ThemeType::Terminal,
            ThemeType::Terminal => ThemeType::Modern,
        };
    }
}
