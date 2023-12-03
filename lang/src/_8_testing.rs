#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn custom_message() {
        assert!(true, "This is the custom error message {}", "algo")
    }
}
