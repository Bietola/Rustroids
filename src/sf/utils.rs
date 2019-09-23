use crate::sf;

/// Transform an arrow event into the direction it is poiting to.
pub fn to_dir(e: &sf::Event) -> Option<sf::Vector2f> {
    use sf::Event::*;
    match e {
        KeyPressed { code, .. } => {
            use sf::Key::*;
            Some(match code {
                Up => sf::Vector2f::new(0., 1.),
                Down => sf::Vector2f::new(0., -1.),
                Left => sf::Vector2f::new(-1., 0.),
                Right => sf::Vector2f::new(1., 0.),
                _ => return None,
            })
        }

        _ => None,
    }
}
