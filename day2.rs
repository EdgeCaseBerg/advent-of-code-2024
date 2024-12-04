use std::path;
use std::fs;
use std::env;

fn main() {
	let reports = match get_reports() {
		Err(DataError::CannotParseLine(bad_line)) => {
			println!("Bad line {:?}", bad_line);
			return;
		}
		Err(DataError::CouldNotFindFile(filename)) => {
			println!("Bad file {:?}", filename);
			return;
		}
		Err(DataError::NoInputDataGiven) => {
			println!("No data given, pass an argument");
			return;
		}
		Ok(reports) => reports
	};
	let mut num_safe_reports = 0;
	for report in &reports {
		if is_report_safe(&report) {
			num_safe_reports += 1;
		} else {
			let mut can_tolerate = false;
			for foo in &permute_report(&report) {
				can_tolerate = can_tolerate || is_report_safe(&foo);
			}
			if can_tolerate {
				num_safe_reports += 1;
			}
		}
	}
	// Not 1273
	println!("{:?}", num_safe_reports);
}

fn permute_report(report: &Vec<i32>) -> Vec<Vec<i32>> {
	let mut permutations = Vec::new();
	permutations.push(report.clone());
	for idx in 0..report.len() {
		let mut clone = report.clone();
		clone.remove(idx);
		permutations.push(clone);
	}
	return permutations;
}

fn is_report_safe(report: &Vec<i32>) -> bool {
	if report.is_empty() {
		return false;
	}

	if report.len() == 1 {
		return true; // I think?
	}

	let mut differences = Vec::new();
	for idx in 1..report.len() {
		let difference = report[idx - 1] - report[idx];
		differences.push(difference);
	}

	let is_within_range = differences.clone().iter().map(|d| d.abs()).all(|diff| diff > 0 && diff < 4);
	let is_increasing_across_all = differences.clone().iter().all(|diff| *diff < 0);
	let is_decreasing_across_all = differences.clone().iter().all(|diff| *diff > 0);

	return is_within_range && (is_decreasing_across_all || is_increasing_across_all);
}


fn get_reports() -> Result<Vec<Vec<i32>>, DataError>{
	match get_filename_from_args() {
		Err(problem) => Err(problem),
		Ok(filename) => parse(&filename)
	}
}

#[derive(Debug)]
enum DataError {
	NoInputDataGiven,
	CouldNotFindFile(String),
	CannotParseLine(String),
}

fn parse(filename: &String) -> Result<Vec<Vec<i32>>, DataError> {
	let newline = "\r\n";
	match fs::read_to_string(filename) {
		Err(_) => Err(DataError::CouldNotFindFile(filename.to_string())),
		Ok(raw_data) => {
			let mut reports = Vec::new();
			let lines: Vec<&str> = raw_data.split(newline).collect();
			for line in lines {
				let report: Result<Vec<i32>, DataError> = line
					.split(' ')
					.map(|s| s.parse::<i32>().map_err(|_| DataError::CannotParseLine(line.to_string())))
					.collect();
				reports.push(report?);
			}
			return Ok(reports);
		}
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