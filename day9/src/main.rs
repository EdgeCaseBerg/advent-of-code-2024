use std::fs;
use std::path;
use std::env;

fn main() {
    let no_arg = String::from("../sample.txt");
    let file_contents = get_filename_from_args().or(Some(no_arg)).and_then(|name| load_file_to_str(&name));
    println!("{:?}", file_contents);
}

fn get_filename_from_args() -> Option<String> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.is_empty() {
        println!("No filename passed to file. Defaulting to sample.txt");
        return None;
    }
    let mut arguments = arguments.iter();
    arguments.next(); // skip the name of the program being ran
    arguments.next().cloned()
}

fn load_file_to_str(filename: &String) -> Option<String> {
    if !path::Path::new(filename).exists() {
        println!("File does not exist. {}", filename);
        return None;
    }
    Some(fs::read_to_string(filename).unwrap())
}