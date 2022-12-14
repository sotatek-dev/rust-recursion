fn _binary_search(
    is_asc: bool,
    input: &Vec<u64>,
    to_find: u64,
    left_index: usize,
    right_index: usize,
) -> Option<u64> {
    if left_index > right_index {
        return None;
    }
    let mid = left_index + (right_index - left_index) / 2;
    if input[mid] == to_find {
        return Some(to_find);
    }

    if is_asc {
        if input[mid] > to_find {
            if mid == 0 {
                return None;
            }
            return _binary_search(is_asc, input, to_find, 0, mid - 1);
        }
        return _binary_search(is_asc, input, to_find, mid + 1, right_index);
    } else {
        if input[mid] < to_find {
            if mid == 0 {
                return None;
            }
            return _binary_search(is_asc, input, to_find, 0, mid - 1);
        }
        return _binary_search(is_asc, input, to_find, mid + 1, right_index);
    }
}

pub fn binary_search(input: &Vec<u64>, to_find: u64) -> Option<u64> {
    let mut is_asc = true;
    if input.len() > 1 {
        is_asc = input[0] < input[(input.len() - 1)];
    }
    return _binary_search(is_asc, input, to_find, 0, input.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn binary_search_test() {
        assert_eq!(binary_search(&vec![1, 2, 3, 4, 5], 2), Some(2));
        assert_eq!(binary_search(&vec![4, 3, 2, 1], 0), None);
    }
}
