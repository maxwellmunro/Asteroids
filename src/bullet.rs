use crate::constants;
use crate::particle::Particle;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::sys::SDL_GetTicks64;
use sdl2::video::Window;

#[derive(Clone)]
pub struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    death: u64,

    last_particle: u64,
    particles_to_spawn: Vec<Particle>,

    pub to_die: bool,
}

impl Bullet {
    pub fn new(x: f32, y: f32, angle: f32) -> Bullet {
        let vx = constants::bullet::VEL * angle.cos();
        let vy = constants::bullet::VEL * angle.sin();

        Bullet {
            x,
            y,
            vx,
            vy,
            death: unsafe { SDL_GetTicks64() } + constants::bullet::LIFESPAN,

            last_particle: unsafe { SDL_GetTicks64() },
            particles_to_spawn: Vec::new(),

            to_die: false,
        }
    }

    pub fn tick(&mut self, dt: f32, screen_bounds: Rect) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        let fps = 1000.0 / dt;

        if constants::particle::bullet::PARTICLES_PER_SECOND > fps {
            let num_particles = (constants::particle::bullet::PARTICLES_PER_SECOND / fps) as i32;

            (0..num_particles).for_each(|_| {
                let vel = -rand::rng().random_range(constants::particle::bullet::VEL_RANGE);
                let angle = self.vy.atan2(self.vx)
                    + rand::rng().random_range(
                        -constants::particle::bullet::ANGLE_OFFSET
                            ..constants::particle::bullet::ANGLE_OFFSET,
                    );

                let vx = vel * angle.cos();
                let vy = vel * angle.sin();

                self.particles_to_spawn
                    .push(Particle::new(self.x, self.y, vx, vy))
            });

            self.last_particle = unsafe { SDL_GetTicks64() };
        } else {
            let ms_per_particle =
                (1000.0 / constants::particle::bullet::PARTICLES_PER_SECOND) as u64;

            if self.last_particle + ms_per_particle < unsafe { SDL_GetTicks64() } {
                self.last_particle = unsafe { SDL_GetTicks64() };
                let vel = -rand::rng().random_range(constants::particle::bullet::VEL_RANGE);
                let angle = self.vy.atan2(self.vx)
                    + rand::rng().random_range(
                        -constants::particle::bullet::ANGLE_OFFSET
                            ..constants::particle::bullet::ANGLE_OFFSET,
                    );

                let vx = vel * angle.cos();
                let vy = vel * angle.sin();

                self.particles_to_spawn
                    .push(Particle::new(self.x, self.y, vx, vy));
            }
        }

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
        let rect = Rect::new(self.x as i32 - 1, self.y as i32 - 1, 3, 3);
        canvas.fill_rect(rect)?;

        Ok(())
    }

    pub fn is_alive(&self) -> bool {
        self.death >= unsafe { SDL_GetTicks64() }
    }

    pub fn get_particles_to_spawn(&mut self) -> Vec<Particle> {
        let particles = self.particles_to_spawn.clone();
        self.particles_to_spawn.clear();
        particles
    }

    pub fn get_physics_trail(&self, dt: f32) -> Vec<(f32, f32)> {
        vec![
            (self.x, self.y),
            (self.x + self.vx * dt, self.y + self.vy * dt),
        ]
    }
    
    pub fn get_location(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
