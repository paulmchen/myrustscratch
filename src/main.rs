struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, name: String) -> User {
    User {
        name,
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct Color (i32,i32,i32);
struct Point (i32,i32,i32);

fn main() {
    let user1 = build_user(String::from("paulc@abc.com"), String::from("paulc"));
    println!("User: {} has email {}.", user1.name, user1.email);

    let user2 = User {
        email: String::from("jk@hotmail.com"),
        name: String::from("jk"),
        ..user1
    };

    println!("User2 has activate status of {}.", user2.active);
    let black = Color(0,0,0);
    let point = Point(1,2,3);
    println!("point x:{}, y:{}, z:{}.", point.0, point.1, point.2);
}
