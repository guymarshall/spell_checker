#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

pub fn save_file_contents(directory: &Path, log_output: bool) -> HashMap<String, String> {
    let mut path_contents: HashMap<String, String> = HashMap::new();
    if directory.is_dir() {
        for entry in fs::read_dir(directory).unwrap() {
            let entry: DirEntry = entry.unwrap();
            let path: PathBuf = entry.path();
            if path.is_dir() {
                save_file_contents(&path, log_output);
            } else {
                let contents: String = fs::read_to_string(&path).unwrap_or("".to_string());

                if log_output {
                    println!("{}", path.to_string_lossy());
                }
                path_contents.insert(path.to_string_lossy().to_string(), contents);
            }
        }
    }
    path_contents
}
