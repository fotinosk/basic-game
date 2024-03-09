use crate::{utils, WIDTH};
use rand::Rng;

#[derive(Debug)]
enum BOUNCE_OBJ {
    LEFT_WALL,
    RIGHT_WALL, 
    TOP_WALL,
    BOTTOM_WALL,
    PADDLE, 
    NONE
}

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
            direction: utils::Location{ x: rng.gen_range(-4.0..4.0), y: rng.gen_range(-2.0..-0.1) }
        };
        b
    }
    pub fn get_dims(&self) -> [f64;4] {
        [self.position.x, self.position.y, 20.0, 20.0]
    }
    pub fn step(&mut self) {
        self.position.x = self.position.x + self.direction.x * crate::DT;
        self.position.y = self.position.y + self.direction.y * crate::DT;

        let bounce = self.check_bounce();

        match bounce {
            BOUNCE_OBJ::NONE => {},
            BOUNCE_OBJ::PADDLE => {
                // the y coord need to be negative
                self.direction.y = self.direction.y.abs() * -1.0; 
            }
            BOUNCE_OBJ::TOP_WALL => {
                self.direction.y = self.direction.y.abs(); 
            } 
            BOUNCE_OBJ::LEFT_WALL => {
                self.direction.x = self.direction.x.abs(); 
            } 
            BOUNCE_OBJ::RIGHT_WALL => {
                self.direction.x = self.direction.x.abs() * -1.0; 
            }
            BOUNCE_OBJ::BOTTOM_WALL => {
                println!("Ball is out of bounds, game over")
            }

        }
    }
    fn check_bounce(&mut self) -> BOUNCE_OBJ  {
        if self.position.y < 0.0 {
            BOUNCE_OBJ::TOP_WALL
        }
        else if self.position.y > crate::HEIGHT {
            BOUNCE_OBJ::BOTTOM_WALL 
        }
        else if self.position.x < 0.0 {
            BOUNCE_OBJ::LEFT_WALL
        }
        else if self.position.x > WIDTH {
            BOUNCE_OBJ::RIGHT_WALL
        }
        // TODO: implement paddle collision detection
        else {
            BOUNCE_OBJ::NONE
        }
    }
}
