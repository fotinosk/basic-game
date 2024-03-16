use crate::{ball::Ball, utils, WIDTH};

pub struct Gravity {
    center: utils::Location,
    strength: f64
}

impl Gravity {
    pub fn new() -> Gravity {
        let gr = Gravity{ 
            center: utils::Location { x: WIDTH / 2.0 , y: -100000.0 } ,
            strength: 1.0
        };
        gr
    }
    pub fn excert_force (&self ,ball: &Ball) -> utils::Location {
        let direction = utils::Location{x: ball.position.x - self.center.x , y: ball.position.y - self.center.y};
        direction
    }
}
