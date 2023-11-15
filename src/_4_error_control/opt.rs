#[cfg(test)]
mod test {
    #[test]
    fn option() {
        let x: Option<i32> = None;
        // let x: Option<i32> = Some(5);

        println!("{:?}", x);
    }

    #[test]
    fn unwrap() {
        let x: Option<i32> = Some(5);
        println!("{:?}", x);
    }

    #[test]
    fn unwrap_none() {
        let x: Option<i32> = None;
        println!("{:?}", x);
    }
}
