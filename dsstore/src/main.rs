extern crate glob;

use std::env;
use std::result::Result;
use std::fs;
use std::path::PathBuf;
use glob::glob;

fn main() {
    let glob_path = match env::current_dir() {
        Ok(cwd) => cwd.join("**/.DS_Store"),
        Err(_) => PathBuf::from("~/**/.DS_Store")
    };
    let glob_str = glob_path.to_str().unwrap();

    println!("Removing .DS_Store from {:?}", glob_path.display());

    for entry in glob(&glob_str).unwrap().filter_map(Result::ok) {
        match fs::remove_file(entry.as_path()) {
            Ok(_) => println!("Removed {:?}", entry.display()),
            Err(e) => println!("Could not remove {}: {:?}", entry.display(), e.to_string())
        }
    }
}