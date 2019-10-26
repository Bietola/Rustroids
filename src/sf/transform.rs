use crate::sf;

/// Combine two sfml transforms
pub fn combine(mut lhs: sf::Transform, mut rhs: sf::Transform) -> sf::Transform {
    lhs.combine(&mut rhs);

    lhs
}

/// Scale transform by scale_x and scale_y
pub fn scale(to_scale: sf::Transform, scale_x: f32, scale_y: f32) -> sf::Transform {
    let mut to_scale = to_scale;
    to_scale.scale(scale_x, scale_y);
    to_scale
}
