// create a unit test module
#[cfg(test)]
mod test {

    // create a test function
    #[test]
    fn test_flow() {
        let x = 1;
        let x = if x == 1 {
            "hello one"
        } else {
            "hello other number"
        };
        // create a variable
        let x = 1;
        // create a conditional statement
        if x == 1 {
            // print a message
            println!("x is 1");
        } else {
            // print a message
            println!("x is not 1");
        }
    }
}
