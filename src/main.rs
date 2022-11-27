#[allow(dead_code)]
#[warn(dead_code)]
struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("qmtruong92@gmail.com"),
        username: String::from("astarfish"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("WallllaceTheLongHairedWeasel");

    let user2 = build_user(
        String::from("qmtruong92@gmail.com"), 
        String::from("analyticalstarfish"),
    );


    let user3 = User {
        email: String::from("ra123@gmail.com"),
        username: String::from("Tally Hassee"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

//fn transform(anything: User) {
//
//}
