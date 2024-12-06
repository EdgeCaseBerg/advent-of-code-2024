use std::path;
use std::fs;
use std::env;
use regex::Regex;

fn main() {
    original_answer();
    silly_fun_parser_answer();
}

fn original_answer() {
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
///////

fn silly_fun_parser_answer() {
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
    let all_tokens: &[SingleToken] = &data.chars()
        .map(|c| SingleToken::new(c))
        .collect::<Vec<SingleToken>>();
    let mut tokens = vec![];
    let mut tokens_cursor = all_tokens.iter();
    loop {
        if tokens_cursor.len() == 0 { 
            break;
        }
        let token = parse_dont(tokens_cursor.as_slice())
            .or(parse_do(tokens_cursor.as_slice()))
            .or(parse_m(tokens_cursor.as_slice()));
        match token {
            Some(token) => {
                let advance_by = token.consumed();
                tokens.push(token);
                for _ in 0..advance_by {
                    tokens_cursor.next();
                }
            }
            None => {
                tokens_cursor.next();
            },
        }
    }

    let mut answer = 0;
    let mut enabled = true;
    for token in tokens {
        match token {
            Command::Do => enabled = true,
            Command::Dont => enabled = false,
            Command::Mul(fst, snd, _) => {
                if !enabled {
                    continue;
                }
                answer += fst * snd;
            }
        }
    }
    println!("{:?}", answer);    
}

fn is_digit_radix10(c: char) -> bool {
    c.is_digit(10)
}

fn parse_do(tokens: &[SingleToken]) -> Option<Command> {
    CompoundToken::matches(tokens, "do()").lift().map(|_| {
        Command::Do
    })
}

fn parse_dont(tokens: &[SingleToken]) -> Option<Command> {
    CompoundToken::matches(tokens, "don't()").lift().map(|_| {
        Command::Dont
    })
}

fn parse_m(tokens: &[SingleToken]) -> Option<Command> {
    CompoundToken::matches(tokens, "mul(").lift().and_then(|_| {
        let tokens = &tokens[4..];
        CompoundToken::matches_func_while(tokens, is_digit_radix10).lift().and_then(|digits| {
            let characters = digits.len();
            if characters <= 0 || characters > 4 {
                return None;
            }
            let first_digits: i32 = digits.iter().collect::<String>().parse().unwrap();
            let tokens = &tokens[characters..];
            let mut tokens_used: i32 = 4;
            CompoundToken::matches(tokens, ",").lift().and_then(|_| {
                let tokens = &tokens[1..];
                tokens_used += 1;
                CompoundToken::matches_func_while(tokens, is_digit_radix10).lift().and_then(|digits| {
                    let characters = digits.len();
                    if characters <= 0 || characters > 4 {
                        return None;
                    }
                    for _ in digits.iter() {
                        tokens_used += 1;
                    }
                    let second_digit: i32 = digits.iter().collect::<String>().parse().unwrap();
                    let skip = characters as usize;
                    let tokens = &tokens[skip..];
                    CompoundToken::matches(tokens, ")").lift().and_then(|_| {
                        tokens_used += 1;
                        Some(Command::Mul(first_digits, second_digit, tokens_used))
                    })
                })
            })
        })
    })
}

#[derive(Debug)]
enum Command {
    Mul(i32, i32, i32),
    Do,
    Dont,
}

impl Command {
    fn consumed(&self) -> i32 {
        match self {
            Command::Do => 4,
            Command::Dont => 7,
            Command::Mul(_,_, tokens_used) => *tokens_used,
        }
    }
}

#[derive(Debug)]
enum CompoundToken {
    Null,
    Token(Vec<char>)
}

impl CompoundToken {
    fn lift(&self) -> Option<Vec<char>> {
        match self {
            CompoundToken::Null => None,
            CompoundToken::Token(chars) => Some(chars.to_vec())
        }
    }

    fn matches(token: &[SingleToken], string: &str) -> CompoundToken {
        let num_tokens = token.len();
        let num_chars = string.chars().count();
        let mut chars = string.chars();
        let mut matching = Vec::new();
        for idx in 0..num_chars.min(num_tokens) {
            match chars.next() {
                None => return CompoundToken::Null,
                Some(char_to_match) => {
                    if token[idx].is_value(&char_to_match) {
                        matching.push(char_to_match)
                    } else {
                        return CompoundToken::Null
                    }
                }
            }
        }
        return CompoundToken::Token(matching);
    }

    fn matches_func_while(tokens: &[SingleToken], f: fn(char) -> bool) -> CompoundToken {
        let mut matching = Vec::new();
        for token in tokens.iter() {
            if token.has_meaning() {
                if f(token.value()) {
                    matching.push(token.value());   
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if matching.is_empty() {
            CompoundToken::Null
        } else {    
            CompoundToken::Token(matching)
        }
    }
}

#[derive(Debug)]
enum SingleToken {
    Meaningless,
    HasMeaning(char)
}

impl SingleToken {
    fn new(c: char) -> SingleToken {
        match c {
            'm' | 'u' | 'l' | '(' | ',' | ')' | '0'..='9' => SingleToken::HasMeaning(c),
            'd' | 'o' | 'n' | '\''| 't' => SingleToken::HasMeaning(c),
            _ => SingleToken::Meaningless
        }
    }

    fn has_meaning(&self) -> bool {
        match self {
            SingleToken::Meaningless => false,
            _ => true
        }
    }

    fn value(&self) -> char {
        match self {
            SingleToken::HasMeaning(c) => *c,
            SingleToken::Meaningless => '\0'
        }
    }

    fn is_value(&self, value: &char) -> bool {
        match self {
            SingleToken::HasMeaning(c) => c == value,
            _ => false
        }
    }

}
