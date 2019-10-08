use crate::sf;

/// Transform an arrow key into the direction it is poiting to.
pub fn to_dir(key: sf::Key) -> Option<sf::Vector2f> {
    use sf::Key::*;
    Some(match key {
        Up => sf::Vector2f::new(0., -1.), 
        Down => sf::Vector2f::new(0., 1.),
        Left => sf::Vector2f::new(-1., 0.),
        Right => sf::Vector2f::new(1., 0.),
        _ => sf::Vector2f::new(0., 0.),
    })
}
