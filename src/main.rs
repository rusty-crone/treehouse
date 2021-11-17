#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    let visitor_list = ["jason", "brad", "andr\u{e9}"];
    let mut allow_them_in = false;
    for visitor in &visitor_list {
        if visitor == &name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Hello, {:?}", name);
    } else {
        println!("You are not in the club {}", name);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}
