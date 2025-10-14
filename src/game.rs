use crate::asteroid::Asteroid;
use crate::bullet::Bullet;
use crate::constants;
use crate::font;
use crate::particle::Particle;
use crate::player::Player;
use crate::polygon;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::sys::SDL_GetTicks64;
use sdl2::video::Window;

pub struct Game {
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,

    screen_bounds: Rect,

    space_released: bool,
    next_asteroid_spawn: u64,

    score: u32,

    player: Player,

    particles: Vec<Particle>,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(constants::window::TITLE, 0, 0)
            .position_centered()
            .fullscreen()
            .build()
            .map_err(|e| e.to_string())?;

        let display_index = window.display_index()?;
        let screen_bounds = video_subsystem.display_bounds(display_index)?;

        let canvas = window
            .into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        let p = Player::new(
            (screen_bounds.width() / 2) as f32,
            (screen_bounds.height() / 2) as f32,
        );

        Ok(Self {
            canvas,
            event_pump,

            screen_bounds,

            space_released: true,
            next_asteroid_spawn: Self::get_next_asteroid_spawn(0),

            score: 0,

            player: p,

            particles: Vec::new(),
            bullets: Vec::new(),
            asteroids: Vec::new(),
        })
    }

    pub fn run(&mut self) {
        let mut last_tick = unsafe { SDL_GetTicks64() };

        'running: loop {
            for event in self.event_pump.poll_iter().collect::<Vec<Event>>() {
                match event {
                    Event::Quit { .. } => break 'running,

                    Event::KeyDown {
                        keycode: Some(k), ..
                    } => self.handle_key_event(k, true),
                    Event::KeyUp {
                        keycode: Some(k), ..
                    } => self.handle_key_event(k, false),

                    _ => {}
                }
            }

            let now = unsafe { SDL_GetTicks64() };
            let dt = (now - last_tick) as f32 / 1000.0;
            last_tick = now;

            self.tick(dt);

            if let Err(e) = self.render() {
                println!("Error rendering game: {}", e);
            }
        }
    }

    fn tick(&mut self, dt: f32) {
        if unsafe { SDL_GetTicks64() } > self.next_asteroid_spawn {
            self.next_asteroid_spawn = Self::get_next_asteroid_spawn(self.score);

            let (x, y) = Asteroid::get_spawn_location(
                self.player.get_x(),
                self.player.get_y(),
                self.screen_bounds,
            );

            let radius = rand::rng().random_range(constants::asteroid::SPAWN_RADIUS_RANGE);

            self.asteroids.push(Asteroid::new(x, y, radius as f32));
        }

        self.player.tick(dt, self.screen_bounds);
        self.particles.append(&mut self.player.get_particles());

        self.particles.retain(|p| p.is_alive());
        self.particles
            .iter_mut()
            .for_each(|p| p.tick(dt, self.screen_bounds));

        self.bullets.retain(|b| b.is_alive() && !b.to_die);
        self.bullets.iter_mut().for_each(|b| {
            b.tick(dt, self.screen_bounds);
            self.particles.append(&mut b.get_particles_to_spawn())
        });

        let mut asteroids_to_add: Vec<Asteroid> = Vec::new();

        self.asteroids.retain(|a| {
            let remove = self.bullets.iter_mut().any(|b| {
                let line = b.get_physics_trail(dt);
                let intersects =
                    polygon::polygons_intersect(line.as_slice(), a.get_hitbox().as_slice())
                        && !b.to_die;
                if intersects {
                    b.to_die = true;

                    self.score += (constants::asteroid::SCORE_PER_RADIUS / a.get_radius()) as u32;
                }

                intersects
            });

            if remove {
                if let Some(mut asteroids) = a.check_split() {
                    asteroids_to_add.append(&mut asteroids);
                }
                self.particles
                    .append(&mut Particle::generate_explosion_particles(
                        a.get_x(),
                        a.get_y(),
                    ));
            }

            !remove
        });

        self.asteroids.append(&mut asteroids_to_add);

        self.asteroids.iter_mut().for_each(|a| {
            a.tick(dt, self.screen_bounds);
            if polygon::polygons_intersect(
                a.get_hitbox().as_slice(),
                self.player.get_hitbox().as_slice(),
            ) {
                println!("Died!");
            }
        });
    }

    fn render(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.player.render(&mut self.canvas, self.screen_bounds)?;

        self.particles
            .iter()
            .try_for_each(|p| p.render(&mut self.canvas))?;

        self.bullets
            .iter()
            .try_for_each(|b| b.render(&mut self.canvas))?;

        self.asteroids
            .iter()
            .try_for_each(|a| a.render(&mut self.canvas, self.screen_bounds))?;

        font::render_text(self.score.to_string().as_str(), 10, 10, &mut self.canvas)?;

        self.canvas.present();

        Ok(())
    }

    fn handle_key_event(&mut self, key: Keycode, pressed: bool) {
        self.player.handle_key_event(key, pressed);

        if key == Keycode::SPACE {
            if pressed && self.space_released {
                self.bullets.push(self.player.shoot_bullet());
            }

            self.space_released = !pressed;
        }
    }

    fn get_next_asteroid_spawn(score: u32) -> u64 {
        let offset = if score < 50 {
            20.0 / constants::asteroid::SPAWN_RATE
        } else {
            1000.0 / (score as f32 * constants::asteroid::SPAWN_RATE)
        } as u64;
        (unsafe { SDL_GetTicks64() } as u64) + offset
    }
}
