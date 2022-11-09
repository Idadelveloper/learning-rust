struct User {
    username: String,
    email: String,
    sign_n_count: u64,
    active: bool,
} 

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// associated function
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let mut user1 = User {
        email: String::from("ida@gmail.com"),
        username: String::from("ida"),
        active: true,
        sign_n_count: 1

    };

    let name = user1.username;
    user1.username = String::from("Ida Delphine");

    // let user2 = build_user(String::from("idadel@gmal.com"), String::from("idadel"));

    // let user3 = User {
    //     email: String::from("idadelphine@gmail.com"),
    //     username: String::from("idadelphine"),
    //     ..user2
    // };

    // tuple struct
    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32);

    let rect = &Rectangle { width: 30, height: 50 };

    println!("rect: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels",
        rect.area()
    );

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };
    
    let rect2 = Rectangle {
        width: 40, 
        height: 50
    };

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    let rect3 = Rectangle::square(25);


}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// } 

// fn area(wdth: u32, height: u32) -> u32 {
//     width * height
// }
