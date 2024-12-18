use std::env;
use std::path::Path;

mod find_files;
mod keywords;
mod transpile_file;
mod transpile_to_js;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("You must provide a directory path as an argument.");
        return;
    }

    let dir_path = Path::new(&args[1]);

    match find_files::list_skibidi_files_in_directory(dir_path) {
        Ok(files) => {
            if files.is_empty() {
                println!("No SkibidiScript files found in the directory.");
            } else {
                for file in &files {
                    if let Err(e) = transpile_file::main(file) {
                        eprintln!("Failed to read {}: {}", file.display(), e);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error accessing directory: {}", e),
    }
}
