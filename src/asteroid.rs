use crate::constants;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f32::consts::PI;

pub struct Asteroid {
    x: f32,
    y: f32,

    vx: f32,
    vy: f32,

    radius: f32,

    shape: Vec<[f32; 2]>,
}

impl Asteroid {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        let points = (radius * constants::asteroid::POINTS_PER_RADIUS).max(3.0) as i32;

        let shape = (0..points)
            .map(|i| {
                let angle = i as f32 * PI * 2.0 / points as f32;
                let radius_fac = 1.0
                    + rand::rng().random_range(
                        -constants::asteroid::RADIUS_OFFSET_FAC
                            ..constants::asteroid::RADIUS_OFFSET_FAC,
                    );
                let point_radius = radius * radius_fac;
                [point_radius * angle.cos(), point_radius * angle.sin()]
            })
            .collect::<Vec<[f32; 2]>>();

        let vel = rand::rng().random_range(constants::asteroid::VEL_RANGE);
        let angle = rand::rng().random_range(0.0..PI * 2.0);

        Self {
            x,
            y,
            vx: vel * angle.cos(),
            vy: vel * angle.sin(),
            radius,
            shape,
        }
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

    pub fn render(&self, canvas: &mut Canvas<Window>, screen_bounds: Rect) -> Result<(), String> {
        let points = self
            .shape
            .iter()
            .map(|p| Point::new((p[0] + self.x) as i32, (p[1] + self.y) as i32))
            .collect::<Vec<Point>>();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        constants::window::WRAPPING_VALS.iter().try_for_each(|p| {
            let dx = p[0] * screen_bounds.width() as i32;
            let dy = p[1] * screen_bounds.height() as i32;

            let translated_points = points
                .iter()
                .map(|e| Point::new(e.x + dx, e.y + dy))
                .collect::<Vec<Point>>();

            canvas.draw_lines(translated_points.as_slice())?;
            canvas.draw_line(translated_points[0], *translated_points.last().unwrap())
        })?;

        Ok(())
    }

    pub fn get_spawn_location(px: f32, py: f32, screen_bounds: Rect) -> (f32, f32) {
        let mut counter = 0;
        let mut best_point = (0.0, 0.0);
        let mut max_sd: f32 = 0.0;

        while counter < constants::asteroid::MAX_SPAWN_ATTEMPTS
            && max_sd < constants::asteroid::MIN_SPAWN_DISTANCE
        {
            counter += 1;

            let x = rand::rng().random_range(0..screen_bounds.width()) as f32;
            let y = rand::rng().random_range(0..screen_bounds.height()) as f32;

            let dx = px - x;
            let dy = py - y;
            let sd = dx * dx + dy * dy;

            if sd > max_sd {
                max_sd = sd;
                best_point = (x, y);
            }
        }

        best_point
    }

    pub fn check_split(&self) -> Option<Vec<Asteroid>> {
        let r = self.radius / 2.0;

        if r < constants::asteroid::MIN_RADIUS {
            return None;
        }

        Some(vec![
            Asteroid::new(self.x, self.y, r),
            Asteroid::new(self.x, self.y, r),
        ])
    }

    pub fn get_hitboxes(&self, screen_bounds: Rect) -> Vec<Vec<(f32, f32)>> {
        constants::window::WRAPPING_VALS.iter().map(|v| {
            let dx = v[0] * screen_bounds.width() as i32;
            let dy = v[1] * screen_bounds.height() as i32;
            
            self.shape
                .iter()
                .map(|p| (p[0] + self.x + dx as f32, p[1] + self.y + dy as f32))
                .collect()
        }).collect::<Vec<Vec<(f32, f32)>>>()
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }
    
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}
