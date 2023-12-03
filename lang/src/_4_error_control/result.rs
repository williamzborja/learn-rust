use std::{
    fs::{self, File},
    io::{self, Read},
};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_result = File::open("hello.txt");

    let mut file = match username_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::ErrorKind};

    #[test]
    fn match_pattern() {
        let greeting_file_result = File::open("hello.txt");

        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem happen creating {:?}", e),
                },
                other => panic!("problem opening file error: {:#?}", other),
            },
        };
    }

    #[test]
    fn () {
        let greeting_file_result = File::open("hello.txt");

        greeting_file_result.unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.text").unwrap_or_else(|e| panic!("Error: {}", e))
            } else {
                panic!("{}", error)
            }
        });
    }

    #[test]
    fn default_panic() {
        let _file_result = File::open("hello.txt").unwrap();
    }

    #[test]
    fn expect() {
        let _file_result =
            File::open("hello.txt").expect("hello.txt should be included in this project");
    }

    #[test]
    fn result_patters() -> Result<(), io::Error> {
        let username = read_username_from_file()?;
        let username2 = read_username_from_file_v2()?;
        let username3 = read_username_from_file_v3()?;
        let username4 = read_username_from_file_v4()?;

        println!("{} {} {} {}", username, username2, username3, username4);
        Ok(())
    }
}
