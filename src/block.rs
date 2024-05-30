use std::collections::HashMap;
use rand::Rng;

use crate::{ball::Ball, constants, utils};
use piston_window::*;

pub enum Collision {
    Left, 
    Right,
    Top,
    Bottom,
    NoCollision
}

#[derive(Debug, Clone)]
pub struct Block {
    pub position: utils::Location,
    pub charge: f64,
    width: f64,
    height: f64,
    pub active: bool
}

impl Block {
    pub fn new(position_x: f64, position_y: f64, charge: f64) -> Block{
        let block = Block{
            position: utils::Location { x: position_x, y: position_y },
            charge,
            width: constants::BLOCK_WIDTH,
            height: constants::BLOCK_HEIGHT,
            active: true
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
        let lower_left_x = self.position.x - self.width / 2.0 - constants::BLOCK_VISIBILITY_MARGIN;
        let lower_left_y = self.position.y - self.height / 2.0 - constants::BLOCK_VISIBILITY_MARGIN;
        rectangle(
            color, 
            [lower_left_x, lower_left_y, self.width - constants::BLOCK_VISIBILITY_MARGIN, self.height - constants::BLOCK_VISIBILITY_MARGIN], 
            transform, 
            g
        )
    }
    pub fn get_center(&self) -> [f64; 2] {
        return [self.position.x, self.position.y]
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

pub struct BlockGrid {
    // Implements the grid that holds multiple blocks
    num_rows: u8, 
    num_cols: u8,
    horizontal_offset: f64,
    vertical_offset: f64,
    block_store: HashMap<utils::Location, Block>  // hashmap indexed by block center location
}

impl BlockGrid {
    pub fn new(num_rows: u8, num_cols: u8) -> BlockGrid {
        let mut block_store = HashMap::new();

        // Generate all blocks
        let mut rng = rand::thread_rng();
        let hor_offset = (constants::WIDTH - num_cols as f64 * constants::BLOCK_WIDTH) / 2.0;

        for col in 0..num_cols {
            for row in 0..num_rows {
                let x = col as f64 * constants::BLOCK_WIDTH + constants::BLOCK_WIDTH / 2.0 + hor_offset;
                let y = row as f64 * constants::BLOCK_HEIGHT + constants::BLOCK_HEIGHT / 2.0 + constants::VERTICAL_BLOCK_OFFSET;

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

        let block_grid = BlockGrid{
            num_cols,
            num_rows,
            horizontal_offset: hor_offset,
            vertical_offset: constants::VERTICAL_BLOCK_OFFSET,
            block_store
        };
        
        block_grid
    }

    pub fn draw<G: Graphics>(&self, g: &mut G, transform: [[f64;3]; 2]) {
        for (_key, val) in &self.block_store {
            if val.active {
                val.draw(g, transform)
            }

        }
    }

    pub fn step(&mut self, ball: &Ball) -> Collision {
        // Check if there has been a colision between the block grid and the ball, return the
        // result so the ball can be updated too. Delete the colided with block
        let x_min = self.horizontal_offset;
        let y_min = self.vertical_offset;

        let x_max = x_min + self.num_cols as f64 * constants::BLOCK_WIDTH;
        let y_max = y_min + self.num_rows as f64 * constants::BLOCK_HEIGHT;

        let ball_loc = ball.get_centre();

        if x_min < ball_loc[0] && ball_loc[0] < x_max && y_min < ball_loc[1] && ball_loc[1] < y_max {
            // need to figure out from which direction is the colision
            // and which block is collided
            self.detect_block_collision(&ball) 
        } else {
            Collision::NoCollision
        }
    }

    pub fn draw_nearest_block_center<G: Graphics>(&self, ball: &Ball, g: &mut G, transform: [[f64;3]; 2]) {
        // TODO: utility function, remove later
        let center_loc = self.get_nearest_block_center(ball.get_centre());
        ellipse(
            [1.0, 1.0, 1.0, 1.0], 
            [center_loc[0], center_loc[1], 12.0, 12.0], 
            transform, 
            g
        )
    }

    fn detect_block_collision(&mut self, ball: &Ball) -> Collision {
        // Detect if it was a collision, the direction so the ball direction can be updated and
        // remove the collided block
        
        let nearest_block_coords = self.get_nearest_block_center(ball.get_centre());
        let nearest_block = self.block_store.get(&utils::Location{x: nearest_block_coords[0], y: nearest_block_coords[1]});

        // step the ball forward and see where it collides
        // edge case when it could collide with corner
        let block_min_x = nearest_block_coords[0] - constants::BLOCK_WIDTH / 2.0;
        let block_max_x = nearest_block_coords[0] + constants::BLOCK_WIDTH / 2.0;

        let block_min_y = nearest_block_coords[1] - constants::BLOCK_HEIGHT / 2.0;
        let block_max_y = nearest_block_coords[1] + constants::BLOCK_HEIGHT / 2.0;

        let ball_loc = ball.get_centre();

        // FIXME: this is naive - the position update takes acceleration too
        let next_ball_loc = [ball_loc[0] + ball.position.x, ball_loc[1] + ball.position.y];


        Collision::Top // Placeholder
    }

    fn get_nearest_block_center(&self, ball_location: [f64; 2]) -> [f64; 2] {
        // Observation - As long as the distance of the blocks (both in x and y) is bigger than the
        // size of the ball, then the ball can only collide with its closest block. If the block is
        // inactive then it won't collide
        
        let ball_loc_offset_x = ball_location[0] - self.horizontal_offset;
        let ball_loc_offset_y = ball_location[1] - self.vertical_offset;

        let block_incr_x = (ball_loc_offset_x / constants::BLOCK_WIDTH) as u32;
        let block_incr_y = (ball_loc_offset_y / constants::BLOCK_HEIGHT) as u32;

        return [self.horizontal_offset + (block_incr_x as f64 + 0.5) * constants::BLOCK_WIDTH, self.vertical_offset + (block_incr_y as f64 + 0.5) * constants::BLOCK_HEIGHT] 
    }
}
