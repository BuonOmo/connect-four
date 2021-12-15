use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod position;
mod solver;
use crate::solver::{Outcome, Solver};

fn main() {
	for test in [
		"end_easy",
		"middle_easy",
		"middle_medium",
		"begin_easy",
		"begin_medium",
		"begin_hard",
	] {
		test_file(test, format!("{}/data/{}", env!("CARGO_MANIFEST_DIR"), test), true);
	}

}



fn test_file(title: &str, filename: String, strongly: bool) {
	let start = std::time::Instant::now();
	fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where P: AsRef<Path>, {
		let file = File::open(filename)?;
		Ok(io::BufReader::new(file).lines())
	}
	let mut sum_durations = 0u128;
	let mut sum_positions_checked = 0u128;
	let mut count = 0u128;
	// let durations: Vec<(u128, u128)> = read_lines(filename).unwrap().map(|line| {
	// 	let line_str = line.unwrap();
	// 	let mut split = line_str.split(' ');
	// 	let pos_str = split.next().unwrap().to_string();
	// 	let expected_outcome: Outcome = split.next().unwrap().parse::<i8>().unwrap().into();
	// 	let now = std::time::Instant::now();
	// 	let (positions_checked, actual_outcome) = Solver::solve(pos_str).unwrap();
	// 	let duration = now.elapsed().as_nanos();

	// 	assert_eq!(actual_outcome, expected_outcome);

	// 	(duration, positions_checked)
	// }).collect();

	for line in read_lines(filename).unwrap() {
		if start.elapsed().as_secs() > 60 { break }

		let line_str = line.unwrap();
		let mut split = line_str.split(' ');
		let pos_str = split.next().unwrap().to_string();
		let expected_outcome: Outcome = split.next().unwrap().parse::<i8>().unwrap().into();
		let now = std::time::Instant::now();
		let (positions_checked, actual_outcome) = Solver::solve(pos_str).unwrap();
		let duration = now.elapsed().as_nanos();

		if strongly {
			assert_eq!(actual_outcome, expected_outcome);
		} else {
			// TODO
		}

		sum_durations += duration;
		sum_positions_checked += positions_checked;
		count += 1;
	}

	let mean_nanos = sum_durations as f64/ count as f64;
		// durations.iter().map(|tuple|tuple.0).sum::<u128>() as f64 / durations.len() as f64;

	println!("test={} mean_time={} mean_nb_pos={:.1} completion={:.2}%",
		title,
		if mean_nanos > 1e9 {
			format!("{:.4}s", mean_nanos / 1e9)
		} else if mean_nanos > 1e6 {
			format!("{:.4}ms", mean_nanos / 1e6)
		} else if mean_nanos > 1e3 {
			format!("{:.4}μs", mean_nanos / 1e3)
		} else {
			format!("{:.4}ns", mean_nanos)
		},
		sum_positions_checked as f64 / count as f64,
		// durations.iter().map(|tuple|tuple.1).sum::<u128>() as f64 / durations.len() as f64,
		count as f64 / 1_0.0
	)
}
