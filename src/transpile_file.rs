use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_and_print_file(file_path: &Path) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    println!("Contents of {}:", file_path.display());
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
