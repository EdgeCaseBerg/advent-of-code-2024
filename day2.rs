fn main() {
	let reports = get_reports();
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

	let is_within_range = differences.clone().into_iter().map(|d| d.abs()).all(|diff| diff > 0 && diff < 4);
	let is_increasing_across_all = differences.clone().into_iter().all(|diff| diff < 0);
	let is_decreasing_across_all = differences.clone().into_iter().all(|diff| diff > 0);

	return is_within_range && (is_decreasing_across_all || is_increasing_across_all);
}


fn get_reports() -> Vec<Vec<i32>>{
	return Vec::from([
		Vec::from([74,76,78,79,76])
	])
}