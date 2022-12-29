#![forbid(unsafe_code)]

mod user_input;

use std::fs::{self, File, DirEntry};
use std::path::{Path, PathBuf};
use std::io::{prelude::*, BufReader};

fn read_files(directory: &Path) -> Vec<String> {
    let mut contents_vector: Vec<String> = Vec::new();
    if directory.is_dir() {
        for entry in fs::read_dir(directory).unwrap() {
            let entry: DirEntry = entry.unwrap();
            let path: PathBuf = entry.path();
            if path.is_dir() {
                read_files(&path);
            } else {
                let contents: String = fs::read_to_string(path).unwrap_or("".to_string());
                println!("{}", &contents);
                contents_vector.push(contents);
            }
        }
    }
    contents_vector
}

fn main() {
    let file: File = File::open("misspelled_words.txt").expect("Invalid path.");
    let reader: BufReader<File> = BufReader::new(file);
    
    let mut misspelled_words: Vec<String> = Vec::new();
    for line in reader.lines() {
        misspelled_words.push(line.unwrap());
    }
    for misspelled_word in misspelled_words {
        println!("{}", misspelled_word);
    }
    let path: String = user_input::get_user_input("Enter path to search:");

    let directory: &Path = Path::new(&path);
    let contents_vector: Vec<String> = read_files(directory);

    println!("{:#?}", contents_vector);
}
