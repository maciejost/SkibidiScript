use std::fs::{self};
use std::io::{self};
use std::path::{Path, PathBuf};

pub fn list_skibidi_files_in_directory(dir_path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut skibidi_files = Vec::new();

    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file()
                && entry_path.extension().and_then(|e| e.to_str()) == Some("skibidi")
            {
                skibidi_files.push(entry_path);
            }
        }
    }

    Ok(skibidi_files)
}
