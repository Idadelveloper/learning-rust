enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn some_function() {
        println!("Learning Rust");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum UseState {
    Bamenda,
    Buea,
    Douala,
    Yaounde,
    Bafoussam
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UseState)
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    
    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // option enum
    enum  Option<T> {
        Some(T),
        None,
    }

    value_in_cents(Coin::Quarter(UseState::Buea));

    // match expression and option enum
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let syntax
    if let Some(3) = some_value {
        println!("three");
    }
}

fn route(ip_kind: IpAddrKind) {

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Town quarter from {:?}!", state);
            25
        }
    }
}

// match expression and option enum
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


