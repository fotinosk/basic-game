use std::collections::HashMap;
use rand::Rng;

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
    pub fn get_center(&self) -> [f64; 2] {
        return [self.position.x, self.position.y]
    }
}

pub struct BlockGrid {
    // Implements the grid that holds multiple blocks
    num_rows: i8, 
    num_cols: i8,
    // block store is a dictionary indexed by the x and y coordinates of the center of each block
    block_store: HashMap<utils::Location, Block>
}

impl BlockGrid {
    pub fn new(random_charge: bool, num_rows: i8, num_cols: i8) -> BlockGrid {
        let block_store = HashMap::new();

        // Generate all blocks
        Self::generate_blocks(&block_store, num_cols, num_rows);

        let mut block_grid = BlockGrid{
            num_cols,
            num_rows,
            block_store
        };
        block_grid
    }

    fn generate_blocks(block_store: &HashMap<utils::Location, Block>, num_cols: i8, num_rows: i8) {
        let mut rng = rand::thread_rng();

        for col in 0..num_cols {
            for row in 0..num_rows {
                // TODO: center on screen
                let x = col as f64 * constants::BLOCK_WIDTH + constants::BLOCK_WIDTH / 2.0;
                let y = row as f64 * constants::BLOCK_HEIGHT + constants::BLOCK_HEIGHT / 2.0;

                let mut charge: f64 = rng.gen();
                charge = match charge {
                    c if c < 0.5 => 0.0,
                    c if c < 0.75 => 1.0,
                    _ => -1.0,
                };
                let b = Block::new(x, y, charge);
                block_store.insert(utils::Location{x, y}, b);
            }
        }
    }
}
