// lifetime are generics to specify the type and life of a reference
// that are really important to prevent dangling pointers

/*
fn longest_error(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
 */

#[allow(dead_code)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
#[allow(dead_code)]
// this is return a dangling pointer
fn longest_dangling<'a>(_x: & str, _y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str() // this is a dangling pointer
    // result is dropped here
}

 */

#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn dangling_pointer() {
        let r;
        {
            let x = &5;
            r = &x;
            println!("{:p}", r);
        }
        // println!("{:p}", r); // this generate error
    }

    #[test]
    fn declaration() {
        fn declaration<'a>() -> &'a str {
            let r: &'a str = "hello";
            r
        }

        let r = declaration();
        println!("{:p}", r);
    }

    #[test]
    fn lifetime() {
        let string1 = String::from("abcd");
        {
            let string2 = "xyz";

            let result = longest(string1.as_str(), string2);
            println!("The longest string is {}", result);
        }
    }

    #[test]
    fn lifetime_2() {
        let _string1 = String::from("abcd");
        // let result;
        {
            let _string2 = "xyz".to_string();

            // result = longest(string1.as_str(), string2.as_str());// this line generate an error
        }
        // println!("The longest string is {}", result);
    }

    #[test]
    fn struct_lifetime() {
        let text = String::from(
            "William Shakespeare, Hamlet, Act I, Scene III. William Rodriguez Rust Act I.",
        );

        let first_sentence = text.split('.').next().expect("sentence without .");
        let _important = ImportantExcerpt {
            part: first_sentence,
        };
    }
}
