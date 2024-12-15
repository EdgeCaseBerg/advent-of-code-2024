use std::{
    env,
    fs
};

fn get_filename_from_args() -> Option<String> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.is_empty() {
        return None;
    }
    let mut arguments = arguments.iter();
    arguments.next().cloned()
}

pub fn get_sample_if_no_input() -> Result<String, std::io::Error> {
    match get_filename_from_args() {
        None => fs::read_to_string("sample.txt"),
        Some(filename) => fs::read_to_string(filename)
    }
}


mod boilerplate_methods_work {
    #[test]
    fn we_can_test_the_program() {
        assert_eq!(1, 1);
    }
}