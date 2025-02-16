struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple like structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;


fn main() {
    let user = build_user(String::from("test"), String::from("test@mail.com"));
    println!("{} {}", user.username, user.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user
    };
    println!("{}", user2.email);
    // user is invalid here (because it is moved to user2)
    // but can still use email, active and sign_in_count

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    black.0;
    origin.1;

}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
