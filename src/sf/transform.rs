use crate::sf;

/// Combine two sfml transforms
/// TODO: WIP
pub fn combine(lhs: sf::Transform, rhs: sf::Transform) -> sf::Transform {
    lhs
}

/// Scale transform by scale_x and scale_y
pub fn scale(to_scale: sf::Transform, scale_x: f32, scale_y: f32) -> sf::Transform {
    let mut to_scale = to_scale;
    to_scale.scale(scale_x, scale_y);
    to_scale
}
