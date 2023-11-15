mod functions;
mod refs_and_borrow;

#[cfg(test)]
mod tests {

    #[test]
    fn copy() {
        let x: i32 = 32;
        // this statement perform a copy
        let y = x;
        println!("{}, {}", x, y);
    }

    #[test]
    fn moving() {
        let text = String::from("Hello, world!");
        // this statement perform a borrow
        let text_moved = text; // here text is moved to text_ref
                               // after this statement, text is no longer available
                               // println!("{}", text); error
        println!("{}", text_moved);
    }

    #[test]
    fn shallow_copy() {
        let text = String::from("Hello World!");
        // reference is shallow copy
        let text_ref = &text;

        println!("{} {}", text_ref, text); // text is still available
                                           // you can have multiple immutable references
    }

    #[test]
    fn deep_copy() {
        // clone or deep copy
        let text = String::from("Hello World!");

        let text2 = text.clone(); // this statement perform a clone
                                  // after this statement, text is still available
                                  // that is more expensive than borrow but you can have multiple mutable references
        println!("{} {}", text, text2);
    }
}
