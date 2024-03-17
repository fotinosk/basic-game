#[derive(Debug)]
pub struct Location {
    pub x: f64,
    pub y: f64
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Stationary
}

impl Location {
    pub fn normalize(&self) ->  Location {
        let magn = (self.x * self.x + self.y * self.y).sqrt();
        Location{x : self.x / magn, y: self.y /magn}
    }
    pub fn scale(&self, factor: f64) -> Location {
        Location{x : self.x * factor, y: self.y * factor}
    }
}
