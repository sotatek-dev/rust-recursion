pub fn is_palindrome<I, T>(input: &mut I) -> bool
where
    I: DoubleEndedIterator<Item = T>,
    T: PartialOrd,
{
    if let Some(c1) = input.next() {
        if let Some(c2) = input.next_back() {
            if c1 != c2 {
                return false;
            }
        }
        return is_palindrome(input);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn is_palindrome_test() {
        assert!(is_palindrome(&mut "aba".chars()));
        assert!(is_palindrome(&mut "abba".chars()));
        assert!(!is_palindrome(&mut "abbabcd".chars()));
        assert!(is_palindrome(&mut vec![1, 2, 1].iter()));
        assert!(!is_palindrome(&mut vec![1, 2, 1, 1, 1].iter()));
    }
}
