use std::ops::{Mul, Sub};

use num::{One, Zero};

pub fn factorial<T>(n: T) -> Option<T>
where
    T: Zero + One + PartialOrd + Mul<Output = T> + Sub<Output = T> + Clone,
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
    return Some(n.clone() * factorial(n.clone() - T::one()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn fibonacci_test() {
        assert_eq!(factorial(0), Some(1));
        assert_eq!(factorial(10), Some(3628800));
    }
}
