#[cfg(test)]
mod test {

    #[test]
    fn test_array() {
        let days = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];

        let bytes = [0; 5];

        println!("days: {:#?}", days);
        println!("bytes: {:#?}", bytes);

        println!("days[0]: {}", days[0]);
    }
}
