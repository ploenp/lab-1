use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
fn main() {

    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);
    let dir_path = env::current_dir().unwrap();
    println!("path provided: {} in directory: {:?}",args[1], dir_path);
    let file_path = &args[1];
    let file_path_full = if file_path.contains("/") {
        file_path.clone()
    } else {
        format!("{}/{}",dir_path.to_str().unwrap(),file_path)
    };
    let file = File::open(file_path_full);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}