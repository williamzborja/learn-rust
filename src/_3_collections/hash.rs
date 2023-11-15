#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn create() {
        // create by new
        let mut map = HashMap::new();
        let map2 = HashMap::from([("name", "william"), ("age", "31")]);

        map.insert("name", "william");

        println!("{:?}", map);
        println!("{:?}", map2);
    }

    #[test]
    fn access() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");

        let _score = scores.get(&team_name).copied().unwrap_or(0);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    #[test]
    fn update() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 50);

        println!("{:?}", scores);
    }
    #[test]
    fn insert_if_not_exits() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);

        scores.entry("Blue".to_string()).or_insert(0);

        println!("{:?}", scores);
        assert_eq!(1, scores.len());
    }

    #[test]
    fn count_words() {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
