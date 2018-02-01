fn main() {
    let mut user1 = User {
        email: String::from("nobody@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 457,
    };

    user1.email = String::from("somebody@example.com");
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
