fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The new value of x is {}", x);

    // integers
    let a = 98_222;
    let b = 0xff;
    let c = 0o77;
    let d = 0b1111_0000;
    let e = b'A';

    let f: u8 = 255;

    // floating point nums
    let f = 2.0;
    let g: f32 = 3.0;


    const LIKE_COUNT: u32 = 10000;

    // compound types
    let tup = ("Ida Delphine", 22);
    let (name, age) = tup;
    let ida_age = tup.1;

    let error_codes = [200, 404, 300, 500];
    let not_found = error_codes;

    let sum = my_function(1, 2);
    println!("{}", sum);

    // control flow
    if sum < 10 {
        println!("first condition was true");
    } else if sum < 22 {
        println!("second condition was true")
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The result of the loop is: {}", result);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    // for loop
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number);
    }

    /*
        Testing out
        block comment
    */




}

// functions

fn my_function(x: i32, y: i32) -> i32 {
    println!("Arguments are {} and {}", x, y);
    let sum = x + y;
    return sum;
}