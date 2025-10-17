use crate::bullet::Bullet;
use crate::constants;
use crate::particle::Particle;
use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::sys::SDL_GetTicks64;
use sdl2::video::Window;
use std::f32::consts::PI;

pub struct Player {
    x: f32,
    y: f32,

    vx: f32,
    vy: f32,

    angle: f32,

    left: bool,
    right: bool,
    up: bool,

    particles_to_spawn: Vec<Particle>,

    last_thrust_particle: u64,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,

            vx: 0.0,
            vy: 0.0,

            angle: -PI / 2.0,

            left: false,
            right: false,
            up: false,

            particles_to_spawn: Vec::new(),
            last_thrust_particle: 0,
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

        if self.up {
            self.vx += constants::player::ACCELERATION * self.angle.cos() * dt;
            self.vy += constants::player::ACCELERATION * self.angle.sin() * dt;

            let fps = 1000.0 / dt;

            if constants::particle::thrust::PARTICLES_PER_SECOND > fps {
                let num_particles =
                    (constants::particle::thrust::PARTICLES_PER_SECOND / fps) as i32;

                (0..num_particles).for_each(|_| {
                    let vel = -rand::rng().random_range(constants::particle::thrust::VEL_RANGE);
                    let angle = self.angle
                        + rand::rng().random_range(
                            -constants::particle::thrust::ANGLE_OFFSET
                                ..constants::particle::thrust::ANGLE_OFFSET,
                        );

                    let vx = vel * angle.cos() + self.vx;
                    let vy = vel * angle.sin() + self.vy;

                    self.particles_to_spawn
                        .push(Particle::new(self.x, self.y, vx, vy))
                });

                self.last_thrust_particle = unsafe { SDL_GetTicks64() };
            } else {
                let ms_per_particle =
                    (1000.0 / constants::particle::thrust::PARTICLES_PER_SECOND) as u64;

                if self.last_thrust_particle + ms_per_particle < unsafe { SDL_GetTicks64() } {
                    self.last_thrust_particle = unsafe { SDL_GetTicks64() };
                    let vel = -rand::rng().random_range(constants::particle::thrust::VEL_RANGE);
                    let angle = self.angle
                        + rand::rng().random_range(
                            -constants::particle::thrust::ANGLE_OFFSET
                                ..constants::particle::thrust::ANGLE_OFFSET,
                        );

                    let vx = vel * angle.cos() + self.vx;
                    let vy = vel * angle.sin() + self.vy;

                    self.particles_to_spawn
                        .push(Particle::new(self.x, self.y, vx, vy));
                }
            }
        }

        self.vx *= constants::player::DECELERATION.powf(dt);
        self.vy *= constants::player::DECELERATION.powf(dt);

        if self.left == self.right {
            return;
        }

        if self.left {
            self.angle -= constants::player::TURN_SPEED * dt;
        }

        if self.right {
            self.angle += constants::player::TURN_SPEED * dt;
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, screen_bounds: Rect) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let points: Vec<Point> = constants::player::PLAYER_SHAPE
            .iter()
            .map(|p| {
                let angle = p[1].atan2(p[0]) + self.angle + PI / 2.0;
                let dist = (p[0] * p[0] + p[1] * p[1]).sqrt();

                let x = (dist * angle.cos() + self.x) as i32;
                let y = (dist * angle.sin() + self.y) as i32;

                Point::new(x, y)
            })
            .collect();

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

    pub fn handle_key_event(&mut self, key: Keycode, pressed: bool) {
        match key {
            Keycode::Left => self.left = pressed,
            Keycode::Right => self.right = pressed,
            Keycode::Up => self.up = pressed,
            _ => {}
        }
    }

    pub fn get_particles(&mut self) -> Vec<Particle> {
        let particles = self.particles_to_spawn.clone();
        self.particles_to_spawn.clear();
        particles
    }

    pub fn shoot_bullet(&self) -> Bullet {
        let x = self.x - constants::player::PLAYER_SHAPE[0][1] * self.angle.cos();
        let y = self.y - constants::player::PLAYER_SHAPE[0][1] * self.angle.sin();
        Bullet::new(x, y, self.angle, true)
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_hitbox(&self) -> Vec<(f32, f32)> {
        constants::player::PLAYER_SHAPE
            .iter()
            .map(|p| {
                let angle = p[1].atan2(p[0]) + self.angle + PI / 2.0;
                let dist = (p[0] * p[0] + p[1] * p[1]).sqrt();

                let x = dist * angle.cos() + self.x;
                let y = dist * angle.sin() + self.y;

                (x, y)
            })
            .collect::<Vec<(f32, f32)>>()
    }

    pub fn die(&mut self, screen_bounds: Rect) {
        self.x = screen_bounds.width() as f32 / 2.0;
        self.y = screen_bounds.height() as f32 / 2.0;

        self.vx = 0.0;
        self.vy = 0.0;

        self.angle = -PI / 2.0;
    }

    pub fn get_pos_and_vel(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.vx, self.vy)
    }

    pub fn set_location(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}
