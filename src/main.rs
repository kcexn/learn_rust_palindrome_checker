extern crate utils;

use std::env;

fn main() {
    let mut arguments = env::args();
    arguments.next(); 
    
    let text = match arguments.next() {
        Some(arg) => arg,
        None => {
            println!("No Arguments found!");
            return
        },
    };

    if utils::is_palindrome(&text) {
        println!("{} is a palindrome", text);
    } else {
        println!("{} is not a palindrome", text);
    }

}
