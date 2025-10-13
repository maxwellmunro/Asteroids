pub mod window {
    pub const TITLE: &str = "Asteroids";
    pub const WRAPPING_VALS: [[i32; 2]; 9] = [
        [0, 0],
        [-1, 0],
        [1, 0],
        [0, -1],
        [0, 1],
        [-1, -1],
        [-1, 1],
        [1, -1],
        [1, 1],
    ];
}

pub mod player {
    pub const TURN_SPEED: f32 = 7.0;
    pub const ACCELERATION: f32 = 1000.0;
    pub const DECELERATION: f32 = 0.7;
    pub const PLAYER_SHAPE: [[f32; 2]; 4] = [[0.0, -40.0], [15.0, 15.0], [0.0, 0.0], [-15.0, 15.0]];
}

pub mod particle {
    pub const MIN_LIFESPAN: u64 = 500;
    pub const MAX_LIFESPAN: u64 = 2000;
    pub mod thrust {
        use std::ops::Range;

        pub const PARTICLES_PER_SECOND: f32 = 10.0;
        pub const VEL_RANGE: Range<f32> = 300.0..500.0;
        pub const ANGLE_OFFSET: f32 = 0.5;
    }
}

pub mod bullet {
    pub const VEL: f32 = 1000.0;
    pub const LIFESPAN: u64 = 1000;
}

pub mod asteroid {
    use std::ops::Range;

    pub const MIN_RADIUS: f32 = 20.0;
    pub const SPAWN_RADIUS_RANGE: Range<f64> = 50.0..100.0;
    pub const RADIUS_OFFSET_FAC: f32 = 0.7;
    pub const POINTS_PER_RADIUS: f32 = 0.1;
    pub const VEL_RANGE: Range<f32> = 10.0..50.0;
    pub const SPAWN_RATE: f32 = 0.01;
    pub const MAX_SPAWN_ATTEMPTS: u32 = 10;
    pub const MIN_SPAWN_DISTANCE: f32 = 500.0 * 500.0;
}
