#![forbid(unsafe_code)]

// for each word in misspelled_words.txt, search whole directory. As soon as word is found, break the loop and print the word, then move to the next word until done
// for word in list
//     for file in directory
//         if word found
//             print word
//             break

// mod user_input;

// use std::fs::File;
// use std::io::{self, prelude::*, BufReader};

// fn main() -> io::Result<()> {
//     let directory: String = user_input::get_user_input("Enter directory to search:");

//     let file: File = File::open("misspelled_words.txt")?;
//     let reader: BufReader<File> = BufReader::new(file);

//     for line in reader.lines() {
//         let line: String = line.unwrap();
//         println!("{}", line);
//     }

//     println!("Directory: {}", directory); // so the compiler doesn't complain

//     Ok(())

// }

mod user_input;

use std::fs::{self, File, DirEntry};
use std::path::{Path, PathBuf};
use std::io::{self, prelude::*, BufReader};

fn read_files(directory: &Path) {
    if directory.is_dir() {
        for entry in fs::read_dir(directory).unwrap() {
            let entry: DirEntry = entry.unwrap();
            let path: PathBuf = entry.path();
            if path.is_dir() {
                read_files(&path);
            } else {
                let contents: String = fs::read_to_string(path).unwrap();
                println!("{}", contents);
            }
        }
    }
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
    read_files(directory);
}
