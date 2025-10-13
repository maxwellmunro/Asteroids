use crate::constants;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::sys::SDL_GetTicks64;
use sdl2::video::Window;

pub struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    death: u64,
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

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let rect = Rect::new(self.x as i32 - 1, self.y as i32 - 1, 3, 3);
        canvas.fill_rect(rect)?;

        Ok(())
    }

    pub fn is_alive(&self) -> bool {
        self.death >= unsafe { SDL_GetTicks64() }
    }
}
