use std::fs;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let maybe_filename = get_filename_from_args();
    if maybe_filename.is_none() {
        return Err("No file provided".into());
    }
    let message: String = fs::read_to_string(maybe_filename.unwrap())?;
    println!("{}", message);
    Ok(())
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