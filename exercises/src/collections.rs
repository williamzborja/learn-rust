#[cfg(test)]
mod tests {

    #[test]
    fn test_median() {
        let nums = vec![3, 4, 5, 6, 7, 8, 9, 10];

        let median = nums.iter().sum::<i32>() as f64 / nums.len() as f64;

        assert_eq!(median, 6.5);
    }
    #[test]
    fn pig_latin() {
        let words = vec!["first", "apple", "banana", "second", "third"];
        let mut pig_latin = Vec::new();

        for word in &words {
            let first_char = word.chars().next();

            if let Some(c) = first_char {
                match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => pig_latin.push(format!("{}-hay", word)),
                    _ => pig_latin.push(format!("{}-{}ay", &word[1..], c)),
                }
            }
        }

        assert_eq!(
            pig_latin,
            vec![
                "irst-fay",
                "apple-hay",
                "anana-bay",
                "econd-say",
                "hird-tay"
            ]
        );
    }
}
