fn main() {
    let mut user1 = build_user(String::from("nobody@example.com"),
        String::from("someusername123"));

    user1.email = String::from("somebody@example.com");
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
