#[cfg(test)]
mod test {

    #[test]
    fn test_text() {
        let s = "william";
        let mut growthable = String::from("");

        growthable.push_str(s);
        growthable.push_str(s);
        growthable.pop();

        println!("{}", growthable);

        let concat = format!("{} {}", growthable, growthable);
        println!("{}", concat);
    }
}
