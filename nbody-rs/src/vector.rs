use std::iter::Sum;
use std::ops::{Deref, Sub};
use num_traits::Pow;
use crate::metric::Metric;

/// An ``N`` dimensional vector over ``T``
pub struct Vector<T, const N: usize> {
    elements: [T; N],
}

impl<T, const N: usize> Deref for Vector<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}


impl<T, const S: usize> Metric<T> for Vector<T, S>
    where
        T: Pow<usize, Output=T> + Pow<f64, Output=T> + Sum,
        for<'a> &'a T: Sub<&'a T, Output=T>
{
    fn distance(&self, other: &Self) -> T {
        self.distance_squared(&other).pow(0.5)
    }

    fn distance_squared(&self, other: &Self) -> T {
        self.iter()
            .zip(other.iter())
            .map(|(a, b)| (a - b).pow(2))
            .sum()
    }
}