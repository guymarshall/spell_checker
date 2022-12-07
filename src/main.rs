#![forbid(unsafe_code)]

// for each word in misspelled_words.txt, search whole directory. As soon as word is found, break the loop and print the word, then move to the next word until done
// for word in list
//     for file in directory
//         if word found
//             print word
//             break

mod user_input;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let directory: String = user_input::get_user_input("Enter directory to search:");

    let file: File = File::open("misspelled_words.txt")?;
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line.unwrap();
        println!("{}", line);
    }

    println!("Directory: {}", directory); // so the compiler doesn't complain

    Ok(())
}