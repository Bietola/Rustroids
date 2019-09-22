extern crate sfml;

mod sf;

use sf::RenderTarget;

// Spaceship
struct Spaceship {
    texture: sf::Texture,
    state: sf::Transform,
    vel: sf::Vector2f,
}

impl Spaceship {
    const SPRITE_SIZE: u32 = 100;

    fn new() -> Self {
        let texture = sf::Texture::from_file("assets/ship.png").expect("Could not load spaceship texture");
        let tsize = texture.size();

        Self {
            texture,
            state: sf::transform::scale(
                sf::Transform::IDENTITY,
                Self::SPRITE_SIZE as f32 / tsize.x as f32,
                Self::SPRITE_SIZE as f32 / tsize.y as f32
            ),
            vel: sf::Vector2::new(0.0, 0.0),
        }
    }

    fn update(&mut self) {
        self.state.translate(self.vel.x, self.vel.y);
    }
}

impl sf::Drawable for Spaceship {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut sf::RenderTarget,
        states: sf::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_with_renderstates(
            &sf::Sprite::with_texture(&self.texture),
            sf::RenderStates {
                transform: sf::transform::combine(self.state, states.transform),

                ..states
            },
        );
    }
}

// Main
fn main() {
    // MCC constants
    const CIRCLE_SPEED: f32 = 25.0;

    // Initialize player spaceship
    let mut ship = Spaceship::new();

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
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Return, ..
                } => {
                    window.close();
                }
                Event::KeyPressed { code: Key::Up, .. } => {
                    ship.vel = sf::Vector2f::new(0.0, -CIRCLE_SPEED);
                }
                Event::KeyPressed {
                    code: Key::Down, ..
                } => {
                    ship.vel = sf::Vector2f::new(0.0, CIRCLE_SPEED);
                }
                Event::KeyPressed {
                    code: Key::Left, ..
                } => {
                    ship.vel = sf::Vector2f::new(-CIRCLE_SPEED, 0.0);
                }
                Event::KeyPressed {
                    code: Key::Right, ..
                } => {
                    ship.vel = sf::Vector2f::new(CIRCLE_SPEED, 0.0);
                }
                Event::KeyPressed { code, .. } => println!("{:?}", code),
                _ => {}
            }
        }

        // Update MCC state
        ship.update();

        // Clear screen
        window.clear(&sf::Color::BLACK);

        // Draw MCC
        // window.draw_circle_shape(
        //     &sf::CircleShape::new(CIRCLE_RADIUS, CIRCLE_POINTS_NUM),
        //     sf::RenderStates {
        //         transform: ship.state,
        //         ..sf::RenderStates::default()
        //     },
        // );
        window.draw(&ship);

        // Display drawn stuff
        window.display();
    }

    println!("Hello, world!");
}
