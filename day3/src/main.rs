use std::path;
use std::fs;
use std::env;
use regex::Regex;

fn main() {
    let data: String = match get_filename_from_args().and_then(|f| parse(&f)) {
        Err(DataError::CouldNotFindFile(filename)) => {
            println!("Bad file {:?}", filename);
            return;
        }
        Err(DataError::NoInputDataGiven) => {
            println!("No data given, pass an argument");
            return;
        }
        Ok(data) => data
    };
    let r = Regex::new("(mul\\([0-9]{1,3},[0-9]{1,3}\\))|(do\\(\\))|(don't\\(\\))").unwrap();
    let mut tokens = vec![];
    for (_, [capture]) in r.captures_iter(&data.as_str()).map(|c| c.extract()) {
        tokens.push(capture);
    }
    println!("{:?}", tokens);
    let mut answer = 0;
    let mut enabled = true;
    for token in tokens {
        match token {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            mul => {
                if !enabled {
                    continue;
                }

                let tuple = parse_mul(mul);
                answer += tuple.0 * tuple.1;
            }
        }
    }
    println!("{:?}", answer);
}

fn parse_mul(str: &str) -> (i32, i32) {
    let halves: Vec<_> = str.split(',').collect();
    let first = halves[0].split('(').collect::<Vec<_>>()[1];
    let second = halves[1].split(')').collect::<Vec<_>>()[0];
    let f: i32 = first.parse().unwrap();
    let s: i32 = second.parse().unwrap();
    (f, s)
}

#[derive(Debug)]
enum DataError {
    NoInputDataGiven,
    CouldNotFindFile(String),
}

fn parse(filename: &String) -> Result<String, DataError> {
    match fs::read_to_string(filename) {
        Err(_) => Err(DataError::CouldNotFindFile(filename.to_string())),
        Ok(raw_data) => Ok(raw_data)
    }
}

fn get_filename_from_args() -> Result<String, DataError> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.is_empty() {
        print!("{:?}", "pass the input data as the first argument.");
        return Err(DataError::NoInputDataGiven)
    }

    let mut arguments = arguments.iter();
    arguments.next(); // skip the name of the program being ran
    let maybe_filename = arguments.next();
    match maybe_filename {
        Some(filename) => {
            if path::Path::new(filename).exists() {
                Ok(filename.to_string())
            } else {
                Err(DataError::CouldNotFindFile(filename.to_string()))
            }
        },
        None => Err(DataError::NoInputDataGiven)
    }
}