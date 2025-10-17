use crate::bullet::Bullet;
use crate::constants;
use rand::Rng;
use rand::seq::IndexedRandom;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::sys::SDL_GetTicks64;
use sdl2::video::Window;
use std::f32::consts::PI;

#[derive(Clone)]
pub enum ShootingType {
    Random,
    Current,
    Future,
}

pub struct Alien {
    x: f32,
    y: f32,

    vx: f32,
    vy: f32,

    shooting_type: ShootingType,
    next_shot: u64,

    bullet: Option<Bullet>,
}

impl Alien {
    pub fn new(score: u64, screen_bounds: Rect) -> Option<Alien> {
        let mut available_types: Vec<ShootingType> = Vec::new();

        if score >= constants::alien::random::MIN_POINTS {
            available_types.push(ShootingType::Random);
        }

        if score >= constants::alien::current::MIN_POINTS {
            available_types.push(ShootingType::Current);
        }

        if score >= constants::alien::future::MIN_POINTS {
            available_types.push(ShootingType::Future);
        }

        let shooting_type = available_types.choose(&mut rand::rng());

        if shooting_type.is_none() {
            return None;
        }

        let shooting_type = shooting_type.unwrap();

        let side = rand::rng().random_range(0..4_u8);

        let (mut x, mut y): (f32, f32) = (0.0, 0.0);
        let (mut vx, mut vy): (f32, f32) = (0.0, 0.0);

        match side {
            0 => {
                x = rand::rng().random_range(0..screen_bounds.width()) as f32;
                y = 0.0;
                vy = 1.0;
            }
            1 => {
                x = 0.0;
                y = rand::rng().random_range(0..screen_bounds.height()) as f32;
                vx = 1.0;
            }
            2 => {
                x = rand::rng().random_range(0..screen_bounds.width()) as f32;
                y = screen_bounds.height() as f32;
                vy = -1.0;
            }
            3 => {
                x = screen_bounds.width() as f32;
                y = rand::rng().random_range(0..screen_bounds.height()) as f32;
                vx = -1.0;
            }
            _ => {}
        }

        Some(match shooting_type {
            ShootingType::Random => Alien::new_random(x, y, vx, vy),
            ShootingType::Current => Alien::new_current(x, y, vx, vy),
            ShootingType::Future => Alien::new_future(x, y, vx, vy),
        })
    }
    fn new_random(x: f32, y: f32, mut vx: f32, mut vy: f32) -> Self {
        let vel = rand::rng().random_range(constants::alien::random::VEL_RANGE);

        vx *= vel;
        vy *= vel;

        Alien {
            x,
            y,

            vx,
            vy,

            shooting_type: ShootingType::Random,
            next_shot: unsafe { SDL_GetTicks64() } + 1000,

            bullet: None,
        }
    }

    fn new_current(x: f32, y: f32, mut vx: f32, mut vy: f32) -> Self {
        let vel = rand::rng().random_range(constants::alien::current::VEL_RANGE);

        vx *= vel;
        vy *= vel;

        Alien {
            x,
            y,

            vx,
            vy,

            shooting_type: ShootingType::Current,
            next_shot: unsafe { SDL_GetTicks64() } + 1000,

            bullet: None,
        }
    }

    fn new_future(x: f32, y: f32, mut vx: f32, mut vy: f32) -> Self {
        let vel = rand::rng().random_range(constants::alien::future::VEL_RANGE);

        vx *= vel;
        vy *= vel;

        Alien {
            x,
            y,

            vx,
            vy,

            shooting_type: ShootingType::Future,
            next_shot: unsafe { SDL_GetTicks64() } + 1000,

            bullet: None,
        }
    }

