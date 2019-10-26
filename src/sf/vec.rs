extern crate num_traits;

/// The element used inside a vector
const ZERO_VEC_ELE: VecEle = 0.;

/// Radiant unit used to represent angles
type Rads = f32;

/// Type shared by all elements inside Vecs
///
/// NB. This is always supposed to be some kind of float
type VecEle = f32;

/// Geomteric vector of 2 float coordinates
#[derive(Debug, Clone, Copy, Add, Mul, Sub, PartialOrd, PartialEq)]
pub struct Vec2 {
    pub x: VecEle,
    pub y: VecEle,
}

/// Some basic vector operations
impl Vec2 {
    /// Create new sf::Vector2 wrapped in Vec2
    pub fn new(x: VecEle, y: VecEle) -> Self {
        Vec2 { x, y }
    }

    /// Create the Vec2 denoting the origin, i.e. ,)
    pub fn origin() -> Self {
        Self::new(ZERO_VEC_ELE, ZERO_VEC_ELE)
    }

    /// Get unit vector indicating direction from angle expressed in radiants
    fn from_rads(rads: Rads) -> Vec2 {
        Vec2::new(rads.sin(), rads.cos())
    }

    /// Get vector magnitude
    pub fn mag(&self) -> f32 {
        (num::pow(self.x, 2) + num::pow(self.y, 2)).sqrt()
    }

    /// Add to other vector
    pub fn vec_add(&self, other: Vec2) -> Self {
        Vec2::new(other.x + self.x, other.y + self.y)
    }

    /// multiply with given scalar
    pub fn scalar_mul(&self, s: VecEle) -> Self {
        Vec2::new(self.x * s, self.y * s)
    }

    /// Get versor poiting to direction of self
    pub fn dir(self) -> Vec2 {
        let mag = self.mag();

        if mag < std::f32::EPSILON {
            Vec2::new(ZERO_VEC_ELE, ZERO_VEC_ELE)
        } else {
            self * (1. / self.mag())
        }
    }

    /// Return vector with given magnitude without changing its direction
    pub fn change_mag(self, new_mag: VecEle) -> Vec2 {
        self.dir() * new_mag
    }

    /// Return vector with direction as specified and unvaried magnitude
    pub fn change_dir_angle(self, new_dir_angle: Rads) -> Vec2 {
        Vec2::from_rads(new_dir_angle) * self.mag()
    }

    /// Clamp on given quantity
    pub fn clamp_on<F>(self, clamp_value_calculator: F, min: f32, max: f32) -> Vec2
    where
        F: FnOnce(Vec2) -> f32,
    {
        let clamp_value = clamp_value_calculator(self);
        match clamp_value {
            m if m < min => self.change_mag(min),
            m if m > max => self.change_mag(max),
            _ => self,
        }
    }

    /// Clamp magnitude between given range
    pub fn clamp_mag(self, min: f32, max: f32) -> Vec2 {
        match self.mag() {
            m if m < min => self.change_mag(min),
            m if m > max => self.change_mag(max),
            _ => self,
        }
    }

    /// Set vector to if magnitude goes under minimum specified
    pub fn with_min(self, min: f32) -> Vec2 {
        if self.mag() < min {
            Vec2::origin()
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_simple_float_add() {
        assert_eq!(Vec2::new(2., 2.) + Vec2::new(2., 2.), Vec2::new(4., 4.));
    }

    #[test]
    fn vec_simple_scalar_mul() {
        assert_eq!(Vec2::new(2., 2.) * 3., Vec2::new(6., 6.));
    }

    #[test]
    fn vec_dir() {
        use std::f32::EPSILON;
        let epsilon_v = Vec2::new(EPSILON, EPSILON);

        assert!(
            Vec2::new((2f32).sqrt() / 2f32, (2f32).sqrt() / 2f32) - Vec2::new(1f32, 1f32).dir()
                < epsilon_v
        );
        assert!(
            Vec2::new((2f32).sqrt() / 2f32, (2f32).sqrt() / 2f32) - Vec2::new(2f32, 2f32).dir()
                < epsilon_v
        );
        assert!(
            Vec2::new((2f32).sqrt() / 2f32, (2f32).sqrt() / 2f32) - Vec2::new(3., 3.).dir()
                < epsilon_v
        );
        assert!(
            Vec2::new((2f32).sqrt() / 2., (2f32).sqrt() / 2.) - Vec2::new(4., 4.).dir() < epsilon_v
        );
    }

    #[test]
    fn vec_mag() {
        use std::f32::EPSILON;

        assert!(Vec2::new(1., 1.).mag() - (2f32).sqrt() < EPSILON);
        assert!(Vec2::new(2., 2.).mag() - (8f32).sqrt() < EPSILON);
        assert!(Vec2::new(3., 3.).mag() - (18f32).sqrt() < EPSILON);
        assert!(Vec2::new(4., 4.).mag() - (32f32).sqrt() < EPSILON);
    }
}
