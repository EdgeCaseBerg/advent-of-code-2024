use std::fs;
use std::path;
use std::env;

fn main() {
	let input = get_filename_from_args()
		.and_then(|filename| load_data_from_file(&filename));

	let data = match input {
		Err(data_error) => {
			match data_error {
				DataError::CouldNotFindFile(badfile) => {
					println!("We could not fine the file {:?}", badfile);
				},
				DataError::NoInputDataGiven => {
					println!("No input file given, please pass one as an argument!");
				},
				DataError::InvalidLeftData(bad) => {
					print!("We could not parse a value from the left list {:?}", bad);
				},
				DataError::InvalidRightData(bad) => {
					print!("We could not parse a value from the right list {:?}", bad);
				}
			};
			return;
		}
		Ok(data) => data
	};

	println!("{:?}", data.distance());
	println!("{:?}", data.similarity());
}


#[derive(Debug)]
struct InputData {
	left: Vec<i32>,
	right: Vec<i32>
}

impl InputData {
	fn distance(&self) -> i32 {
		let mut left_copy = self.left.clone();
		left_copy.sort();
		let mut right_copy = self.right.clone();
		right_copy.sort();
		let mut distance = 0;
		assert!(left_copy.len() == right_copy.len());
		for idx in 0..left_copy.len() {
			distance += (left_copy[idx] - right_copy[idx]).abs();
		}
		return distance
	}

	fn similarity(&self) -> i32 {
		let mut left_copy = self.left.clone();
		left_copy.sort();
		let mut right_copy = self.right.clone();
		right_copy.sort();

		let mut similarity = 0;
		for left_idx in 0..left_copy.len() {
			let left_value = left_copy[left_idx];
			let mut scalar = 0;
			for right_idx in 0..right_copy.len() {
				if left_value == right_copy[right_idx] {
					scalar += 1;
				}
			}
			similarity += left_value * scalar;
		}
		return similarity
	}
}

#[derive(Debug)]
enum DataError {
	NoInputDataGiven,
	CouldNotFindFile(String),
	InvalidLeftData(String),
	InvalidRightData(String)
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


fn load_data_from_file(filename: &String) -> Result<InputData, DataError> {
	match fs::read_to_string(filename) {
		Ok(contents) => {
			let mut left_data = Vec::new();
			let mut right_data = Vec::new();

			let mut line: String = String::new();
			for c in contents.chars() {
				if c != ' ' && c != '\r' && c != '\n' {
					line.push(c);
				} else {
					if line.is_empty() {
						continue;
					}

					// time to parse.
					let is_left = c == ' ';
					if is_left {
						let value: i32 = match line.parse() {
							Ok(good) => good,
							Err(_) => { // it'd be nice if we make our invalid data return what the invalid data was
								return Err(DataError::InvalidLeftData(line))
							}
						};
						left_data.push(value);
						line.clear();
					} else if c == '\n' {
						let value: i32 = match line.parse() {
							Ok(good) => good,
							Err(_) => {
								return Err(DataError::InvalidRightData(line))
							}
						};
						right_data.push(value);
						line.clear();
					}
				}
			}

			Ok(InputData {
				left: left_data,
				right: right_data
			})
		},
		Err(error) => {
			println!("{:?}", error);
			Err(DataError::CouldNotFindFile(filename.to_string()))
		}
	}
}
