extern crate piston_window;

use piston_window::*;

mod player;
mod utils;

fn main() {

    let mut paddle = player::Paddle{
        position_lower_left: utils::Location{x: 0.0, y: 0.0},
        position_upper_right: utils::Location{x: 50.0, y: 10.0},
        move_direction: utils::Direction::Stationary
    };

    let mut window: PistonWindow =
        WindowSettings::new("Basic Game!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    
    // let event_settings = EventSettings::new().ups(15);
    // let mut events = Events::new(event_settings);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);
            paddle.step();
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [paddle.position_lower_left.x, paddle.position_lower_left.y, paddle.position_upper_right.x, paddle.position_upper_right.y],
                      context.transform,
                      graphics);
        });
        if let Some(args) = event.button_args() {
            if args.state == ButtonState::Press {
                if args.button == Button::Keyboard(Key::Left) {
                    println!("{:?}", paddle);
                    paddle.move_horizontal(utils::Direction::Left); 
                }
                if args.button == Button::Keyboard(Key::Right) {
                    println!("{:?}", paddle);
                    paddle.move_horizontal(utils::Direction::Right); 
                }
            }
            if args.state == ButtonState::Release {
                paddle.move_horizontal(utils::Direction::Stationary);
            }
        }
            
    }
}
