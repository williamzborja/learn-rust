// create a unit test module with and a test function
#[cfg(test)]
mod test {
    // create a test function
    #[test]
    fn test_match() {
        let x = "hello";
        match x {
            "hello" => println!("x is hello"),
            "goodbye" => println!("x is goodbye"),
            _ => println!("x is something else"),
        }

        let x = 1;
        match x {
            negative if x < 0 => println!("{} is less than zero", negative),
            1..=100 => println!("x is between 1 and 100"),
            _ => println!("x is something else"),
        }

        match x {
            1..=3 => println!("x is 1 or 2 or 3"),
            _ => println!("x is something else"),
        }
    }
}
