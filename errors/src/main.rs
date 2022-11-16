use core::panic;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::error::Error;

fn main() {
    // a();

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem openng the file: {:?}", other_error)
            }
        }
    };

    // using closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // result enum
    let f = File::open("hey.txt").expect("Failed to open hey.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    
}

// error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    // let mut f = File::open("hey.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    fs::read_to_string("hey.txt")


}

