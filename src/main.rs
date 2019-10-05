#[macro_use] extern crate bitflags;
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
    // Initialize game state
    let mut game_state = GameState::new();

    // Game window setup
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

                // Try to handle other types of events as player actions
                _ => {
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

        // Clear screen and render game
        window.clear(&sf::Color::BLACK);
        window.draw(&game_state);
        window.display();
    }
}
