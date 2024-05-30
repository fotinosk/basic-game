extern crate piston_window;
use piston_window::*;
use force_fields::Force;

mod block;
mod constants;
mod player;
mod ball;
mod utils;
mod force_fields;


fn main() {
    let mut state = utils::GameState::NotStarted;

    // initialize game objects
    let mut paddle = player::Paddle::new(constants::WIDTH, constants::HEIGHT, constants::OFFSET);
    let mut ball = ball::Ball::new();
    let mut block_grid = block::BlockGrid::new(constants::NUM_BLOCK_ROWS, constants::NUM_BLOCK_COLS);

    // initialize forces here
    let forces: Vec<Box<dyn Force>> = vec![
        Box::new(force_fields::Gravity::new()),
        Box::new(force_fields::ElectricField::new(constants::WIDTH / 2.0, constants::HEIGHT / 2.0)),
    ];

    // initialize gui
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

            paddle.draw(graphics, context.transform);
            block_grid.draw(graphics, context.transform);
            ellipse(constants::PADDDLE_COLOR, [constants::WIDTH / 2.0, constants::HEIGHT / 2.0, 5.0, 5.0], context.transform, graphics);
            ball.draw(graphics, context.transform);  // always draw ball last

            match state {
                // Game over logic
                utils::GameState::GameOver => {
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
                }
                // Main Game Loop
                utils::GameState::InPlay => {
                    let accel = force_fields::sum_forces(&forces, &ball);
                    paddle.step();

                    // Detect ball-block colision here
                    let block_colision = block_grid.step(&ball);
                    // if !matches!(block_colision, block::Collision::NoCollision) {
                    //     block_grid.draw_nearest_block_center(&ball, graphics, context.transform);
                    // }

                    let inplay = ball.step(&paddle, accel);
                    if !inplay {
                        state = utils::GameState::GameOver
                    }
                }
                utils::GameState::Paused => {
                    // utils::draw_forces(&forces, &ball, &context.draw_state, context.transform, graphics);
                    // block_grid.draw_nearest_block_center(&ball, graphics, context.transform);
                }
                _ => {}
            }

            glyphs.factory.encoder.flush(device);
        });

        // Button press processing
        if let Some(args) = event.button_args() {
            if args.state == ButtonState::Press {
                if args.button == Button::Keyboard(Key::Left) && matches!(state, utils::GameState::InPlay) {
                    paddle.move_horizontal(utils::Direction::Left); 
                }
                if args.button == Button::Keyboard(Key::Right) && matches!(state, utils::GameState::InPlay) {
                    paddle.move_horizontal(utils::Direction::Right); 
                }
                if args.button == Button::Keyboard(Key::Space) {
                    state = match state {
                        utils::GameState::Paused | utils::GameState::NotStarted => utils::GameState::InPlay,
                        utils::GameState::InPlay => utils::GameState::Paused,
                        utils::GameState::GameOver => utils::GameState:: GameOver
                    }
                }
                if args.button == Button::Keyboard(Key::Q) {
                    break;
                }
            }
            if args.state == ButtonState::Release {
                // Stop the paddle - otherwise it moves non stop
                paddle.move_horizontal(utils::Direction::Stationary);
            }
        }
    }
}
