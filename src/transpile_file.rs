use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn create_ts_file(file_path: &Path) -> io::Result<File> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap_or(Ok(String::new()))?;

    println!("{}", first_line);

    let extension = if first_line.trim() == "\"use skibidi tsx\";" {
        "tsx"
    } else {
        "ts"
    };

    let new_file_path = file_path.with_extension(extension);

    let new_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(new_file_path)?;

    Ok(new_file)
}

pub fn read_and_print_file(file_path: &Path) -> io::Result<()> {
    //let file = File::open(file_path)?;
    //let reader = io::BufReader::new(file);

    println!("Skibidi contents of {}:", file_path.display());

    // for line in reader.lines() {
    //     println!("{}", line?);
    // }

    create_ts_file(file_path);

    Ok(())
}
