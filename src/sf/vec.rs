extern crate num_traits;

use std::ops::{Add, Mul};

use num_traits::ops::inv::Inv;
use num_traits::Float;

use crate::sf;

/// Elements of vectors must satisfy this trait
pub trait VecEle: Copy + Mul<Output = Self> + Add<Output = Self> {}

/// VecEle trivial implementations
// TODO: find a quicker way of doing this
impl VecEle for f32 {}
impl VecEle for f64 {}
impl VecEle for i32 {}
impl VecEle for i64 {}

/// Newype for sf::Vector
#[derive(Debug, Clone, Copy, Add, PartialEq)]
pub struct Vec2<T: VecEle>(pub sf::Vector2<T>);

/// Some basic vector operations
impl<T: VecEle> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2(sf::Vector2::new(x, y))
    }

    pub fn abs(&self) -> T {
        use num::{pow, sqrt};
        pow(self.0.x, 2)
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

/// Vector addition with sf vector
impl<T: VecEle> Add<sf::Vector2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: sf::Vector2<T>) -> Self::Output {
        Vec2::new(rhs.x + self.0.x, rhs.y + self.0.y)
    }
}

/// Vector multiplication
// TODO: find a way of expressing this with derive
impl<T: VecEle> Mul for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(rhs.0.x * self.0.x, rhs.0.y * self.0.y)
    }
}

/// Vector multiplication with sf vector
// TODO: find a way of expressing this with derive
impl<T: VecEle> Mul<sf::Vector2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: sf::Vector2<T>) -> Self::Output {
        Vec2::new(rhs.x * self.0.x, rhs.y * self.0.y)
    }
}

/// Scalar multiplication
impl<T: VecEle> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, s: T) -> Self::Output {
        Vec2::new(s * self.0.x, s * self.0.y)
    }
}

/// Return versor indicating direction of given vector
pub fn dir<T>(v: &sf::Vector2<T>) -> sf::Vector2<T>
where
    T: Float + Inv<Output = T>,
{
    scalar_mul(magnitude(&v).inv(), &v)
}

/// Clamp vector magnitude
pub fn clamp_magnitude<T>(v: sf::Vector2<T>, min: T, max: T) -> sf::Vector2<T>
where
    T: Float,
{
    match magnitude(&v) {
        m if m < min => change_magnitude(v, min),
        m if m > max => {
            change_magnitude(v, max)
        }
        _ => v,
    }
}

/// Change magnitude of vector
pub fn change_magnitude<T>(v: sf::Vector2<T>, new_mag: T) -> sf::Vector2<T>
where
    T: Float,
{
    scalar_mul(new_mag / magnitude(&v), &v)
}

/// Calculate magnitude of vector
pub fn magnitude<T>(v: &sf::Vector2<T>) -> T
where
    T: Float,
{
    (num::pow(v.x, 2) + num::pow(v.y, 2)).sqrt()
}

/// Calculate multiplication between vector and scalar
pub fn scalar_mul<T, U>(s: T, v: &sf::Vector2<U>) -> sf::Vector2<U>
where
    U: Mul<Output = U> + Copy,
    T: Into<U> + Copy,
{
    sf::Vector2::new(s.into() * v.x, s.into() * v.y)
}

/// Add two vectors together
pub fn add<T>(lhs: &sf::Vector2<T>, rhs: &sf::Vector2<T>) -> sf::Vector2<T>
where
    T: Float,
{
    sf::Vector2::new(lhs.x + rhs.x, lhs.y + rhs.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_simple_int_add() {
        assert_eq!(Vec2::new(2, 2) + Vec2::new(2, 2), Vec2::new(4, 4));
    }

    #[test]
    fn vec_simple_float_add() {
        assert_eq!(Vec2::new(2., 2.) + Vec2::new(2., 2.), Vec2::new(4., 4.));
    }
}
