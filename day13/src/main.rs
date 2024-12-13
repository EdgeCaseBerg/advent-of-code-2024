use std::fs;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let maybe_filename = get_filename_from_args();
    let maybe_filename = Some(String::from("../sample.txt"));
    if maybe_filename.is_none() {
        return Err("No file provided".into());
    }
    let input: String = fs::read_to_string(maybe_filename.unwrap())?;

    Ok(())
}

fn get_filename_from_args() -> Option<String> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.is_empty() {
        return None;
    }
    let mut arguments = arguments.iter();
    arguments.next(); // skip the name of the program being ran
    arguments.next().cloned()
}
