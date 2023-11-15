#[allow(dead_code)]
fn take_ownership(text: String) -> String {
    println!("{}", text);
    text
}

#[allow(dead_code)]
fn make_copy(number: i32) {
    println!("{}", number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_value() {
        let text = String::from("Hello, world!");
        let text2 = String::from("Hello, world!");
        let num = 32;

        take_ownership(text); // text is moved and dropped
        let text3 = take_ownership(text2); // text2 ownership is moved and return to main

        make_copy(num);

        println!("num: {}, text2: {}", num, text3);
        //only text3 is available
        // text was moved and dropped
        // text2 was moved and returned to text3
    }
}
