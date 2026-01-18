use crate::config::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};
use macroquad::prelude::*;

pub struct Background {
    time: f32,
    dim_timer: f32,
}

impl Background {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            dim_timer: 0.0,
        }
    }

    pub fn update(&mut self, is_game_over: bool) {
        let dt = get_frame_time();
        self.time += dt;

        if is_game_over {
            self.dim_timer = (self.dim_timer + dt * 2.0).min(1.0);
        } else {
            self.dim_timer = (self.dim_timer - dt * 2.0).max(0.0);
        }
    }

    pub fn draw(&self) {
        let theme = crate::theme::get_current_theme();

        // Draw a subtle animated gradient background
        draw_rectangle(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT, theme.bg);

        // Subtle moving glows from theme
        let center_x = VIRTUAL_WIDTH / 2.0;
        let center_y = VIRTUAL_HEIGHT / 2.0;

        for i in 0..5 {
            let offset = i as f32 * (std::f32::consts::PI * 2.0 / 5.0);
            let glow_x = center_x + (self.time * 0.2 + offset).cos() * 200.0;
            let glow_y = center_y + (self.time * 0.15 + offset).sin() * 150.0;

            let mut color = theme.glow_colors[i % 2];
            // Fade glows when game ends
            color.a *= 1.0 - self.dim_timer * 0.5;

            draw_circle(
                glow_x,
                glow_y,
                250.0 + (self.time + offset).sin() * 50.0,
                color,
            );
        }

        // Draw a dim overlay when game is over
        if self.dim_timer > 0.0 {
            draw_rectangle(
                0.0,
                0.0,
                VIRTUAL_WIDTH,
                VIRTUAL_HEIGHT,
                Color::new(0.0, 0.0, 0.0, self.dim_timer * 0.3),
            );
        }
    }
}
