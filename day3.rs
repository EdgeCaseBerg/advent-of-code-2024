use std::path;
use std::fs;
use std::env;
use std::collections::VecDeque;

fn main() {
	let data: Vec<char> = match get_filename_from_args().and_then(|f| parse(&f)) {
		Err(DataError::CouldNotFindFile(filename)) => {
			println!("Bad file {:?}", filename);
			return;
		}
		Err(DataError::NoInputDataGiven) => {
			println!("No data given, pass an argument");
			parse(&String::from("input-day-3.txt")).unwrap().chars().collect()
		}
		Ok(data) => data.chars().collect()
	};

	
	let mut answer = 0;
	let mut enabled = true;
	// let mut statement = String::new();
	let mut tokens: VecDeque<SingleToken> = data.into_iter().map(|c| SingleToken::new(c)).collect();
	// let commands: Vec<SingleToken> = Vec::new();
	for _ in 0..tokens.len() {
		let maybe_token = tokens.pop_front();
		if maybe_token.is_none() {
			continue;
		}

		match maybe_token.unwrap() {
			SingleToken::Meaningless => {}, // Ignore or reset if processing?
			SingleToken::HasMeaning(c) => {
				match c {
					'm' => {
						tokens.push_front(SingleToken::new(c)); // add m in for parsing
						match attempt_m_parse(&tokens) {
							Ok((fst, snd)) => {
								if enabled {
									println!("mul({:?},{:?})", fst, snd);
									answer += fst * snd
								}
							}
							_ => {}
						}
						tokens.pop_front(); // remove the m again 
					}
					'd' => {
						tokens.push_front(SingleToken::new(c)); // add d in for parsing
						match attempt_d_parse(&tokens) {
							None => {}
							Some(enable) => {
								if (enable) {
									println!("do()");
								} else {
									println!("don't()");
								}
								enabled = enable;
								enabled = true;
							}
						};
						tokens.pop_front(); // remove the token again
					}
					_ => {}// this token is actually meaningless
				}
			}
		}
	}
	answer += 1;
	println!("{:?}", answer);
}

fn attempt_d_parse(data: &VecDeque<SingleToken>) -> Option<bool> {
	let potental_do = data.iter().take("do()".len()).fold(String::new(), |mut a, t| {
		a.push(t.value());
		a
	});
	let potental_do = potental_do.as_str();
	let potental_dont = data.iter().take("don't()".len()).fold(String::new(), |mut a, t| {
		a.push(t.value());
		a
	});
	let potental_dont = potental_dont.as_str();
	let enable = match potental_do {
		"do()" => true,
		_ => false
	};

	let disable = match potental_dont {
		"don't()" => true,
		_ => false
	};

	match (enable, disable) {
		(true, false) => Some(true),
		(false, true) => Some(false),
		(false, false) => None,
		(true, true) => None
	}
}

fn attempt_m_parse(data: &VecDeque<SingleToken>) -> Result<(i32, i32), &str> {
	let mut potential: Vec<&SingleToken> = data.iter().take(12).collect();
	if !potential.iter().all(|t| t.has_meaning()) {
		return Err("Lacks meaning");
	}

	for _ in 0.."mul(".len() {
		potential.remove(0);
	}
	let mut potential = potential.into_iter();

	let mut number_of_first_tokens = 0;
	let mut first_number_str = String::new();
	let mut current_token: &SingleToken = &SingleToken::Meaningless;
	loop {
		let maybe_token = potential.next();
		if maybe_token.is_none() {
			break;
		}

		current_token = maybe_token.unwrap();

		let valid_token = match current_token {
			SingleToken::Meaningless => {
				false
			},
			SingleToken::HasMeaning(digit) => digit.is_numeric()
		};
		if !valid_token {
			break;
		}
		number_of_first_tokens += 1;
		first_number_str.push(current_token.value());
	}

	if number_of_first_tokens > 3 || number_of_first_tokens == 0 {
		return Err("digit does not match constraints");
	}

	match current_token {
		SingleToken::HasMeaning(',') => {},
		_ => {
			return Err("comma not found between digits");
		}
	}

	let mut second_number_tokens = 0;
	let mut second_number_str = String::new();
	loop {
		let maybe_token = potential.next();
		if maybe_token.is_none() {
			break;
		}

		current_token = maybe_token.unwrap();

		let valid_token = match current_token {
			SingleToken::Meaningless => {
				false
			},
			SingleToken::HasMeaning(digit) => digit.is_numeric()
		};
		if !valid_token {
			break;
		}
		second_number_tokens += 1;
		second_number_str.push(current_token.value());
	}

	if second_number_tokens > 3 || second_number_tokens == 0 {
		return Err("digit does not match constraints");
	}

	match first_number_str.parse() {
		Ok(fst) => {
			match second_number_str.parse() {
				Ok(snd) => Ok((fst, snd)),
				_ => Err("Cant parse second number")
			}
		},
		_ => Err("Cant parse first number")
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

}

// 190604937
// 82857512

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