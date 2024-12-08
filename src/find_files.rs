use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn list_skibidi_files_in_directory() -> std::io::Result<Vec<PathBuf>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a valid dir.");
        return Ok(Vec::new());
    }

    let dir_path = &args[1];
    let path = Path::new(dir_path);

    let mut skibidi_files = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file()
                && entry_path.extension().and_then(|e| e.to_str()) == Some("skibidi")
            {
                skibidi_files.push(entry_path);
            }
        }
    } else {
        println!("Please provide a valid dir.");
    }

    Ok(skibidi_files)
}

pub fn main() {
    match list_skibidi_files_in_directory() {
        Ok(files) => {
            if files.is_empty() {
                println!("No SkibidiScript files found in dir");
            } else {
                for file in files {
                    println!("{}", file.display());
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
