use std::i32;

#[allow(dead_code)]
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(dead_code)]
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(dead_code)]
fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(dead_code)]
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest() {
        let list_int = &vec![1, 2, 3, 45, 7];
        let largest = largest_i32(list_int);
        assert_eq!(&45, largest);

        let list_char = &vec!['a', 'b', 'f', 'c'];
        let largest = largest_char(list_char);
        assert_eq!(&'f', largest);

        let list_string = &vec!["ae", "cd"];

        let largest_int = largest_generic(list_int);
        let largest_char = largest_generic(list_char);
        let largest_str = largest_generic(list_string);

        assert_eq!(&45, largest_int);
        assert_eq!(&'f', largest_char);
        assert_eq!(&"cd", largest_str);
    }

    #[test]
    fn with_struct() {
        let integer = Point { x: 1, y: 22 };
        let floats = Point { x: 2.1, y: 33.0 };

        println!("{:?} {:?}", integer, floats);

        let x_ref = integer.x();

        println!("{:p} {}", x_ref, x_ref);
    }
}
