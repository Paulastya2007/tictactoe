use crate::config::*;
use macroquad::prelude::*;

pub struct ScreenScale {
    pub scale: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

pub fn calculate_scale() -> ScreenScale {
    let scale_x = screen_width() / VIRTUAL_WIDTH;
    let scale_y = screen_height() / VIRTUAL_HEIGHT;
    let scale = scale_x.min(scale_y);

    let offset_x = (screen_width() - VIRTUAL_WIDTH * scale) * 0.5;
    let offset_y = (screen_height() - VIRTUAL_HEIGHT * scale) * 0.5;

    ScreenScale {
        scale,
        offset_x,
        offset_y,
    }
}

pub fn mouse_to_virtual(scale: &ScreenScale) -> Vec2 {
    let (mx, my) = mouse_position();
    vec2(
        (mx - scale.offset_x) / scale.scale,
        (my - scale.offset_y) / scale.scale,
    )
}
