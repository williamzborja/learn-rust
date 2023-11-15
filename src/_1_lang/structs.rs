#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[allow(dead_code)]
struct Color(i32, i32, i32);

#[allow(dead_code)]
struct AlwaysEqual;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let mut user = User {
            active: true,
            username: String::from("johndoe"),
            email: String::from("some@email.com"),
            sign_in_count: 1,
        };

        user.email = String::from("other@mail.com");

        let _another_user = User {
            username: "userfoo".to_string(),
            ..user
        };

        let _color = Color(255, 255, 255);
    }
}
