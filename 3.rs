// Given a string of words, implement a function that returns the shortest word in the string.


use std::io;

fn find_shortest_word(s: &str) -> &str {
    s.split_whitespace()
     .min_by_key(|word| word.len())
     .unwrap_or("")
}

fn main() {
    let mut input = String::new();
    println!("Enter a string of words:");
    io::stdin().read_line(&mut input).unwrap();
    let trimmed_input = input.trim();

    let shortest_word = find_shortest_word(trimmed_input);
    println!("The shortest word is '{}'.", shortest_word);
}
