use crate::{ball::Ball, constants, utils};

pub trait Force {
    fn excert_force(&self, ball: &Ball) -> utils::Location;
    fn get_center(&self) -> [f64; 2];
    fn print_name(&self);
}

pub struct Gravity {
    center: utils::Location,
    strength: f64,
}

impl Gravity {
    pub fn new() -> Gravity {
        let gr = Gravity {
            center: utils::Location {
                x: constants::WIDTH / 2.0,
                y: 100000.0,
            },
            strength: constants::GRAVITATIONAL_STRENGTH,
        };
        gr
    }
}

impl Force for Gravity {
    fn excert_force(&self, ball: &Ball) -> utils::Location {
        let direction = utils::Location {
            x: ball.position.x - self.center.x,
            y: ball.position.y - self.center.y,
        };
        let unit_vector = direction.normalize();
        unit_vector.scale(self.strength)
    }
    fn get_center(&self) -> [f64; 2] {
        [self.center.x, self.center.y]
    }
    fn print_name(&self) {
        println!("Gravity")
    }
}

pub struct ElectricField {
    center: utils::Location,
    q: f64,
}

impl ElectricField {
    pub fn new(x: f64, y: f64, attractive: bool) -> ElectricField {
        let mult: f64;
        match attractive {
            true => {mult = 1.0;},
            false => {mult = -1.0;}
        }
        let em = ElectricField {
            center: utils::Location { x, y },
            q: constants::ELECTRIC_FIELD_STRENGTH * mult,
        };
        em
    }
}

impl Force for ElectricField {
    fn get_center(&self) -> [f64; 2] {
        [self.center.x, self.center.y]
    }

    fn excert_force(&self, ball: &Ball) -> utils::Location {
        let direction = utils::Location {
            x: ball.position.x - self.center.x,
            y: ball.position.y - self.center.y,
        };
        let dist = direction.magnitute();
        let unit_vector = direction.normalize();
        let factor = self.q / (dist * dist);

        unit_vector.scale(factor)
    }

    fn print_name(&self) {
        println!("EM")
    }
}

pub fn sum_forces(forces: &[Box<dyn Force>], ball: &Ball) -> utils::Location {
    let mut total_force = utils::Location { x: 0.0, y: 0.0 };

    for f in forces {
        let force = f.excert_force(ball);
        total_force.x += force.x;
        total_force.y += force.y;
    }
    total_force
}

pub fn sum_block_forces(forces: &Vec<ElectricField>, ball: &Ball) -> utils::Location {
    let mut total_force = utils::Location { x: 0.0, y: 0.0 };

    for f in forces {
        let force = f.excert_force(ball);
        total_force.x += force.x;
        total_force.y += force.y;
    }
    total_force
}
