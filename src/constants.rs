pub const WIDTH: f64 = 1000.0;
pub const HEIGHT: f64 = 900.0;

pub const OFFSET: f64 = 20.0;
pub const PADDLE_FRICTION_COMPONENT: f64 = 1.0;
pub const PADDLE_VELOCITY_INJECTION: f64 = 2.0;  // to avoid getting stuck give energy with every bounce

pub const DT: f64 = 1.0;

pub const BALLRADIUS: f64 = 10.0;
pub const INIT_BALL_SPEED_X: f64 = 2.0;
pub const INIT_BALL_SPEED_Y: f64 = -3.0;
pub const MAX_BALL_SPEED: f64 = 10.0;

pub const GRAVITATIONAL_STRENGTH: f64 = -0.005;
pub const ELECTRIC_FIELD_STRENGTH: f64 = -100.0;

pub const SPEED: f64 = 10.0; // Paddle speed
pub const PADDLE_LEN: f64 = 180.0;
pub const PADDLE_HEIGHT: f64 = 10.0;

pub const BLOCK_WIDTH: f64 = 80.0;
pub const BLOCK_HEIGHT: f64 = 40.0;
pub const NUM_BLOCK_COLS: u8 = 10;
pub const NUM_BLOCK_ROWS: u8 = 8;

// colors
pub const BALL_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const PADDDLE_COLOR: [f32; 4] = [0.95, 0.33, 0.33, 1.0];
pub const LETTER_COLOR: [f32; 4] = [0.45, 0.8, 0.35, 1.0];
pub const UNCHARGED_BLOCK_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const POSITIVE_BLOCK_COLOR: [f32; 4] = [0.2, 0.9, 0.5, 1.0];
pub const NEGATIVE_BLOCK_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub const VERTICAL_BLOCK_OFFSET: f64 = 60.0;
pub const BLOCK_VISIBILITY_MARGIN: f64 = BALLRADIUS / 2.0; // invisible border around block

