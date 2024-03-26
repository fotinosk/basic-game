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
const MAGN_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

const DT: f64 = 1.0;

fn main() {
    let mut started = false;
    let mut inplay = true;

    let mut paddle = player::Paddle::new(WIDTH, HEIGHT, OFFSET);
    let mut ball = ball::Ball::new(WIDTH, HEIGHT, OFFSET);
    let gravity = force_fields::Gravity::new();
    let e_field = force_fields::ElectricField::new(WIDTH / 2.0, HEIGHT / 2.0);

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
                let elec_accel = e_field.excert_force(&ball);
                let accel = utils::Location{ x: grav_accel.x + elec_accel.x, y: grav_accel.y + elec_accel.y}; 
                paddle.step();
                inplay = ball.step(&paddle, accel);

                // draw line to show gravitational force
                line::Line::new(utils::color_by_distance(&ball.get_centre(), &e_field.get_center()), 2.0).draw_from_to(
                    ball.get_centre(),
                    e_field.get_center(),
                    &context.draw_state,
                    context.transform,
                    graphics
                )
            }
            rectangle(PADDDLE_COLOR, paddle.get_dims(), context.transform, graphics);
            ellipse(PADDDLE_COLOR, ball.get_dims(), context.transform, graphics);

            // TODO: draw perpendicular vector to ensure it's correct
            line::Line::new(
                utils::color_by_distance(
                    &paddle.get_centre(), &ball.get_centre()
                ), 2.0).draw_from_to(
                    paddle.get_centre(), 
                    ball.get_centre(), 
                    &context.draw_state, 
                    context.transform, 
                    graphics
                );

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
