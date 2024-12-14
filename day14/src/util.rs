
use std::env;

pub fn get_filename_from_args() -> Option<String> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.is_empty() {
        return None;
    }
    let mut arguments = arguments.iter();
    arguments.next().cloned()
}

