extern crate num_traits;

use std::ops::{Add, Mul};

use num_traits::{Float, FromPrimitive, One, ToPrimitive, Zero};

use crate::sf;

/// Radiant unit used to represent angles
type Rads = f32;

/// Trait alias for traits that all vector elements must satisfy
// TODO: make a macro for this
pub trait VecEle:
    ToPrimitive
    + FromPrimitive
    + PartialOrd
    + One
    + Zero
    + Copy
    + Clone
    + Mul<Output = Self>
    + Add<Output = Self>
{
    /// Convert from radiants
    fn from_rads(rads: Rads) -> Option<Self>;
}
impl<T> VecEle for T
where
    T: ToPrimitive
        + FromPrimitive
        + PartialOrd
        + One
        + Zero
        + Copy
        + Clone
        + Mul<Output = Self>
        + Add<Output = Self>,
{
    /// Convert from radiants
    fn from_rads(rads: Rads) -> Option<T> {
        T::from_f32(rads)
    }
}

/// Newype for sf::Vector
#[derive(Debug, Clone, Copy, Add, Sub, PartialOrd, PartialEq)]
pub struct Vec2<T: VecEle>(pub sf::Vector2<T>);

/// Some basic vector operations
impl<T: VecEle> Vec2<T> {
    /// Create new sf::Vector2 wrapped in Vec2
    pub fn new(x: T, y: T) -> Self {
        Vec2(sf::Vector2::new(x, y))
    }

    /// Create the Vec2 denoting the origin, i.e. (0, 0)
    pub fn origin() -> Self {
        Self::new(T::zero(), T::zero())
    }

    /// Get unit vector indicating direction from angle expressed in radiants
    fn from_rads(rads: Rads) -> Vec2<T> {
        Vec2::new(
            T::from_rads(rads.sin()).unwrap(),
            T::from_rads(rads.cos()).unwrap(),
        )
    }

    /// Get vector magnitude
    pub fn mag(&self) -> f32 {
        (num::pow(self.0.x, 2) + num::pow(self.0.y, 2))
            .to_f32()
            .expect("Failed in calculating magnitude!")
            .sqrt()
    }

    /// Add to other vector
    pub fn vec_add(&self, other: Vec2<T>) -> Self {
        Vec2::new(other.0.x + self.0.x, other.0.y + self.0.y)
    }

    /// multiply with given scalar
    pub fn scalar_mul(&self, s: T) -> Self {
        Vec2::new(self.0.x * s, self.0.y * s)
    }

    /// Get versor poiting to direction of self
    pub fn dir(self) -> Vec2<T> {
        let mag = self.mag();

        if mag < std::f32::EPSILON {
            Vec2::new(T::zero(), T::zero())
        } else {
            self * T::from_f32(1. / self.mag()).unwrap()
        }
    }

    /// Return vector with given magnitude without changing its direction
    pub fn change_mag(self, new_mag: T) -> Vec2<T> {
        self.dir() * new_mag
    }

    /// Return vector with direction as specified and unvaried magnitude
    pub fn change_dir_angle(self, new_dir_angle: Rads) -> Vec2<T> {
        Vec2::from_rads(new_dir_angle) * self.mag()
    }

    /// Clamp on given quantity
    pub fn clamp_on<F>(self, clamp_value_calculator: F, min: f32, max: f32) -> Vec2<T>
    where
        F: FnOnce(Vec2<T>) -> f32,
    {
        let clamp_value = clamp_value_calculator(self);
        match clamp_value {
            m if m < min => self.change_mag(T::from_f32(min).unwrap()),
            m if m > max => self.change_mag(T::from_f32(max).unwrap()),
            _ => self,
        }
    }

    /// Clamp magnitude between given range
    pub fn clamp_mag(self, min: f32, max: f32) -> Vec2<T> {
        match self.mag() {
            m if m < min => self.change_mag(T::from_f32(min).unwrap()),
            m if m > max => self.change_mag(T::from_f32(max).unwrap()),
            _ => self,
        }
    }

    /// Set vector to 0 if magnitude goes under minimum specified
    pub fn with_min(self, min: f32) -> Vec2<T> {
        if self.mag() < min {
            Vec2::origin()
        } else {
            self
        }
    }
}

/// From impl a la newtype
impl<T: VecEle> From<sf::Vector2<T>> for Vec2<T> {
    fn from(v: sf::Vector2<T>) -> Self {
        Vec2::new(v.x, v.y)
    }
}

/// From impl a la newtype
impl<T: VecEle> Into<sf::Vector2<T>> for Vec2<T> {
    fn into(self) -> sf::Vector2<T> {
        self.0
    }
}

/// Vector multiplication
// TODO: find a way of expressing this with derive
impl<T: VecEle> Mul for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        self.vec_add(rhs)
    }
}

/// Scalar multiplication
impl<T: VecEle> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, s: T) -> Self::Output {
        self.scalar_mul(s)
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

    #[test]
    fn sf_vec_into_vec() {
        assert_eq!(Vec2(sf::Vector2f::new(1., 1.)), Vec2::new(1., 1.));
    }
}
