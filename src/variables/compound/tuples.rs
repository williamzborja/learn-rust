#[cfg(test)]
mod test {

    #[test]
    fn test_tuples() {
        let _tup = (12, "william");

        // destructuring
        let (age, name) = _tup;

        println!("age {} and name {}", age, name);
    }
}
