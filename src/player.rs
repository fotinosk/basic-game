use piston_window::*;

use crate::{utils, constants};

#[derive(Debug)]
pub struct Paddle {
    pub position_lower_left: utils::Location,
    pub move_direction: utils::Direction,
    pub width: f64,
    pub height: f64

}

impl Paddle {
    pub fn new(screen_width: f64, screen_height: f64, offset: f64) -> Paddle {
        let paddle = Paddle{
            position_lower_left : utils::Location{x: (screen_width - constants::PADDLE_LEN) / 2.0, y: screen_height - constants::PADDLE_HEIGHT - offset}, 
            move_direction: utils::Direction::Stationary,
            width: constants::PADDLE_LEN,
            height: constants::PADDLE_HEIGHT
        };
        paddle
    }

    pub fn move_horizontal(&mut self, dir: utils::Direction) {
        self.move_direction = dir;
    }

    pub fn step(&mut self) {
        match self.move_direction {
            utils::Direction::Left => {
                self.position_lower_left.x -= constants::SPEED * constants::DT;
                self.position_lower_left.x = self.position_lower_left.x.max(0.0);
            }
            utils::Direction::Right => {
                self.position_lower_left.x += constants::SPEED * constants::DT;
                self.position_lower_left.x = (self.position_lower_left.x).min(constants::WIDTH - constants::PADDLE_LEN);
            }
            utils::Direction::Stationary => {}
        }
    }

    pub fn get_dims(&self) -> [f64; 4] {
        [self.position_lower_left.x, self.position_lower_left.y, self.width, self.height]
    }
    pub fn get_centre(&self) -> [f64;2] {
        [self.position_lower_left.x + constants::PADDLE_LEN / 2.0, self.position_lower_left.y]
    }
    pub fn draw<G: Graphics>(&self, g: &mut G, transform: [[f64;3]; 2]) {
        rectangle(constants::PADDDLE_COLOR, self.get_dims(), transform, g);
    }
}
