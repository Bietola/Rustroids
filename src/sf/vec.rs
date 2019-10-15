extern crate num_traits;

use std::ops::{Add, Mul};

use num_traits::{FromPrimitive, Zero, One, ToPrimitive};

use crate::sf;

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
}
impl<T> VecEle for T where
    T: ToPrimitive
        + FromPrimitive
        + PartialOrd
        + One
        + Zero
        + Copy
        + Clone
        + Mul<Output = Self>
        + Add<Output = Self>
{
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

    /// Get vector magnitude
    pub fn mag(&self) -> f32 {
        (num::pow(self.0.x, 2) + num::pow(self.0.y, 2))
            .to_f32()
            .expect("Failed in calculating magnitude!")
            .sqrt()
    }

    /// Get versor poiting to direction of self
    pub fn dir(&self) -> Vec2<T> {
        let mag = self.mag();

        if mag < std::f32::EPSILON {
            Vec2::new(T::zero(), T::zero())
        } else {
            self * T::from_f32(1. / self.mag()).unwrap()
        }
    }

    /// Return vector with given magnitude without changing its direction
    pub fn change_mag(&self, new_mag: T) -> Vec2<T> {
        &self.dir() * new_mag
    }

    /// Clamp magnitude between given range
    pub fn clamp_mag(&self, min: f32, max: f32) -> Vec2<T> {
        match self.mag() {
            m if m < min => self.change_mag(T::from_f32(min).unwrap()),
            m if m > max => self.change_mag(T::from_f32(max).unwrap()),
            _ => self.clone(),
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
impl<T: VecEle> AsRef<sf::Vector2<T>> for Vec2<T> {
    fn as_ref(&self) -> &sf::Vector2<T> {
        &self.0
    }
}

/// Vector addition with sf vector
impl<T: VecEle> Add<sf::Vector2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: sf::Vector2<T>) -> Self::Output {
        Vec2::new(rhs.x + self.0.x, rhs.y + self.0.y)
    }
}

/// Vector multiplication
// TODO: find a way of expressing this with derive
impl<T: VecEle> Mul for &Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2::new(rhs.0.x * self.0.x, rhs.0.y * self.0.y)
    }
}

/// Vector multiplication with sf vector
// TODO: find a way of expressing this with derive
impl<T: VecEle> Mul<&sf::Vector2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: &sf::Vector2<T>) -> Self::Output {
        Vec2::new(rhs.x * self.0.x, rhs.y * self.0.y)
    }
}

/// Scalar multiplication
impl<T: VecEle> Mul<T> for &Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, s: T) -> Self::Output {
        Vec2::new(s * self.0.x, s * self.0.y)
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
        assert_eq!(&Vec2::new(2., 2.) * 3., Vec2::new(6., 6.));
    }

    #[test]
    fn vec_dir() {
        use std::f32::EPSILON;
        let epsilon_v = Vec2::new(EPSILON, EPSILON);

        assert!(
            Vec2::new((2f32).sqrt() / 2f32, (2f32).sqrt() / 2f32) - Vec2::new(1f32, 1f32).dir() < epsilon_v
        );
        assert!(
            Vec2::new((2f32).sqrt() / 2f32, (2f32).sqrt() / 2f32) - Vec2::new(2f32, 2f32).dir() < epsilon_v
        );
        assert!(
            Vec2::new((2f32).sqrt() / 2f32, (2f32).sqrt() / 2f32) - Vec2::new(3., 3.).dir() < epsilon_v
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
