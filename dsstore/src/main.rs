extern crate glob;

use std::env;
use std::result::Result;
use std::fs;
use glob::glob;

fn main() {
    let cwd = env::current_dir().unwrap();
    let glob_path = cwd.join("**/.DS_Store");
    let glob_str = glob_path.to_str().unwrap();

    println!("Removing .DS_Store from {:?}", glob_path.display());

    for entry in glob(&glob_str).unwrap().filter_map(Result::ok) {
        match fs::remove_file(entry.as_path()) {
            Ok(_) => println!("Removed {:?}", entry.display()),
            Err(e) => println!("Could not remove {}: {:?}", entry.display(), e.to_string())
        }
    }
}