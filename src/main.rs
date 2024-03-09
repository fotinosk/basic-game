extern crate piston_window;

use piston_window::*;

mod player;
mod ball;
mod utils;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;

const OFFSET: f64 = 20.0;
const PADDDLE_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

const DT: f64 = 1.0;

fn main() {

    let mut paddle = player::Paddle::new(WIDTH, HEIGHT, OFFSET);
    let mut ball = ball::Ball::new(WIDTH, HEIGHT, OFFSET);

    let mut window: PistonWindow =
        WindowSettings::new("Basic Game!", [WIDTH, HEIGHT])
        .exit_on_esc(true).resizable(false).build().unwrap();
    
    // let event_settings = EventSettings::new().ups(15);
    // let mut events = Events::new(event_settings);
    
    let mut started = false;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);
            if started {
                paddle.step();
                ball.step();
            }
            rectangle(PADDDLE_COLOR,
                      paddle.get_dims(),
                      context.transform,
                      graphics);

            ellipse(PADDDLE_COLOR, ball.get_dims(), context.transform, graphics);
        });

        if let Some(args) = event.button_args() {
            if args.state == ButtonState::Press {
                if args.button == Button::Keyboard(Key::Left) && started {
                    paddle.move_horizontal(utils::Direction::Left); 
                }
                if args.button == Button::Keyboard(Key::Right) && started {
                    paddle.move_horizontal(utils::Direction::Right); 
                }
                if args.button == Button::Keyboard(Key::Space) {
                    started = true;
                }
            }
            if args.state == ButtonState::Release {
                paddle.move_horizontal(utils::Direction::Stationary);
            }
        }
            
    }
}
