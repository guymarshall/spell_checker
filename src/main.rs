#![forbid(unsafe_code)]

mod user_input;

use std::fs::{self, File, DirEntry};
use std::path::{Path, PathBuf};
use std::io::{prelude::*, BufReader};

fn save_file_contents(directory: &Path) -> Vec<String> {
    let mut contents_vector: Vec<String> = Vec::new();
    if directory.is_dir() {
        for entry in fs::read_dir(directory).unwrap() {
            let entry: DirEntry = entry.unwrap();
            let path: PathBuf = entry.path();
            if path.is_dir() {
                save_file_contents(&path);
            } else {
                let contents: String = fs::read_to_string(&path).unwrap_or("".to_string());
                println!("File: {}", path.to_string_lossy());
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
    let path: String = user_input::get_user_input("Enter path to search:");

    let directory: &Path = Path::new(&path);
    let contents_vector: Vec<String> = save_file_contents(directory);

    println!("{:#?}", contents_vector);
}
