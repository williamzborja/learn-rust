#[allow(dead_code)]
fn calculate_length(text: &String) -> usize {
    text.len()
}

#[allow(dead_code)]
fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[..i];
        }
    }
    &text[..]
}

#[allow(dead_code)]
fn first_word_with_slice(text: &str) -> &str {
    let bytes = text.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[..i];
        }
    }
    &text[..]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn stack_ref() {
        let x = 32;
        let x_ref = &x;

        println!("pointer: {:p} value x:{}", x_ref, x);
    }

    #[test]
    fn heap_ref() {
        let text = String::from("Hello, world!");
        let len = calculate_length(&text);

        // text data is located in heap
        // --------Heap Memory----------------
        // |H|e|l|l|o|,| |w|o|r|l|d|!|\0|
        // -----------------------------------

        // text variable is located in stack
        // String is a struct that contains
        // a pointer to the heap memory length and capacity
        // | 0x1234 | len: 14| cap: 14

        // text_ref is located in stack is only a pointer
        // text_ref: 0x1232
        println!("pointer: {:p} value: {}", &text, text);
        println!("{}", len);
    }

    #[test]
    #[allow(unused_mut)]
    fn multiple_refs() {
        let mut text = String::from("Hello");

        let r1 = &text;
        let r2 = &text;
        //we can have multiples references at the same time

        println!("{} {} {}", text, r1, r2);

        // until here all works fine and here end the scope of r1 and r2

        // we can't have mutable and immutable references at the same time

        let mut r3 = &mut text;

        // println!("{}", text); // this throw an error because when I'm passing text inmutable to
        // println and i already have a mutable reference alive

        println!("{}", r3); // this works because r1 and r2 are out of scope only exist one mutable reference

        println!("{}", text)
    }

    fn dangle() -> String {
        let _text = "Hello".to_string();
        // &text; // this is a err or dangling pointer
        "".to_string() //
    }

    #[test]
    fn dangling_ref() {
        let ref_to_string = dangle();

        println!("{}", ref_to_string);
    }

    #[test]
    fn slicing() {
        let text = String::from("Hello");

        let len = text.len();

        let slice = &text[3..len];
        println!("{:?}", slice);

        let slice = &text[3..];

        println!("{:?}", slice);

        let slice = &text[0..len];

        println!("{:?}", slice);

        let slice = &text[..];

        println!("{:?}", slice);

        println!("{:?}", first_word(&text));

        // text.clear(); // this create and error but this is a help
        // to identify references or slices to invalid memory
    }

    #[test]
    fn string_literals_are_slices() {
        let s = "Hello, world!";

        println!("{}", s);
    }

    #[test]
    fn slice_and_string_as_parameter() {
        let text = String::from("Hello, world!");
        let literal = "Hello, world!";
        let _slice = &text[..];

        let _word = first_word(&text);
        let _word = first_word(literal);
    }
}
