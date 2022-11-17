struct Point<T, U> {
    x: T,
    y: U,
}

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f64> {
//     fn y(&self) -> f64 {
//         self.y
//     }
// }

impl <T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = get_largest(number_list);

    println!("The largest number is {}", largest);


    let char_list = vec!['a', 'm', 'y', 'q'];

    let mut largest = get_largest(char_list);

    println!("The largest character is {}", largest);

    // structs
    // let p1 = Point {x: 5, y: 10};
    // let p2 = Point {x: 5.0, y: 10.0};
    // let p3 = Point {x: 5, y: 10.9};
    
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // enums
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let integer = Option::Some(5);
    let float = Option::Some(5.0);


}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
