struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username.clone(),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let user4 = build_user(String::from("email"), String::from("username"));

    println!("{}", user4.email);
    println!("{}", user3.email);
    println!("{}", user2.email);
    println!("{}", user1.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black is {}, {}, {}.", black.0, black.1, black.2);
    println!("origin is {}, {}, {}.", origin.0, origin.1, origin.2);

    let Point(x, y, z) = origin;
    println!("new point is {} {} {}", x, y, z);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
