use std::string::String;

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    println!("{}", r1);
    
    let r3 = &mut s; // no problem
    r3.push_str("World");
    println!("{}", s);
    println!("{}", s);
}