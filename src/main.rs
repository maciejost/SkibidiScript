use std::path::Path;

#[macro_use]
extern crate lazy_static;

use std::env;
mod find_files;
mod keywords;
mod transpile_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <program> <directory-path>");
        return;
    }

    let dir_path = Path::new(&args[1]);

    match find_files::list_skibidi_files_in_directory(dir_path) {
        Ok(files) => {
            if files.is_empty() {
                println!("No SkibidiScript files found in the directory.");
            } else {
                for file in &files {
                    if let Err(e) = transpile_file::read_and_print_file(file) {
                        eprintln!("Failed to read {}: {}", file.display(), e);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error accessing directory: {}", e),
    }
}
