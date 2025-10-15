use crate::asteroid::Asteroid;
use crate::bullet::Bullet;
use crate::constants;
use crate::font;
use crate::high_score;
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

enum GameState {
    MainMenu,
    InGame,
}

pub struct Game {
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,

    screen_bounds: Rect,

    space_released: bool,
    next_asteroid_spawn: u64,

    lives: u32,
    next_life_points: u64,

    score: u64,
    pb: u64,

    player: Player,

    particles: Vec<Particle>,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,

    state: GameState,
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

        let pb = high_score::load_score("highscore.bin").unwrap_or(0);

        Ok(Self {
            canvas,
            event_pump,

            screen_bounds,

            space_released: true,
            next_asteroid_spawn: Self::get_next_asteroid_spawn(0),

            lives: constants::player::START_LIVES,
            next_life_points: constants::player::POINTS_PER_LIFE,

            score: 0,
            pb,

            player: p,

            particles: Vec::new(),
            bullets: Vec::new(),
            asteroids: Vec::new(),

            state: GameState::MainMenu,
        })
    }

    pub fn run(&mut self) {
        let mut last_tick = unsafe { SDL_GetTicks64() };

        'running: loop {
            let new_size = self.canvas.window().drawable_size();
            if new_size.0 > 0 && new_size.1 > 0 {
                self.screen_bounds.set_width(new_size.0);
                self.screen_bounds.set_height(new_size.1);
            }

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

            match self.state {
                GameState::MainMenu => {
                    if let Err(e) = self.render_main_menu() {
                        println!("{}{}", constants::strings::RENDER_ERROR, e);
                    }
                }
                GameState::InGame => {
                    self.tick_game(dt);

                    if let Err(e) = self.render_game() {
                        println!("{}{}", constants::strings::RENDER_ERROR, e);
                    }
                }
            }
        }
    }

    fn tick_game(&mut self, dt: f32) {
        if unsafe { SDL_GetTicks64() } > self.next_asteroid_spawn
            && constants::asteroid::MAX_ASTEROIDS > self.asteroids.len() as u32
        {
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
                let intersects = a
                    .get_hitboxes(self.screen_bounds)
                    .iter()
                    .any(|hitbox| polygon::polygons_intersect(hitbox, line.as_slice()))
                    && !b.to_die;

                if intersects {
                    b.to_die = true;

                    self.score += (constants::asteroid::SCORE_PER_RADIUS / a.get_radius()) as u64;
                    if self.score > self.next_life_points {
                        self.next_life_points += constants::player::POINTS_PER_LIFE;
                        self.lives += 1;
                    }
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
        self.asteroids
            .iter_mut()
            .for_each(|a| a.tick(dt, self.screen_bounds));

        if self.asteroids.iter().any(|a| {
            a.get_hitboxes(self.screen_bounds).iter().any(|hitbox| {
                polygon::polygons_intersect(hitbox, self.player.get_hitbox().as_slice())
            })
        }) {
            self.die();
        }
    }

    fn render_game(&mut self) -> Result<(), String> {
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
        let pb_str = self.pb.to_string();
        let pb_length = (pb_str.len() as u32 * constants::font::FONT_SIZE)
            + ((pb_str.len() - 1) as u32 * constants::font::MARGIN);
        font::render_text(
            pb_str.as_str(),
            (self.screen_bounds.width() - pb_length - 10) as i32,
            10,
            &mut self.canvas,
        )?;
        font::render_lives(self.lives, &self.screen_bounds, &mut self.canvas)?;

        self.canvas.present();

        Ok(())
    }

    fn render_main_menu(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        let string_width = (constants::strings::START_TEXT.len() as u32
            * constants::font::FONT_SIZE)
            + ((constants::strings::START_TEXT.len() - 1) as u32 * constants::font::MARGIN);

        if self.screen_bounds.width() < string_width
            || self.screen_bounds.height() / 2 < constants::font::FONT_SIZE
        {
            self.canvas.present();

            return Err(String::from(constants::strings::WINDOW_SIZE_ERROR));
        }

        let x = (self.screen_bounds.width() - string_width) / 2;
        font::render_text(
            constants::strings::START_TEXT,
            x as i32,
            (self.screen_bounds.height() / 2 - constants::font::FONT_SIZE) as i32,
            &mut self.canvas,
        )?;

        self.canvas.present();

        Ok(())
    }

    fn handle_key_event(&mut self, key: Keycode, pressed: bool) {
        match self.state {
            GameState::MainMenu => {
                if key == Keycode::SPACE {
                    self.state = GameState::InGame;
                }
            }
            GameState::InGame => {
                self.player.handle_key_event(key, pressed);

                if key == Keycode::SPACE {
                    if pressed && self.space_released {
                        self.bullets.push(self.player.shoot_bullet());
                    }

                    self.space_released = !pressed;
                }
            }
        }
    }

    fn get_next_asteroid_spawn(score: u64) -> u64 {
        let offset = if score < 50 {
            20.0 / constants::asteroid::SPAWN_RATE
        } else {
            1000.0 / (score as f32 * constants::asteroid::SPAWN_RATE)
        } as u64;
        (unsafe { SDL_GetTicks64() } as u64) + offset
    }

    fn die(&mut self) {
        self.player.die(self.screen_bounds);

        self.asteroids.clear();
        self.particles.clear();
        self.bullets.clear();

        if self.lives == 1 {
            self.lives = constants::player::START_LIVES;

            if self.score > self.pb {
                if let Err(e) =
                    high_score::save_score(constants::strings::HIGH_SCORE_PATH, self.score)
                {
                    println!("{}{}", constants::strings::HIGH_SCORE_ERROR, e);
                }

                self.pb = self.score;
            }

            self.score = 0;
        } else {
            self.lives -= 1;
        }
    }
}
