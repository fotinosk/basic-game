use rand::Rng;
use core::panic;
use std::collections::HashMap;

use crate::{ball::Ball, constants, utils};
use piston_window::*;

pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    NoCollision,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub position: utils::Location,
    pub charge: f64,
    width: f64,
    height: f64,
    pub active: bool,
}

impl Block {
    pub fn new(position_x: f64, position_y: f64, charge: f64) -> Block {
        let block = Block {
            position: utils::Location {
                x: position_x,
                y: position_y,
            },
            charge,
            width: constants::BLOCK_WIDTH,
            height: constants::BLOCK_HEIGHT,
            active: true,
        };
        block
    }
    pub fn draw<G: Graphics>(&self, g: &mut G, transform: [[f64; 3]; 2]) {
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
            [
                lower_left_x,
                lower_left_y,
                self.width - constants::BLOCK_VISIBILITY_MARGIN,
                self.height - constants::BLOCK_VISIBILITY_MARGIN,
            ],
            transform,
            g,
        )
    }
    pub fn get_center(&self) -> [f64; 2] {
        return [self.position.x, self.position.y];
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
    block_store: HashMap<utils::Location, Block>, // hashmap indexed by block center location
}

impl BlockGrid {
    pub fn new(num_rows: u8, num_cols: u8) -> BlockGrid {
        let mut block_store = HashMap::new();

        // Generate all blocks
        let mut rng = rand::thread_rng();
        let hor_offset = (constants::WIDTH - num_cols as f64 * constants::BLOCK_WIDTH) / 2.0;

        for col in 0..num_cols {
            for row in 0..num_rows {
                let x =
                    col as f64 * constants::BLOCK_WIDTH + constants::BLOCK_WIDTH / 2.0 + hor_offset;
                let y = row as f64 * constants::BLOCK_HEIGHT
                    + constants::BLOCK_HEIGHT / 2.0
                    + constants::VERTICAL_BLOCK_OFFSET;

                let mut charge: f64 = rng.gen();
                charge = match charge {
                    c if c < 0.5 => 0.0,
                    c if c < 0.75 => 1.0,
                    _ => -1.0,
                };
                let b = Block::new(x, y, charge);
                block_store.insert(utils::Location { x, y }, b);
            }
        }

        let block_grid = BlockGrid {
            num_cols,
            num_rows,
            horizontal_offset: hor_offset,
            vertical_offset: constants::VERTICAL_BLOCK_OFFSET,
            block_store,
        };

        block_grid
    }

    pub fn draw<G: Graphics>(&self, g: &mut G, transform: [[f64; 3]; 2]) {
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

        if x_min < ball_loc[0] && ball_loc[0] < x_max && y_min < ball_loc[1] && ball_loc[1] < y_max
        {
            // need to figure out from which direction is the colision
            // and which block is collided
            self.detect_block_collision(&ball)
        } else {
            Collision::NoCollision
        }
    }

    fn detect_block_collision(&mut self, ball: &Ball) -> Collision {
        // Determine if the ball collides with an active block, if it does, then deactivate the
        // block and share the collision direction so it can change the ball trajectory
        let nearest_block_coords = self.get_nearest_block_center(ball.get_centre());

        let ball_location = ball.get_centre();
        let ball_direction = ball.get_direction();
        let ball_new_loc = [ball_location[0] + ball_direction.x * constants::DT, ball_location[1] + ball_direction.y * constants::DT];

        let left_border = nearest_block_coords[0] - constants::BLOCK_WIDTH / 2.0 - constants::BALLRADIUS;
        let right_border = nearest_block_coords[0] + constants::BLOCK_WIDTH / 2.0 + constants::BALLRADIUS;
        let bottom_border = nearest_block_coords[1] - constants::BLOCK_HEIGHT/ 2.0 - constants::BALLRADIUS; 
        let top_border = nearest_block_coords[1] + constants::BLOCK_HEIGHT/ 2.0 + constants::BALLRADIUS; 

        if ball_new_loc[0] < left_border {
            return Collision::NoCollision
        }
        if ball_new_loc[0] > right_border {
            return Collision::NoCollision
        }
        if ball_new_loc[1] < bottom_border {
            return Collision::NoCollision
        }
        if ball_new_loc[1] > top_border {
            return Collision::NoCollision
        }

        // there must be a collision
        // check that the nearest block is active, otherwise it won't collide
        let nearest_block = self.block_store.get_mut(&utils::Location {
            x: nearest_block_coords[0],
            y: nearest_block_coords[1],
        }).unwrap();

        if !nearest_block.active {
            return Collision::NoCollision
        }
        else {
            nearest_block.deactivate();
        }

        // match nearest_block {
        //     None => panic!("Error could not find nearest block"),
        //     Some(&mut Block) => {
        //         if !Block.active {
        //             return Collision::NoCollision
        //         }
        //         else {
        //             Block.deactivate()
        //         }
        //     }
        // }

        if ball_new_loc[0] > left_border && ball_location[0] < left_border {
            return Collision::Left
        }
        if ball_new_loc[0] < right_border && ball_location[0] > right_border {
            return Collision::Right
        }
        if ball_new_loc[1] > bottom_border && ball_location[1] < bottom_border {
            return Collision::Bottom
        }
        if ball_new_loc[1] < bottom_border && ball_location[1] > bottom_border {
            return Collision::Top
        }
        Collision::NoCollision

    }

    fn get_nearest_block_center(&self, ball_location: [f64; 2]) -> [f64; 2] {
        // Observation - As long as the distance of the blocks (both in x and y) is bigger than the
        // size of the ball, then the ball can only collide with its closest block. If the block is
        // inactive then it won't collide

        let ball_loc_offset_x = ball_location[0] - self.horizontal_offset;
        let ball_loc_offset_y = ball_location[1] - self.vertical_offset;

        let block_incr_x = (ball_loc_offset_x / constants::BLOCK_WIDTH) as u32;
        let block_incr_y = (ball_loc_offset_y / constants::BLOCK_HEIGHT) as u32;

        return [
            self.horizontal_offset + (block_incr_x as f64 + 0.5) * constants::BLOCK_WIDTH,
            self.vertical_offset + (block_incr_y as f64 + 0.5) * constants::BLOCK_HEIGHT,
        ];
    }
}
