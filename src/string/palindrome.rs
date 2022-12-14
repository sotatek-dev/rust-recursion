use std::str::Chars;

fn _is_palindrome(input_chars: &mut Chars) -> bool {
    if input_chars.as_str().len() == 0 {
        return true;
    }
    if let (Some(c1), Some(c2)) = (input_chars.next(), input_chars.next_back()) {
        if c1 != c2 {
            return false;
        }
    }
    return _is_palindrome(input_chars);
}

pub fn is_palindrome(input: &str) -> bool {
    if input.len() <= 1 {
        return true;
    }
    let mut input_chars = input.chars();
    return _is_palindrome(&mut input_chars);
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn is_palindrome_test() {
        assert!(is_palindrome("aba"));
        assert!(is_palindrome("abba"));
        assert!(!is_palindrome("abbaaaaddf"));
    }
}
