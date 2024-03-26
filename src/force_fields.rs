use crate::{ball::Ball, utils, WIDTH};

pub struct Gravity {
    center: utils::Location,
    strength: f64
}

impl Gravity {
    pub fn new() -> Gravity {
        let gr = Gravity{ 
            center: utils::Location { x: WIDTH / 2.0 , y: 100000.0 } ,
            strength: -0.01
        };
        gr
    }
    pub fn excert_force (&self ,ball: &Ball) -> utils::Location {
        let direction = utils::Location{x: ball.position.x - self.center.x , y: ball.position.y - self.center.y};
        let unit_vector = direction.normalize();
        unit_vector.scale(self.strength) 
    }
    pub fn get_center(&self) -> [f64; 2] {
        [self.center.x, self.center.y]
    }
}

pub struct ElectricField {
    center: utils::Location,
    q: f64
}

impl ElectricField {
    pub fn get_center(&self) -> [f64; 2] {
        [self.center.x, self.center.y]
    }
    // TODO: this does not seem to be working as it should be
    pub fn new(x: f64, y: f64) -> ElectricField {
        let em = ElectricField{
            center: utils::Location { x, y },
            q: -150.0
        };
        em
    }
    pub fn excert_force (&self, ball: &Ball) -> utils::Location {
        let direction = utils::Location{x: ball.position.x - self.center.x , y: ball.position.y - self.center.y};
        let dist = direction.magnitute();
        let unit_vector = direction.normalize();
        let factor = self.q / (dist * dist);

        unit_vector.scale(factor)
    }
    
}
