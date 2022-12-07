use std::io;

pub fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    user_input.trim().to_string()
}