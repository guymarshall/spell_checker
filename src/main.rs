#![forbid(unsafe_code)]

mod user_input;
mod scan_directory;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

use scan_directory::save_file_contents;

fn main() {
    let file: File = File::open("misspelled_words.txt").expect("Invalid path.");
    let reader: BufReader<File> = BufReader::new(file);
    
    let mut misspelled_words: Vec<String> = Vec::new();
    for line in reader.lines() {
        misspelled_words.push(line.unwrap());
    }
    let path: String = user_input::get_user_input("Enter path to search:");

    let directory: &Path = Path::new(&path);
    let path_contents: HashMap<String, String> = save_file_contents(directory, true);

    // let _ = &path_contents.into_iter().for_each(|(_, value)| println!("{}", &value)); // angers borrow-checker

    let mut found_words: Vec<String> = Vec::new();
    // &misspelled_words.into_iter().for_each(|misspelled_word| {
    //     &path_contents.into_iter().for_each(|(_, value)| {
    //         if value.contains(&misspelled_word) {
    //             found_words.push(misspelled_word);
    //         }
    //     });
    // });
    for misspelled_word in &misspelled_words {
        for (_, value) in &path_contents {
            if value.contains(misspelled_word) {
                found_words.push(misspelled_word.to_string());
            }
        }
    }

    found_words.into_iter().for_each(|word| println!("{}", word));
}
