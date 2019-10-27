extern crate num_traits;

use num_traits::Zero;

/// Zero element
pub const ZERO_VEC_ELE: VecEle = 0.;

/// Element which is approximately 0
///
/// Mostly used in float comparisons.
pub const EPSILON_VEC_ELE: VecEle = std::f32::EPSILON;

/// Radiant unit used to represent angles
pub type Rads = f32;

/// Type shared by all elements inside Vecs
///
/// NB. This is always supposed to be some kind of float
pub type VecEle = f32;

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
        Vec2::new(rads.cos(), rads.sin())
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

        if mag < EPSILON_VEC_ELE {
            Vec2::new(ZERO_VEC_ELE, ZERO_VEC_ELE)
        } else {
            self * (1. / self.mag())
        }
    }

    /// Get angle associated to direction that vector is poiting to
    ///
    /// NB. Returns `None` if vector is too small (and hence has no direction).
    pub fn dir_angle(self) -> Option<Rads> {
        let mag = self.mag();

        if mag < EPSILON_VEC_ELE {
            None
        } else {
            use std::f32::consts::PI;

            let atan = (self.y / self.x).atan();

            Some(if self.x >= 0. && self.y >= 0. {
                // First quadrant
                atan
            } else if self.x < 0. && self.y >= 0. {
                // Second quadrant
                PI / 2. - atan
            } else if self.x < 0. && self.y < 0. {
                // Third quadrant
                PI + atan
            } else if self.x >= 0. && self.y < 0. {
                // Fourth quadrant
                // NB. since `atan` is negative here, the value of the angle does not exceed
                // 2pi
                2. * PI + atan
            } else {
                // Should never arrive here...
                panic!("Impossible...");
            })
        }
    }

    /// Return vector with given magnitude without changing its direction
    pub fn change_mag(self, new_mag: VecEle) -> Vec2 {
        if self.mag() < EPSILON_VEC_ELE {
            Vec2::new(1., 0.) * new_mag
        } else {
            self.dir() * new_mag
        }
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
        assert!(Vec2::new(1., 1.).mag() - (2f32).sqrt() < EPSILON_VEC_ELE);
        assert!(Vec2::new(2., 2.).mag() - (8f32).sqrt() < EPSILON_VEC_ELE);
        assert!(Vec2::new(3., 3.).mag() - (18f32).sqrt() < EPSILON_VEC_ELE);
        assert!(Vec2::new(4., 4.).mag() - (32f32).sqrt() < EPSILON_VEC_ELE);
    }

    #[test]
    fn dir_angle_always_positive() {
        // Numbers correspond to quadrants
        let angle1 = Vec2::new(1., 1.).dir_angle().unwrap();
        let angle2 = Vec2::new(-1., 1.).dir_angle().unwrap();
        let angle3 = Vec2::new(-1., -1.).dir_angle().unwrap();
        let angle4 = Vec2::new(1., -1.).dir_angle().unwrap();

        // All should be positive angles
        let rads_zero = Rads::zero();
        assert!(
            angle1 > rads_zero && angle2 > rads_zero && angle3 > rads_zero && angle4 > rads_zero
        );
    }

    #[test]
    fn dir_angle_stays_in_quadrant_of_vector() {
        // Numbers correspond to quadrants
        let angle1 = Vec2::new(1., 1.).dir_angle().unwrap();
        let angle2 = Vec2::new(-1., 1.).dir_angle().unwrap();
        let angle3 = Vec2::new(-1., -1.).dir_angle().unwrap();
        let angle4 = Vec2::new(1., -1.).dir_angle().unwrap();

        // All angles should lie in their respective quadrants
        use std::f32::consts::PI;
        assert!(0. < angle1 && angle1 < PI / 2.);
        assert!(PI / 2. < angle2 && angle2 < PI);
        assert!(PI < angle3 && angle3 < 3. * (PI / 2.));
        assert!(3. * (PI / 2.) < angle4 && angle4 < 2. * PI);
    }

    #[test]
    fn change_mag_of_epsilon_vec_sets_default_position() {
        let angle = Vec2::origin().change_mag(1.).dir_angle().unwrap();

        assert_eq!(angle, Rads::zero());
    }
}
