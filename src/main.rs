#![forbid(unsafe_code)]

mod scan_directory;

use promptput::input;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use scan_directory::save_file_contents;

fn main() {
    println!("Example path: 'c:/users/YOUR_USERNAME/downloads'");
    let file: File = File::open("misspelled_words.txt").expect("Invalid path.");
    let reader: BufReader<File> = BufReader::new(file);

    let mut misspelled_words: Vec<String> = Vec::new();
    for line in reader.lines() {
        misspelled_words.push(line.unwrap());
    }
    let path: String = input("Enter path to search:");

    let directory: &Path = Path::new(&path);
    let path_contents: HashMap<String, String> = save_file_contents(directory, true);

    let mut found_words: Vec<String> = Vec::new();
    for misspelled_word in &misspelled_words {
        for value in path_contents.values() {
            if value.contains(misspelled_word) && !found_words.contains(misspelled_word) {
                found_words.push(misspelled_word.to_string());
            }
        }
    }

    found_words
        .into_iter()
        .for_each(|word: String| println!("{}", word));
}
