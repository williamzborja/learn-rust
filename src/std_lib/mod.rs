// create a test module
#[cfg(test)]
mod test {
    #[test]
    fn test_std_lib() {
        std::env::set_var("RUST_BACKTRACE", "full");
                                std::fs::read("./Cargo.toml").unwrap();

        concat!("std", "_", "lib");
        format!("{}{}", "std", "_lib");
    }
}
