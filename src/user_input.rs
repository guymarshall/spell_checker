#![forbid(unsafe_code)]

use std::io;
use std::error::Error;

pub fn get_user_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input)?;

    Ok(user_input)
}

pub fn input(prompt: &str) -> String {
    loop {
        match get_user_input(prompt) {
            Ok(input) => {
                return input;
            },
            Err(error) => {
                println!("Error: {}", error);
            },
        };
    }
}