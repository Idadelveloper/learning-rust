fn main() {
    // ownership rules
    /*
        1. Each value in Rust has a variable that's called it's owner.
        2. There can only be one owner at a time.
        3. When the owner goes out of scope, the value will be dropped.
    */

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{} world", s1);
    
    let a = String::from("hello");
    takes_ownership(a);
    // println!("{}", a);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);

    // references
    let st1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", st1, len);

    // mutable reference
    let mut s1 = String::from("hello");
    change(&mut s1);

    // slices
    let mut s = String::from("hello world");
    let s2 = "hello world";
    let word = first_word(s2);

}

fn takes_ownership(some_strng: String) {
    println!("{}", some_strng);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(s_string: String) -> String {
    s_string
}

// references
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

// mutable reference
fn change(some_string: &String) {
    some_string.push_str(", world");
}


// Rules of references
/*
    1. At any given time, you can have either one mutable reference or any number of immutable references.
    2. References must always be valid.
*/ 

// slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
