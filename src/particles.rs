use macroquad::prelude::*;

pub struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: Color,
    life: f32, // 1.0 to 0.0
    size: f32,
}

struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    const fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    fn spawn_move_effect(&mut self, pos: Vec2, color: Color) {
        for _ in 0..15 {
            let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
            let speed = rand::gen_range(50.0, 150.0);
            let vel = vec2(angle.cos() * speed, angle.sin() * speed);

            self.particles.push(Particle {
                pos,
                vel,
                color,
                life: 1.0,
                size: rand::gen_range(3.0, 6.0),
            });
        }
    }

    fn spawn_win_effect(&mut self, pos: Vec2, color: Color) {
        for _ in 0..60 {
            let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
            let speed = rand::gen_range(100.0, 400.0);
            let vel = vec2(angle.cos() * speed, angle.sin() * speed);

            self.particles.push(Particle {
                pos,
                vel,
                color,
                life: 1.0,
                size: rand::gen_range(4.0, 10.0),
            });
        }
    }

    fn update(&mut self, dt: f32) {
        for p in self.particles.iter_mut() {
            p.pos += p.vel * dt;
            p.vel *= 0.96; // Drag
            p.life -= dt * 1.5;
            p.size *= 0.98; // Shrink
        }

        self.particles.retain(|p| p.life > 0.0);
    }

    fn draw(&self) {
        for p in &self.particles {
            let mut color = p.color;
            color.a = p.life;
            draw_circle(p.pos.x, p.pos.y, p.size, color);
        }
    }
}

static mut SYSTEM: ParticleSystem = ParticleSystem::new();

#[allow(static_mut_refs)]
pub fn spawn_move(pos: Vec2, color: Color) {
    unsafe {
        SYSTEM.spawn_move_effect(pos, color);
    }
}

#[allow(static_mut_refs)]
pub fn spawn_win(pos: Vec2, color: Color) {
    unsafe {
        SYSTEM.spawn_win_effect(pos, color);
    }
}

#[allow(static_mut_refs)]
pub fn update_and_draw(dt: f32) {
    unsafe {
        SYSTEM.update(dt);
        SYSTEM.draw();
    }
}
