use crate::utils;

const SPEED: f64 = 4.0;

#[derive(Debug)]
pub struct Paddle {
    pub position_lower_left: utils::Location,
    pub position_upper_right: utils::Location,
    pub move_direction: utils::Direction
}

impl Paddle {
    pub fn move_horizontal(&mut self, dir: utils::Direction) {
        self.move_direction = dir;
    }
    pub fn step(&mut self) {
        match self.move_direction {
            utils::Direction::Left => {
                self.position_upper_right.x -= SPEED;
                self.position_lower_left.x -= SPEED;
            }
            utils::Direction::Right => {
                self.position_upper_right.x += SPEED;
                self.position_lower_left.x += SPEED;
            }
            utils::Direction::Stationary => {}
        }
    }
}
