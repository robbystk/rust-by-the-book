fn main() {
    let mut user1 = build_user(String::from("nobody@example.com"),
        String::from("someusername123"));

    user1.email = String::from("somebody@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another123"),
        ..user1
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
