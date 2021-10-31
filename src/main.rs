use std::io::{self, Write};

fn main() {

    let mut input = String::new();

    print!(">");
    io::stdout().flush().expect("Error");

    io::stdin().read_line(&mut input).expect("Error reading the user's input");

    for word in input.split_whitespace() {

        if let 'a' | 'e' | 'i' | 'o' | 'u' = word.chars().nth(0).unwrap() {
            println!("{}-hay", word.trim());
        } else {
            println!("{}-{}ay", &word[word.chars().nth(1).into_iter().len()..].trim(), word.chars().nth(0).unwrap())
        }
    }
}
