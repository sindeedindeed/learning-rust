struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotherone@example.com");
    println!("{0}", user1.email);
    let user2 = User {
        email: String::from("user@example.com"),
        ..user1
    };
    println!("{} {}", user2.email, user2.sign_in_count);
}
