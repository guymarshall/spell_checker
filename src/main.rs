#![forbid(unsafe_code)]

mod user_input;

use std::collections::HashMap;
use std::fs::{self, File, DirEntry};
use std::path::{Path, PathBuf};
use std::io::{prelude::*, BufReader};

fn save_file_contents(directory: &Path) -> HashMap<String, String> {
    let mut path_contents: HashMap<String, String> = HashMap::new();
    if directory.is_dir() {
        for entry in fs::read_dir(directory).unwrap() {
            let entry: DirEntry = entry.unwrap();
            let path: PathBuf = entry.path();
            if path.is_dir() {
                save_file_contents(&path);
            } else {
                let contents: String = fs::read_to_string(&path).unwrap_or("".to_string());
                path_contents.insert(path.to_string_lossy().to_string(), contents);
            }
        }
    }
    path_contents
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
    let path_contents: HashMap<String, String> = save_file_contents(directory);

    println!("{:#?}", path_contents);
}
