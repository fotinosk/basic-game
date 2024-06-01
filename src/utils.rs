use std::hash::{Hash, Hasher};

use crate::{ball, constants, force_fields::Force, utils};
use piston_window::*;

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Stationary,
}

#[derive(Debug)]
pub enum GameState {
    InPlay,
    GameOver,
    NotStarted,
    Paused,
    Finished
}

impl Location {
    pub fn normalize(&self) -> Location {
        let magn = (self.x * self.x + self.y * self.y).sqrt();
        Location {
            x: self.x / magn,
            y: self.y / magn,
        }
    }
    pub fn scale(&self, factor: f64) -> Location {
        Location {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
    pub fn magnitute(&self) -> f64 {
        let magn = (self.x * self.x + self.y * self.y).sqrt();
        magn
    }
    // pub fn get_normal_vector(&self) -> Location {
    //     let unit_vector = self.normalize();
    //     Location{x : unit_vector.y, y: -1.0 * unit_vector.x}
    // }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Location {} // only PartialEq needed

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
    }
}

pub fn color_by_distance(body1: &[f64; 2], body2: &[f64; 2]) -> [f32; 4] {
    // if the distance is zero then the color is red
    // the further they get the more green it gets

    let max_distance =
        (constants::WIDTH * constants::WIDTH + constants::HEIGHT * constants::HEIGHT).sqrt();
    let dist_vector = Location {
        x: body1[0] - body2[0],
        y: body1[1] - body2[1],
    };
    let distance = dist_vector.magnitute();

    [
        (1.0 - distance / max_distance) as f32,
        (distance / max_distance) as f32,
        0.0,
        1.0,
    ]
}

pub fn draw_forces<G: Graphics>(
    forces: &[Box<dyn Force>],
    ball: &ball::Ball,
    draw_state: &DrawState,
    trnsf: [[f64; 3]; 2],
    graphics: &mut G,
) {
    for force in forces {
        line::Line::new(
            utils::color_by_distance(&ball.get_centre(), &force.get_center()),
            2.0,
        )
        .draw_from_to(
            ball.get_centre(),
            force.get_center(),
            draw_state,
            trnsf,
            graphics,
        )
    }
}
