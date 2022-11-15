use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // strings are stored as a collection of UTF-8 encoded bytes

    let mut s = String::from("foo");

    s.push_str("bar");
    s.push('!');

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3: String = s1 + &s2;

    // indexing
    let hello: String = String::from("Hello");
    // let c: char = hello[0];

    for b in "नमस्ते".chars() {
        println!("{}", b);
    }

    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }


}
