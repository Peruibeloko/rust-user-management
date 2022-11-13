use std::io::{self, Write};

pub fn prompt_user(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap().to_string();
    input.trim().to_string()
}