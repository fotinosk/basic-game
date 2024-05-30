pub const WIDTH: f64 = 800.0;
pub const HEIGHT: f64 = 640.0;

pub const OFFSET: f64 = 20.0;
pub const PADDDLE_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub const DT: f64 = 1.5;

pub const BALLRADIUS: f64 = 10.0;
pub const BALL_COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const INIT_BALL_SPEED_X: f64 = 3.0;
pub const INIT_BALL_SPEED_Y: f64 = -3.0;

pub const GRAVITATIONAL_STRENGTH: f64 = -0.0005;
pub const ELECTRIC_FIELD_STRENGTH: f64 = -1000.0;

pub const SPEED: f64 = 6.0;
// pub const PADDLE_LEN: f64 = 80.0;
pub const PADDLE_LEN: f64 = 800.0;
pub const PADDLE_HEIGHT: f64 = 10.0;

pub const BLOCK_WIDTH: f64 = 40.0;
pub const BLOCK_HEIGHT: f64 = 20.0;
pub const NUM_BLOCK_COLS: u8 = 15; 
pub const NUM_BLOCK_ROWS: u8 = 16; 
pub const UNCHARGED_BLOCK_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 0.9];
pub const POSITIVE_BLOCK_COLOR: [f32; 4] = [1.0, 0.0, 1.0, 0.9];
pub const NEGATIVE_BLOCK_COLOR: [f32; 4] = [0.0, 1.0, 1.0, 0.9];

pub const VERTICAL_BLOCK_OFFSET: f64 = 60.0;
pub const BLOCK_VISIBILITY_MARGIN: f64 = 2.0;  // invisible border around block