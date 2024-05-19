use crate::{utils, constants};
use piston_window::*;

pub struct Block {
    pub position: utils::Location,
    pub charge: f64,
    width: f64,
    height: f64
}

impl Block {
    pub fn new(position_x: f64, position_y: f64, charge: f64) -> Block{
        let block = Block{
            position: utils::Location { x: position_x, y: position_y },
            charge,
            width: constants::BLOCK_WIDTH,
            height: constants::BLOCK_HEIGHT
        };
        block
    }
    pub fn draw<G: Graphics>(&self, g: &mut G, transform: [[f64;3]; 2]) {
        let color;
        if self.charge == 0.0 {
            color = constants::UNCHARGED_BLOCK_COLOR;
        } else if self.charge > 0.0 {
            color = constants::POSITIVE_BLOCK_COLOR;
        } else {
            color = constants::NEGATIVE_BLOCK_COLOR;
        }
        let lower_left_x = self.position.x - self.width / 2.0;
        let lower_left_y = self.position.y - self.height / 2.0;
        rectangle(
            color, 
            [lower_left_x, lower_left_y, self.width, self.height], 
            transform, 
            g
        )
    }
}
