fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "somebody123",
        active: true,
        sign_in_count: 1,
    };
}

struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}
