use crate::config::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};
use macroquad::prelude::*;

pub struct Background {
    time: f32,
}

impl Background {
    pub fn new() -> Self {
        Self { time: 0.0 }
    }

    pub fn update(&mut self) {
        self.time += get_frame_time();
    }

    pub fn draw(&self) {
        // Simple background color
        draw_rectangle(
            0.0,
            0.0,
            VIRTUAL_WIDTH,
            VIRTUAL_HEIGHT,
            Color::new(0.96, 0.96, 0.98, 1.0),
        );

        // Subtle moving glows
        let center_x = VIRTUAL_WIDTH / 2.0;
        let center_y = VIRTUAL_HEIGHT / 2.0;

        let glow_count = 5;
        for i in 0..glow_count {
            let offset = i as f32 * (std::f32::consts::PI * 2.0 / glow_count as f32);
            let glow_x = center_x + (self.time * 0.2 + offset).cos() * 200.0;
            let glow_y = center_y + (self.time * 0.15 + offset).sin() * 150.0;

            let color = if i % 2 == 0 {
                Color::new(0.2, 0.5, 1.0, 0.03) // Blueish
            } else {
                Color::new(0.2, 1.0, 0.5, 0.02) // Greenish
            };

            draw_circle(
                glow_x,
                glow_y,
                250.0 + (self.time + offset).sin() * 50.0,
                color,
            );
        }
    }
}
