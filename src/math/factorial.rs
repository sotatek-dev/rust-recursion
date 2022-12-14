use num::Integer;

pub fn factorial<T>(n: T) -> Option<T>
where
    T: PartialOrd + Integer + Clone,
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
    fn factorial_test() {
        assert_eq!(factorial(0), Some(1));
        assert_eq!(factorial(10), Some(3628800));
    }
}
