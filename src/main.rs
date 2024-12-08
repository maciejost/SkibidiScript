use std::env;
use std::path::Path;

mod find_files;
mod keywords;
mod transpile_file;

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
                    // TODO: Implementer funksjon som kjører keywords::js_keyword på filen, og skriver resultatet til en ny tsx/ts-fil
                    // TODO: Implementer funksjon som kjører swc på direcotryen, og skriver resultatet til en dist.js-fil
                    // TODO: Fjern ts/tsx-filer etter at swc har kjørt

                    if let Err(e) = transpile_file::read_and_print_file(file) {
                        eprintln!("Failed to read {}: {}", file.display(), e);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error accessing directory: {}", e),
    }
}
