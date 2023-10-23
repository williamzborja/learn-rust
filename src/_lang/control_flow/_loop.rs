// create a unit test module with a test function
#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    // create a test function
    #[test]
    fn test_loop() {
        let mut i = 0;

        loop {
            println!("i is {}", i);
            i += 1;
            thread::sleep(Duration::from_millis(10));
            if i == 10 {
                break;
            }
        }

        let mut x = 1;
        // create a loop with outer label
        'outer: loop {
            println!("x is {}", x);
            x += 1;
            // create a loop
            'inner: loop {
                x += 1;
                println!("x is {}", x);
                if x == 3 {
                    // break the outer loop
                    break 'outer;
                }
                if x == 5 {
                    // break the inner loop
                    break 'inner;
                }
            }
        }
    }
}
