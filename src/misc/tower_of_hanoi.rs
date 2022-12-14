use num::Integer;

fn _tower_of_hanoi<T>(
    steps: &mut Vec<(T, String, String)>,
    n: T,
    from_rod: &str,
    to_rod: &str,
    aux_rod: &str,
) where
    T: PartialOrd + Integer + Clone,
{
    if n == T::zero() {
        return;
    }
    _tower_of_hanoi(steps, n.clone() - T::one(), from_rod, aux_rod, to_rod);
    steps.push((n.clone(), String::from(from_rod), String::from(to_rod)));
    _tower_of_hanoi(steps, n.clone() - T::one(), aux_rod, to_rod, from_rod);
}

pub fn tower_of_hanoi<T>(
    n: T,
    from_rod: &str,
    to_rod: &str,
    aux_rod: &str,
) -> Option<Vec<(T, String, String)>>
where
    T: PartialOrd + Integer + Clone,
{
    if n <= T::zero() {
        return None;
    }
    let mut steps = Vec::<(T, String, String)>::new();
    _tower_of_hanoi(&mut steps, n, from_rod, to_rod, aux_rod);
    return Some(steps);
}

#[cfg(test)]
mod tests {
    use super::tower_of_hanoi;

    #[allow(non_snake_case)]
    #[test]
    fn tower_of_hanoi_test() {
        let steps = tower_of_hanoi(3, "A", "C", "B").unwrap();
        let A = String::from("A");
        let B = String::from("B");
        let C = String::from("C");
        let expects = vec![
            (1, A.clone(), C.clone()),
            (2, A.clone(), B.clone()),
            (1, C.clone(), B.clone()),
            (3, A.clone(), C.clone()),
            (1, B.clone(), A.clone()),
            (2, B.clone(), C.clone()),
            (1, A.clone(), C.clone()),
        ];

        let matching = steps.iter().zip(&expects).filter(|&(a, b)| a.eq(b)).count();
        assert_eq!(steps.len(), matching);
    }
}
