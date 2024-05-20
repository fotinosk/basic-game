extern crate piston_window;
use piston_window::*;
use force_fields::Force;

mod ui;
mod block;
mod constants;
mod player;
mod ball;
mod utils;
mod force_fields;


fn main() {
    let mut started = false;
    let mut inplay = true;

    let mut paddle = player::Paddle::new(constants::WIDTH, constants::HEIGHT, constants::OFFSET);
    let mut ball = ball::Ball::new(constants::WIDTH, constants::HEIGHT, constants::OFFSET);

    // initialize forces here
    let forces: Vec<Box<dyn Force>> = vec![
        Box::new(force_fields::Gravity::new()),
        Box::new(force_fields::ElectricField::new(constants::WIDTH / 2.0, constants::HEIGHT / 2.0)),
    ];

    let mut window: PistonWindow =
        WindowSettings::new("Basic Game!", [constants::WIDTH, constants::HEIGHT])
        .exit_on_esc(true).resizable(false).build().unwrap();
    
    let mut events = Events::new(
        (||{
            let mut settings = EventSettings::new();
            settings.ups = 360;
            settings.max_fps = 360;
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
                utils::draw_forces(&forces, &ball, &context.draw_state, context.transform, graphics);
                let accel = force_fields::sum_forces(&forces, &ball);
                paddle.step();
                inplay = ball.step(&paddle, accel);
            }
            paddle.draw(graphics, context.transform);
            ball.draw(graphics, context.transform);
            ellipse(constants::PADDDLE_COLOR, [constants::WIDTH / 2.0, constants::HEIGHT / 2.0, 5.0, 5.0], context.transform, graphics);

            if !inplay {
                let _ = text(
                    constants::PADDDLE_COLOR, 
                    28, 
                    "GAME OVER", 
                    &mut glyphs, 
                    context.transform.trans(constants::HEIGHT/2.0, constants::WIDTH/2.0 - 100.0), 
                    graphics
                );
                let _ = text(
                    constants::PADDDLE_COLOR, 
                    20, 
                    " Press Q to exit",  // a very elegant solution to centering text 
                    &mut glyphs, 
                    context.transform.trans(constants::HEIGHT/2.0, constants::WIDTH/2.0 - 50.0), 
                    graphics
                );
                started = false;
            }

            glyphs.factory.encoder.flush(device);
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
                    started = !started;
                }
                if args.button == Button::Keyboard(Key::Q) {
                    break;
                }
            }
            if args.state == ButtonState::Release {
                paddle.move_horizontal(utils::Direction::Stationary);
            }
        }
    }
}
