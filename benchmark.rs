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
		for strongly in [false, true] {
			if let Err(err) =
				test_file(
					test,
					format!("{}/data/{}", env!("CARGO_MANIFEST_DIR"), test),
					strongly,
				)
			{
				println!("{}", err);
				break
			}
		}
	}

}



fn test_file(title: &str, filename: String, strongly: bool) -> Result<(), String> {
	let start = std::time::Instant::now();
	fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where P: AsRef<Path>, {
		let file = File::open(filename)?;
		Ok(io::BufReader::new(file).lines())
	}
	let mut sum_durations = 0u128;
	let mut sum_positions_checked = 0u128;
	let mut count = 0u16;

	for line in read_lines(filename).unwrap() {
		if start.elapsed().as_secs() > 1_200 { break }

		let line_str = line.unwrap();
		let mut split = line_str.split(' ');
		let pos_str = split.next().unwrap().to_string();
		let expected_outcome: Outcome = split.next().unwrap().parse::<i8>().unwrap().into();
		let now = std::time::Instant::now();
		let (_best_mov, positions_checked, actual_outcome) =
			if strongly {
				Solver::solve_str(pos_str).unwrap()
			} else {
				Solver::weakly_solve_str(pos_str).unwrap()
			};
		let duration = now.elapsed().as_nanos();

		if strongly {
			if actual_outcome != expected_outcome {
				return Err(format!(
					"{}:{}: {}\n\
						\texpected {:?}, got {:?}",
					title,
					count,
					line_str.trim(),
					actual_outcome,
					expected_outcome
				))
			}
		} else {
			if std::mem::discriminant(&actual_outcome) != std::mem::discriminant(&expected_outcome) {
				return Err(format!(
					"{}:{}: {}\n\
						\texpected {:?}, got {:?} (weak solver)",
					title,
					count,
					line_str.trim(),
					actual_outcome,
					expected_outcome
				))
			}
		}

		sum_durations += duration;
		sum_positions_checked += positions_checked;
		count += 1;
	}

	let mean_nanos = sum_durations as f64/ count as f64;

	println!("test={} mean_time={} mean_nb_pos={:.1} strongly_solved={} completion={}",
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
		strongly,
		count,
	);

	match count {
		1_000 => Ok(()),
		_ => Err("Could not finish in time.".to_string())
	}
}
