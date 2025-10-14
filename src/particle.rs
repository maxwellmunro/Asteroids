use std::f32::consts::PI;
use crate::constants;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::sys::SDL_GetTicks64;
use sdl2::video::Window;

#[derive(Copy, Clone)]
pub struct Particle {
    x: f32,
    y: f32,

    vx: f32,
    vy: f32,

    death: u64,
}

impl Particle {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Particle {
            x,
            y,
            vx,
            vy,
            death: unsafe { SDL_GetTicks64() }
                + rand::rng().random_range(
                    constants::particle::MIN_LIFESPAN..constants::particle::MAX_LIFESPAN,
                ),
        }
    }
    
    pub fn generate_explosion_particles(x: f32, y: f32) -> Vec<Particle> {
        let num_particles = rand::rng().random_range(constants::particle::explosion::COUNT_RANGE);

        (0..num_particles).map(|_| {
            let angle = rand::rng().random_range(0.0..(PI * 2.0));
            let vel = rand::rng().random_range(constants::particle::explosion::VEL_RANGE);
            
            Particle::new(x, y, vel * angle.cos(), vel * angle.sin())            
        }).collect::<Vec<Particle>>()
    }

    pub fn tick(&mut self, dt: f32, screen_bounds: Rect) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        if self.x < 0.0 {
            self.x = screen_bounds.width() as f32;
        }

        if self.x > screen_bounds.width() as f32 {
            self.x = 0.0;
        }

        if self.y < 0.0 {
            self.y = screen_bounds.height() as f32;
        }

        if self.y > screen_bounds.height() as f32 {
            self.y = 0.0;
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_point(Point::new(self.x as i32, self.y as i32))?;

        Ok(())
    }

    pub fn is_alive(&self) -> bool {
        self.death >= unsafe { SDL_GetTicks64() }
    }
}
