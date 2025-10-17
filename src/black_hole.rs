use crate::constants;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::sys::SDL_GetTicks64;
use sdl2::video::Window;
use std::f32::consts::PI;

pub struct BlackHole {
    x: f32,
    y: f32,

    angle: f32,

    max_radius: f32,
    radius: f32,
    expanding: bool,
    shrink_time: u64,
}

impl BlackHole {
    pub fn new(screen_bounds: Rect) -> Self {
        let x = rand::rng().random_range(0..screen_bounds.width());
        let y = rand::rng().random_range(0..screen_bounds.height());

        let max_radius = rand::rng().random_range(constants::black_hole::MAX_RADII_RANGE);
        let max_time = rand::rng().random_range(constants::black_hole::MAX_TIME_RANGE);

        let shrink_time = unsafe { SDL_GetTicks64() }
            + (1000.0 * max_radius / constants::black_hole::GROWTH_RATE) as u64
            + max_time;

        BlackHole {
            x: x as f32,
            y: y as f32,

            angle: 0.0,

            max_radius,

            radius: 0.0,

            expanding: true,
            shrink_time,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        if self.expanding {
            self.radius += constants::black_hole::GROWTH_RATE * dt;
        } else if unsafe { SDL_GetTicks64() } >= self.shrink_time {
            self.radius -= constants::black_hole::SHRINK_RATE * dt;
        }

        self.radius = self.radius.min(self.max_radius);

        self.angle += constants::black_hole::ROT_RATE * dt;
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, screen_bounds: Rect) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        constants::window::WRAPPING_VALS.iter().try_for_each(|p| {
            let dx = p[0] * screen_bounds.width() as i32;
            let dy = p[1] * screen_bounds.height() as i32;

            for i in 0..constants::black_hole::LINES {
                let line_angle = i as f32 * PI * 2.0 / constants::black_hole::LINES as f32;

                for j in 1..constants::black_hole::LINE_RES {
                    let last_angle = line_angle
                        + ((j - 1) as f32 * constants::black_hole::LINE_ROT
                        / constants::black_hole::LINE_RES as f32)
                        + self.angle;

                    let next_angle = line_angle
                        + (j as f32 * constants::black_hole::LINE_ROT
                        / constants::black_hole::LINE_RES as f32)
                        + self.angle;

                    let last_dist =
                        (j - 1) as f32 * self.radius / constants::black_hole::LINE_RES as f32;

                    let next_dist = j as f32 * self.radius / constants::black_hole::LINE_RES as f32;

                    let p1 = Point::new(
                        (self.x + last_dist * last_angle.cos()) as i32 + dx,
                        (self.y + last_dist * last_angle.sin()) as i32 + dy,
                    );

                    let p2 = Point::new(
                        (self.x + next_dist * next_angle.cos()) as i32 + dx,
                        (self.y + next_dist * next_angle.sin()) as i32 + dy,
                    );

                    canvas.draw_line(p1, p2)?;
                }
            }

            Ok::<(), String>(())
        })?;

        Ok(())
    }

    pub fn get_force(&self, x: f32, y: f32, dt: f32) -> (f32, f32){
        let dx = self.x - x;
        let dy = self.y - y;
        let d = (dx * dx + dy * dy).sqrt();

        let range = constants::black_hole::RANGE_FAC * self.radius;

        if d <= range {
            let pull = self.radius * constants::black_hole::FORCE_FAC / (d * d);
            let fac = dt * pull / d;

            return (fac * dx, fac * dy);
        }

        (0.0, 0.0)
    }

    pub fn is_alive(&self) -> bool {
        self.radius >= 0.0
    }
}
