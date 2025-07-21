struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username1"),
        email: String::from("teste@123.com"),
        sign_in_count: 1,
    };

    let black = Color(0, 0, 0);

    let user2 = User {
        email: String::from("abc@444.com"),
        ..user1
    };

    println!("{}", user1.email);

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}