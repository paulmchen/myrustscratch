use crate::Coin::{Penny, Nickle};

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor (i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method doing something

    }
}


fn route(ip: IpAddrKind) {
    println!("Ip kind.");
}

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin:Coin)  -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from .");
            25
        }
    }
}


fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        None=> None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    route(home);
    route(loopback);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let m = Message::Write(String::from("hello"));
    m.call();


    let v= value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("The coin value is {}.", v);

    let seven = Some(5);
    let eight = plus_one(seven);
    let none = plus_one(None);
}
