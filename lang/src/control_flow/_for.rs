#[cfg(test)]
mod test {
    #[test]
    fn test_for() {
        let nums = [1, 2, 3, 4, 5];

        for n in nums.iter() {
            println!("{}", n);
        }

        // create a for that print index and element
        for (index, element) in nums.iter().enumerate() {
            println!("index is {} and element is {}", index, element);
        }
    }
}
