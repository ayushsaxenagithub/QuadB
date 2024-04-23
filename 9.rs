// Reverse a string in Rust


use std::io;

fn reverse_string(s: &mut Vec<char>) {
    s.reverse();
}

fn main() {
    let mut input = String::new();
    println!("Enter a string to reverse:");
    io::stdin().read_line(&mut input).unwrap();
    let mut chars: Vec<char> = input.trim().chars().collect();
    reverse_string(&mut chars);
    let reversed: String = chars.into_iter().collect();
    println!("Reversed string: {}", reversed);
}

