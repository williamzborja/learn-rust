#[cfg(test)]
mod test {
    extern crate ferris_says;
    use ferris_says::say;
    use std::io::{stdout, BufWriter};

    #[test]
    fn test_deps() {
        let out = "Hello fellow Rustaceans!";
        let width = 24;

        let mut writer = BufWriter::new(stdout());
        say(out, width, &mut writer).unwrap();
    }
}
