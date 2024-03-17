use crate::{player::Paddle, utils, HEIGHT, WIDTH};

const BALLRADIUS: f64 = 20.0;

#[derive(Debug)]
enum BounceObj {
    LeftWall,
    RightWall, 
    TopWall,
    BottomWall,
    Paddle, 
    NoBounce 
}

pub struct Ball {
    pub position: utils::Location,
    pub direction: utils::Location, 
    pub charge: f64,
    radius: f64
}

impl Ball {
    pub fn new(screen_width: f64 , screen_height: f64, paddle_height: f64) -> Ball {
        let b = Ball {
            position: utils::Location{x: screen_width / 2.0, y: screen_height - 2.5 * paddle_height},
            direction: utils::Location{ x: 3.0, y: -3.0 },
            charge: 1.0,
            radius: BALLRADIUS
        };
        b
    }
    pub fn get_dims(&self) -> [f64;4] {
        [self.position.x, self.position.y, self.radius, self.radius]
    }
    pub fn get_centre(&self) -> [f64;2] {
        [self.position.x + self.radius/2.0, self.position.y + self.radius /2.0]
    }
    pub fn step(&mut self, paddle: &Paddle, acceleration: utils::Location) -> bool {
        self.direction.x += acceleration.x * crate::DT;
        self.direction.y += acceleration.y * crate::DT;

        self.position.x = self.position.x + self.direction.x * crate::DT;
        self.position.y = self.position.y + self.direction.y * crate::DT;

        let bounce = self.check_bounce(&paddle);

        match bounce {
            BounceObj::NoBounce => { true },
            BounceObj::Paddle => {
                // the y coord need to be negative
                println!("Paddle bounce");
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
    fn check_bounce(&mut self, paddle: &Paddle) -> BounceObj  {
        if self.position.y < 0.0 {
            BounceObj::TopWall
        }
        else if self.position.y > HEIGHT {
            BounceObj::BottomWall 
        }
        else if self.position.x < 0.0 {
            BounceObj::LeftWall
        }
        else if self.position.x > WIDTH - self.radius {
            BounceObj::RightWall
        }
        else if self.position.y > HEIGHT - 2.5 * crate::OFFSET && 
        paddle.position_lower_left.x < self.position.x &&
        self.position.x < paddle.position_lower_left.x + paddle.width {
                BounceObj::Paddle
        }
        else {
            BounceObj::NoBounce
        }
    }
}
