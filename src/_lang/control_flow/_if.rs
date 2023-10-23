// create a unit test module
#[cfg(test)]
mod test {
    // create a test function
    #[test]
    fn test_flow() {
        let x = 1;
        // ternary if
        let x = if x == 1 {
            "hello one"
        } else {
            "hello other number"
        };
        println!("x is {}", x);

        let x = 2;
        // create a conditional statement
        if x == 1 {
            // print a message
            println!("x is 1");
        } else if x == 2 {
            // print a message
            println!("x is not 1");
        } else {
            println!("x is not 1 or 2");
        }
    }
}
