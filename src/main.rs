extern crate piston_window;

use piston_window::*;

mod player;
mod ball;
mod utils;
mod force_fields;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;

const OFFSET: f64 = 20.0;
const PADDDLE_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

const DT: f64 = 1.0;

fn main() {
    let mut started = false;
    let mut inplay = true;

    let mut paddle = player::Paddle::new(WIDTH, HEIGHT, OFFSET);
    let mut ball = ball::Ball::new(WIDTH, HEIGHT, OFFSET);
    let gravity = force_fields::Gravity::new();

    let mut window: PistonWindow =
        WindowSettings::new("Basic Game!", [WIDTH, HEIGHT])
        .exit_on_esc(true).resizable(false).build().unwrap();
    
    let mut events = Events::new(
        (||{
            let mut settings = EventSettings::new();
            settings.ups = 160;
            settings.max_fps = 160;
            settings
        })()
    );    

    let font = include_bytes!("assets/FiraSans-Bold.ttf");
    let mut glyphs = Glyphs::from_bytes(
        font,
        window.create_texture_context(),
        TextureSettings::new(),
    ).unwrap();

    while let Some(event) = events.next(&mut window) {
        window.draw_2d(&event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            if started {
                let grav_accel = gravity.excert_force(&ball);
                paddle.step();
                inplay = ball.step(&paddle, grav_accel);
            }
            rectangle(PADDDLE_COLOR, paddle.get_dims(), context.transform, graphics);
            ellipse(PADDDLE_COLOR, ball.get_dims(), context.transform, graphics);

            if !inplay {
                let _ = text(
                    PADDDLE_COLOR, 
                    28, 
                    "GAME OVER", 
                    &mut glyphs, 
                    context.transform.trans(HEIGHT/2.0, WIDTH/2.0 - 100.0), 
                    graphics
                );
                let _ = text(
                    PADDDLE_COLOR, 
                    20, 
                    " Press Q to exit",  // a very elegant solution to centering text 
                    &mut glyphs, 
                    context.transform.trans(HEIGHT/2.0, WIDTH/2.0 - 50.0), 
                    graphics
                );

            }

            glyphs.factory.encoder.flush(device);
        });

        if !inplay {
            started = false;
        }

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
                if args.button == Button::Keyboard(Key::Q) {
                    // started = false;
                    break;
                }
            }
            if args.state == ButtonState::Release {
                paddle.move_horizontal(utils::Direction::Stationary);
            }
        }
            
    }
}
