struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn make_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

pub struct Color (i32, i32, i32);
pub struct Point (i32, i32, i32);

fn main() {
    // Creating a new user
    let user1 = User {
        username: String::from("Sam"),
        email: String::from("user1@circle.org"),
        sign_in_count: 1,
        active: true,
    };

    // Creating a new user using a function
    let user2 = make_user(String::from("John"), String::from("user1@circle.org"));

    // Creating a new user using an existing user attribiutes
    let user3 = User {
        email: String::from("user3@circle.org"),
        ..user1 // rest of the attributes are copied from user1
    };

    // Making tuple structs
    let blue = Color(0, 0, 255);
    let origin = Point(0, 0, 0);
}
