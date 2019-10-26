use crate::sf;

/// Transform an arrow key into an angle indicating the direction it is poiting to.
/// Returns `None` if the key does not indicate a direction
pub fn to_angle(key: sf::Key) -> Option<f32> {
    use sf::Key::*;
    use std::f32::consts::PI;
    Some(match key {
        Up => (3. / 2.) * PI,
        Down => (1. / 2.) * PI,
        Left => PI,
        Right => 0.,
        _ => return None,
    })
}
