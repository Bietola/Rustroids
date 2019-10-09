extern crate num_traits;

use std::ops::{Add, Mul};

use num_traits::Float;
use num_traits::ops::inv::Inv;

use crate::sf;

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
            debug!("CLAMPING VEL!");
            change_magnitude(v, max)
        },
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
    use num::pow;
    (pow(v.x, 2) + pow(v.y, 2)).sqrt()
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
