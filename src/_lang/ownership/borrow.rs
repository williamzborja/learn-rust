#[cfg(test)]
mod tests {

    #[test]
    fn copy() {
        let x: i32 = 32;
        // this statement perform a copy
        let y = x;

        println!("{x}, {y}");
    }

    #[test]
    fn move_content() {
        let text = String::from("Hello, world!");
        // this statement perform a borrow
        let text_moved = text; // here text is moved to text_ref

        // after this statement, text is no longer available
        //
        println!("{}", text_moved);
    }

    #[test]
    fn borrow() {
        let text = String::from("Hello, world!");
        // this statement perform a borrow
        let text_ref = &text;

        println!("{} {}", text_ref, text);
    }
}
