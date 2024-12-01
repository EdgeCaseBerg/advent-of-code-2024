use std::fs;
use std::path;
use std::env;
use std::collections::HashMap;

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

	println!("{:?}", data.total_distance());
	println!("{:?}", data.similarity());
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Copy)]
struct LocationID {
	id: i32
}

impl LocationID {
	fn distance(self, other: LocationID) -> i32 {
		return (self.id - other.id).abs();
	}
}


#[derive(Debug)]
struct InputData {
	left: Vec<LocationID>,
	right: Vec<LocationID>,
	count_by_right_location: HashMap<LocationID, i32>
}

impl InputData {
	fn new() -> InputData {
		return InputData {
			left: Vec::new(),
			right: Vec::new(),
			count_by_right_location: HashMap::new()
		}
	}

	fn add_to_left(&mut self, id: LocationID) {
		self.left.push(id);
	}

	fn add_to_right(&mut self, id: LocationID) {
		self.right.push(id);
		self.count_by_right_location.entry(id).and_modify(|count| { *count += 1 }).or_insert(1);
	}

	fn total_distance(&self) -> i32 {
		let mut left_copy = self.left.clone();
		left_copy.sort();
		let mut right_copy = self.right.clone();
		right_copy.sort();
		let mut distance = 0;
		assert!(left_copy.len() == right_copy.len());
		for idx in 0..left_copy.len() {
			distance += left_copy[idx].distance(right_copy[idx]);
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
			let left_value: LocationID = left_copy[left_idx];
			match self.count_by_right_location.get(&left_value) {
				Some(scalar) => {
					similarity += left_value.id * scalar;
				},
				None => {}
			}
			
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
		Err(_) => Err(DataError::CouldNotFindFile(filename.to_string())),
		Ok(contents) => {
			let mut input_data = InputData::new();
			
			let mut line: String = String::new();
			for c in contents.chars() {
				if c != ' ' && c != '\r' && c != '\n' {
					line.push(c);
				} else {
					if line.is_empty() {
						continue;
					}
					let is_left = c == ' ';
					if is_left {
						let value = match line.parse() {
							Ok(good) => LocationID { id: good },
							Err(_) => {
								return Err(DataError::InvalidLeftData(line))
							}
						};
						input_data.add_to_left(value);
						line.clear();
					} else if c == '\n' {
						let value = match line.parse() {
							Ok(good) => LocationID { id: good },
							Err(_) => {
								return Err(DataError::InvalidRightData(line))
							}
						};
						input_data.add_to_right(value);
						line.clear();
					}
				}
			}

			Ok(input_data)
		}
	}
}
