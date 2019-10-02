extern crate num;

use std::ops::{Add, Mul};

use num::{Float, One};

use crate::sf;

pub fn magnitude<T, U>(v: &sf::Vector2<T>) -> U
where
    U: Float,
    T: Add<Output = T> + One + Copy + Into<U>,
{
    use num::pow;
    ((pow(v.x, 2) + pow(v.y, 2)).into()).sqrt()
}

pub fn scalar_mul<T, U>(s: T, v: &sf::Vector2<U>) -> sf::Vector2<U>
where
    U: Mul<Output = U> + Copy,
    T: Into<U> + Copy,
{
    sf::Vector2::new(s.into() * v.x, s.into() * v.y)
}
