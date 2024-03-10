use crate::utils;

const SPEED: f64 = 4.0;
const PADDLE_LEN: f64 = 50.0;
const PADDLE_HEIGHT: f64 = 10.0;

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
            position_lower_left : utils::Location{x: (screen_width - PADDLE_LEN) / 2.0, y: screen_height - PADDLE_HEIGHT - offset}, 
            move_direction: utils::Direction::Stationary,
            width: PADDLE_LEN,
            height: PADDLE_HEIGHT
        };
        paddle
    }

    pub fn move_horizontal(&mut self, dir: utils::Direction) {
        self.move_direction = dir;
    }

    pub fn step(&mut self) {
        match self.move_direction {
            utils::Direction::Left => {
                self.position_lower_left.x -= SPEED;
                self.position_lower_left.x = self.position_lower_left.x.max(0.0);
            }
            utils::Direction::Right => {
                self.position_lower_left.x += SPEED;
                self.position_lower_left.x = (self.position_lower_left.x).min(crate::WIDTH - PADDLE_LEN);
            }
            utils::Direction::Stationary => {}
        }
    }

    pub fn get_dims(&self) -> [f64; 4] {
        [self.position_lower_left.x, self.position_lower_left.y, self.width, self.height]
    }
}
