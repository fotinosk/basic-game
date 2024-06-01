extern crate piston_window;
use force_fields::Force;
use piston_window::*;

mod ball;
mod block;
mod constants;
mod force_fields;
mod player;
mod utils;

fn main() {
    let mut state = utils::GameState::NotStarted;

    // initialize game objects
    let mut paddle = player::Paddle::new(constants::WIDTH, constants::HEIGHT, constants::OFFSET);
    let mut ball = ball::Ball::new();
    let mut block_grid =block::BlockGrid::new(constants::NUM_BLOCK_ROWS, constants::NUM_BLOCK_COLS);

    // initialize forces here
    let forces: Vec<Box<dyn Force>> = vec![
        Box::new(force_fields::Gravity::new()),
    ];
    let mut block_forces = block_grid.get_forces();

    // initialize gui
    let mut window: PistonWindow =
        WindowSettings::new("Basic Game!", [constants::WIDTH, constants::HEIGHT])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

    let mut events = Events::new((|| {
        let mut settings = EventSettings::new();
        settings.ups = 360;
        settings.max_fps = 360;
        settings
    })());

    let font = include_bytes!("assets/FiraSans-Bold.ttf");
    let mut glyphs = Glyphs::from_bytes(
        font,
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .unwrap();

    while let Some(event) = events.next(&mut window) {
        window.draw_2d(&event, |context, graphics, device| {
            clear([0.0; 4], graphics);

            paddle.draw(graphics, context.transform);
            block_grid.draw(graphics, context.transform);
            ball.draw(graphics, context.transform); // always draw ball last

            match state {
                utils::GameState::GameOver => {
                    // Game over logic
                    let _ = text(
                        constants::PADDDLE_COLOR,
                        28,
                        "GAME OVER",
                        &mut glyphs,
                        context
                            .transform
                            .trans(constants::HEIGHT / 2.0, constants::WIDTH / 2.0 - 100.0),
                        graphics,
                    );
                    let _ = text(
                        constants::PADDDLE_COLOR,
                        20,
                        " Press Q to exit", // a very elegant solution to centering text
                        &mut glyphs,
                        context
                            .transform
                            .trans(constants::HEIGHT / 2.0, constants::WIDTH / 2.0 - 50.0),
                        graphics,
                    );
                }
                utils::GameState::InPlay => {
                    // Main Game Loop
                    block_forces = block_grid.get_forces();
                    let accel = force_fields::sum_forces(&forces, &ball);
                    let block_accel = force_fields::sum_block_forces(&block_forces, &ball);
                    let total_accel = utils::Location{
                        x: accel.x + block_accel.x,
                        y: accel.y + block_accel.y,
                    };
                    paddle.step();

                    // Detect ball-block colision here
                    let block_collision = block_grid.step(&ball);
                    // let inplay = ball.step(&paddle, accel, block_collision);
                    let inplay = ball.step(&paddle, total_accel, block_collision);

                    if !inplay {
                        state = utils::GameState::GameOver
                    }
                    if block_grid.active_blocks == 0 {
                        state = utils::GameState::Finished
                    }
                }
                utils::GameState::Paused => {
                    utils::draw_forces(&forces, &ball, &context.draw_state, context.transform, graphics);
                    utils::draw_block_forces(&block_forces, &ball, &context.draw_state, context.transform, graphics);
                }
                utils::GameState::Finished => {
                    // Win Screen
                    let _ = text(
                        constants::PADDDLE_COLOR,
                        28,
                        "WINNER WINNER, CHICKEN DINNER!",
                        &mut glyphs,
                        context
                            .transform
                            .trans(constants::WIDTH / 2.0 - 250.0, constants::HEIGHT / 2.0),
                        graphics,
                    );
                }
                _ => {}
            }

            glyphs.factory.encoder.flush(device);
        });

        // Button press processing
        if let Some(args) = event.button_args() {
            if args.state == ButtonState::Press {
                if args.button == Button::Keyboard(Key::Left)
                    && matches!(state, utils::GameState::InPlay)
                {
                    paddle.move_horizontal(utils::Direction::Left);
                }
                if args.button == Button::Keyboard(Key::Right)
                    && matches!(state, utils::GameState::InPlay)
                {
                    paddle.move_horizontal(utils::Direction::Right);
                }
                if args.button == Button::Keyboard(Key::Space) {
                    state = match state {
                        utils::GameState::Paused | utils::GameState::NotStarted => {
                            utils::GameState::InPlay
                        }
                        utils::GameState::InPlay => utils::GameState::Paused,
                        utils::GameState::GameOver => utils::GameState::GameOver,
                        utils::GameState::Finished => utils::GameState::Finished,
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
