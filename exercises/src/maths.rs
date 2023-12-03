use std::collections::HashSet;

fn factorize(number: i32) -> HashSet<i32> {
    let mut factor_nums: HashSet<i32> = HashSet::from([1, number]);
    let n = number / 2;

    for x in 2..=n {
        match number % x {
            0 => {
                factor_nums.insert(n / x);
                factor_nums.insert(x);
            }
            _ => continue,
        };
    }

    factor_nums // return result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor() {
        let n = 24;

        assert_eq!(factorize(n), HashSet::from([1, 2, 3, 4, 6, 8, 12, 24]));
    }
}
