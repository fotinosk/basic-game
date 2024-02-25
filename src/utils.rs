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
