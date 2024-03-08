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

#[derive(Debug)]
pub struct Velocity {
    // x and y are unit vectors
    pub x: f64,
    pub y: f64, 
    pub magn: f64
}
