use num::traits::{One, Zero};
use std::ops::{Add, Sub};

pub fn fibonacci<T>(n: T) -> Option<T>
where
    T: Zero + One + PartialOrd + Add<Output = T> + Sub<Output = T> + Clone,
{
    if n < T::zero() {
        return None;
    }
    if n.is_zero() {
        return Some(T::one());
    }
    if n.is_one() {
        return Some(T::one());
    }
    return Some(
        fibonacci(n.clone() - T::one()).unwrap()
            + fibonacci(n.clone() - T::one() - T::one()).unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::fibonacci;

    #[test]
    fn fibonacci_test() {
        assert_eq!(fibonacci(0), Some(1));
        assert_eq!(fibonacci(1), Some(1));
        assert_eq!(fibonacci(2), Some(2));
        assert_eq!(fibonacci(3), Some(3));
        assert_eq!(fibonacci(4), Some(5));
        assert_eq!(fibonacci(20), Some(10946));
    }
}
