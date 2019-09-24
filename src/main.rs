extern crate sfml;

mod entity;
mod game_state;
mod sf;
mod system;
mod action;

use game_state::GameState;
use action::Action;
use sf::RenderTarget;

// Main
fn main() {
    // MCC constants
    const CIRCLE_SPEED: f32 = 25.0;

    // Initialize game state
    let mut game_state = GameState::new();

    // Game window
    let mut window = sfml::graphics::RenderWindow::new(
        (800, 600),
        "Hello SFML!",
        sf::Style::CLOSE,
        &Default::default(),
    );

    window.set_framerate_limit(60);

    // Game loop
    while window.is_open() {
        // Handle events
        while let Some(e) = window.poll_event() {
            use sf::Event;
            use sf::Key;

            match e {
                // Handle closing of window
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Return, ..
                } => {
                    window.close();
                },

                _ => {
                    // handle player actions
                    if let Some(action) = Action::from_event(&e) {
                        system::handle_player_action(&mut game_state, &action);
                    } else {
                        // Do nothing...
                        ;
                    }
                }
            }
        }

        // Update MCC state
        system::update_physics(&mut game_state);

        // Clear screen
        window.clear(&sf::Color::BLACK);

        // Draw MCC
        // window.draw_circle_shape(
        //     &sf::CircleShape::new(CIRCLE_RADIUS, CIRCLE_POINTS_NUM),
        //     sf::RenderStates {
        //         transform ship.state,
        //         ..sf::RenderStates::default()
        //     },
        // );
        window.draw(&game_state);

        // Display drawn stuff
        window.display();
    }

    println!("Hello, world!");
}
