use regex::Regex;
use std::io;

fn main() {
    println!("Hello, world!");

    // regex
    // (\d+) \s? \+ \s? (\d+) --> e.g: 12 + 23
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

    // user data input
    println!("Please, enter your expression: ");
    let mut expression = String::new();

    io::stdin()
        .read_line(&mut expression)
        .unwrap();

    // make operations
    let caps = re_add.captures(expression.as_str()).unwrap();

    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    
    println!("{:?} left: {} and right: {}.", caps, left_value, right_value);

    let addition = left_value + right_value;

    // show results
    println!("Result: {}", addition);
}
