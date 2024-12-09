use regex::Regex;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::keywords::js_keyword;

fn create_ts_file(file_path: &Path) -> io::Result<PathBuf> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap_or(Ok(String::new()))?;

    let extension = if first_line.trim() == "\"use skibidi tsx\";" {
        "tsx"
    } else {
        "ts"
    };

    let new_file_path = file_path.with_extension(extension);

    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&new_file_path)?;

    Ok(new_file_path)
}

pub fn main(file_path: &Path) -> io::Result<()> {
    let input_content = fs::read_to_string(file_path)?;

    // Match all whole words, (levelTenGyat matches, but levelTenGyatWOWZA does not)
    // Hope it works idk how to write regex lol.
    let keyword_regex = Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap();

    let transpiled_content = keyword_regex.replace_all(&input_content, |caps: &regex::Captures| {
        let keyword = &caps[0];
        js_keyword(keyword)
    });

    let transpiled_content = transpiled_content.to_string();

    let new_file = create_ts_file(file_path)?;

    fs::write(new_file, transpiled_content)?;

    Ok(())
}
