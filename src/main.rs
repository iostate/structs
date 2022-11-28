struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let mut a_point = Point(1, 2, 3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
