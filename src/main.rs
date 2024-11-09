#![forbid(unsafe_code)]

mod scan_directory;

use promptput::input;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Error};
use std::path::Path;

use scan_directory::save_file_contents;

fn main() {
    println!("Example path: 'c:/users/YOUR_USERNAME/downloads'");
    let file: File = File::open("misspelled_words.txt").expect("Invalid path.");
    let reader: BufReader<File> = BufReader::new(file);

    let misspelled_words: Vec<String> = reader
        .lines()
        .map(|line: Result<String, Error>| line.unwrap())
        .collect();

    let path: String = input("Enter path to search:");

    let directory: &Path = Path::new(&path);
    let path_contents: HashMap<String, String> = save_file_contents(directory, true);

    let found_words: Vec<String> = misspelled_words
        .iter()
        .filter(|misspelled_word: &&String| {
            path_contents
                .values()
                .any(|value: &String| value.contains(*misspelled_word))
        })
        .map(|misspelled_word: &String| misspelled_word.to_string())
        .collect();

    found_words
        .into_iter()
        .for_each(|word: String| println!("{}", word));
}
