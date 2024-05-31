use crate::{constants, player::Paddle, utils};
use piston_window::*;

#[derive(Debug)]
enum BounceObj {
    LeftWall,
    RightWall,
    TopWall,
    BottomWall,
    Paddle,
    NoBounce,
}

pub struct Ball {
    pub position: utils::Location,
    pub direction: utils::Location,
    pub charge: f64,
    radius: f64,
}

impl Ball {
    pub fn new() -> Ball {
        let b = Ball {
            position: utils::Location {
                x: constants::WIDTH / 2.0,
                y: constants::HEIGHT
                    - constants::OFFSET
                    - constants::PADDLE_HEIGHT
                    - constants::BALLRADIUS,
            },
            direction: utils::Location {
                x: constants::INIT_BALL_SPEED_X,
                y: constants::INIT_BALL_SPEED_Y,
            },
            charge: 1.0,
            radius: constants::BALLRADIUS,
        };
        b
    }
    pub fn get_dims(&self) -> [f64; 4] {
        [self.position.x, self.position.y, self.radius, self.radius]
    }
    pub fn get_centre(&self) -> [f64; 2] {
        [
            self.position.x + self.radius / 2.0,
            self.position.y + self.radius / 2.0,
        ]
    }
    pub fn get_direction(&self) -> utils::Location {
       self.direction 
    }
    pub fn step(&mut self, paddle: &Paddle, acceleration: utils::Location) -> bool {
        self.direction.x += acceleration.x * constants::DT;
        self.direction.y += acceleration.y * constants::DT;

        self.position.x = self.position.x + self.direction.x * constants::DT;
        self.position.y = self.position.y + self.direction.y * constants::DT;

        let bounce = self.check_bounce(&paddle);

        match bounce {
            BounceObj::NoBounce => true,
            BounceObj::Paddle => {
                // the y coord needs to be negative
                self.direction.y = self.direction.y.abs() * -1.0;
                true
            }
            BounceObj::TopWall => {
                self.direction.y = self.direction.y.abs();
                true
            }
            BounceObj::LeftWall => {
                self.direction.x = self.direction.x.abs();
                true
            }
            BounceObj::RightWall => {
                self.direction.x = self.direction.x.abs() * -1.0;
                true
            }
            BounceObj::BottomWall => {
                println!("Ball is out of bounds, game over");
                false
            }
        }
    }
    fn check_bounce(&mut self, paddle: &Paddle) -> BounceObj {
        if self.position.y < 0.0 {
            BounceObj::TopWall
        } else if self.position.y > constants::HEIGHT {
            BounceObj::BottomWall
        } else if self.position.x < 0.0 {
            BounceObj::LeftWall
        } else if self.position.x > constants::WIDTH - self.radius {
            BounceObj::RightWall
        } else if self.position.y
            > constants::HEIGHT
                - constants::OFFSET
                - 0.5 * constants::PADDLE_HEIGHT
                - constants::BALLRADIUS
            && paddle.position_lower_left.x < self.position.x
            && self.position.x < paddle.position_lower_left.x + paddle.width
        {
            BounceObj::Paddle
        } else {
            BounceObj::NoBounce
        }
    }
    pub fn draw<G: Graphics>(&self, g: &mut G, transform: [[f64; 3]; 2]) {
        ellipse(constants::BALL_COLOR, self.get_dims(), transform, g);
    }
}