    pub fn tick(&mut self, dt: f32, screen_bounds: Rect, score: u64, player: (f32, f32, f32, f32)) {
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

        if unsafe { SDL_GetTicks64() } >= self.next_shot {
            let res = self.shoot(player.0, player.1, player.2, player.3);

            if res.is_ok() {
                let shots_per_second = match self.shooting_type {
                    ShootingType::Random => constants::alien::random::SHOTS_PER_SECOND_PER_POINT,
                    ShootingType::Current => constants::alien::current::SHOTS_PER_SECOND_PER_POINT,
                    ShootingType::Future => constants::alien::future::SHOTS_PER_SECOND_PER_POINT,
                };

                self.next_shot = unsafe { SDL_GetTicks64() }
                    + (1000.0 / (score as f32 * shots_per_second)) as u64;
            }
        }
    }

    fn shoot(&mut self, x: f32, y: f32, vx: f32, vy: f32) -> Result<(), ()> {
        let angle = match self.shooting_type {
            ShootingType::Random => rand::rng().random_range(0.0..(2.0 * PI)),
            ShootingType::Current => {
                let dx = x - self.x;
                let dy = y - self.y;

                dy.atan2(dx)
            }
            ShootingType::Future => {
                let angle_to_target = (y - self.y).atan2(x - self.x);
                let b_x = self.x + constants::alien::SHOOT_RADIUS * angle_to_target.cos();
                let b_y = self.y + constants::alien::SHOOT_RADIUS * angle_to_target.sin();

                let dx = x - b_x;
                let dy = y - b_y;

                let r_dot_v = dx * vx + dy * vy;
                let a_r2 = dx * dx + dy * dy;
                let a_v2 = vx * vx + vy * vy;
                let s2 = constants::bullet::VEL * constants::bullet::VEL;

                let discriminant = r_dot_v * r_dot_v - (a_v2 - s2) * a_r2;
                if discriminant < 0.0 {
                    return Err(());
                }

                let denominator = a_v2 - s2;
                if denominator.abs() < f32::EPSILON {
                    return Err(());
                }

                let t1 = (-r_dot_v + discriminant.sqrt()) / denominator;
                let t2 = (-r_dot_v - discriminant.sqrt()) / denominator;

                let t = [t1, t2]
                    .iter()
                    .cloned()
                    .filter(|&t| t > 0.0)
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .ok_or(())?;

                let aim_x = dx + vx * t;
                let aim_y = dy + vy * t;

                aim_y.atan2(aim_x)
            }
        };

        let b_x = self.x + constants::alien::SHOOT_RADIUS * angle.cos();
        let b_y = self.y + constants::alien::SHOOT_RADIUS * angle.sin();

        self.bullet = Some(Bullet::new(b_x, b_y, angle, false));

        Ok(())
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, screen_bounds: Rect) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let shape = match self.shooting_type {
            ShootingType::Random => constants::alien::random::SHAPE,
            ShootingType::Current => constants::alien::current::SHAPE,
            ShootingType::Future => constants::alien::future::SHAPE,
        };

        let points: Vec<Point> = shape
            .iter()
            .map(|p| Point::new((p.0 + self.x) as i32, (p.1 + self.y) as i32))
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

    pub fn get_bullet(&mut self) -> Option<Bullet> {
        let bullet = self.bullet.clone();
        self.bullet = None;
        bullet
    }

    pub fn get_hitboxes(&self, screen_bounds: Rect) -> Vec<Vec<(f32, f32)>> {
        let shape = match self.shooting_type {
            ShootingType::Random => constants::alien::random::SHAPE,
            ShootingType::Current => constants::alien::current::SHAPE,
            ShootingType::Future => constants::alien::future::SHAPE,
        };

        constants::window::WRAPPING_VALS
            .iter()
            .map(|v| {
                let dx = v[0] * screen_bounds.width() as i32;
                let dy = v[1] * screen_bounds.height() as i32;

                shape
                    .iter()
                    .map(|p| (p.0 + self.x + dx as f32, p.1 + self.y + dy as f32))
                    .collect::<Vec<(f32, f32)>>()
            })
            .collect::<Vec<Vec<(f32, f32)>>>()
    }

    pub fn get_type(&self) -> ShootingType {
        self.shooting_type.clone()
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }
}
