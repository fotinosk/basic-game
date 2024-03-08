use crate::utils;
use rand::Rng;

pub struct Ball {
    pub position: utils::Location,
    pub direction: utils::Location  // vector specifing the velocity
}

impl Ball {
    pub fn new(screen_width: f64 , screen_height: f64, paddle_height: f64) -> Ball {
        // initialize the ball at the middle of the paddle with a random velocity vector
        let mut rng = rand::thread_rng();
        let b = Ball {
            position: utils::Location{x: screen_width / 2.0, y: screen_height - 2.5 * paddle_height},
            direction: utils::Location{ x: rng.gen_range(-4.0..4.0), y: rng.gen_range(0.1..2.0) }
        };
        b
    }
    pub fn get_dims(&self) -> [f64;4] {
        [self.position.x, self.position.y, 20.0, 20.0]
    }
}
