/// A metric over type ``T``
pub trait Metric<T> {
    /// Computes the distance between ``self`` and ``other``
    fn distance(&self, other: &Self) -> T;

    /// Computes the squared distance between ``self`` and ``other``.
    fn distance_squared(&self, other: &Self) -> T;
}


