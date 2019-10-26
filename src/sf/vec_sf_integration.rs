use crate::sf;
use crate::sf::vec;

/// Convert Vec2 into sfml compatible equivalent
impl vec::Vec2 {
    /// Convert Vec2 into sfml compatible equivalent
    fn into_sf_vec(self) -> sf::Vector2<vec::VecEle> {
        sf::Vector2::new(self.x, self.y)
    }
}
