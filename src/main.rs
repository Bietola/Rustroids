#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
extern crate simplelog;
#[macro_use]
extern crate derive_more;

extern crate sfml;

mod action;
mod entity;
mod game_state;
mod sf;
mod system;

use simplelog::*;

use action::Action;
use game_state::GameState;
use sf::RenderTarget;

// Main
fn main() {
    // Initialize logging system
    let _ = SimpleLogger::init(LevelFilter::Debug, Config::default());

    // Initialize game state
    let mut game_state = GameState::new();

    // Game window setup
    let mut window = sfml::graphics::RenderWindow::new(
        (800, 600),
        "Hello SFML!",
        sf::Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(30);

    // Game loop
    while window.is_open() {
        // Start iteration timer
        //  NB. this is started before the rendering so that the physics system receives
        //      a reasonable delta to work with
        let game_iteration_timer = sf::Clock::start();

        // Clear screen and render game
        window.clear(&sf::Color::BLACK);
        window.draw(&game_state);
        window.display();

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
                }

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
        // TODO: change constant timestamp to actual delta
        system::physics::update_all(&mut game_state, game_iteration_timer.elapsed_time());
    }
}
