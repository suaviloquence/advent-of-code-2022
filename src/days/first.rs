use super::Solution;

static INPUT: &str = include_str!("../../input/1");

struct First;

impl Solution for First {
	fn run_one() -> i32 {
		let mut max = None;
		let mut total = 0;

		for line in INPUT.lines() {
			if line.is_empty() {
				max = max.max(Some(total));
				total = 0;
			} else {
				total += line.parse::<i32>().expect("Invalid number :(");
			}
		}

		max.unwrap_or_default()
	}

	fn run_two() -> i32 {
		let mut vec = Vec::new();

		let mut total = 0;

		for line in INPUT.lines() {
			if line.is_empty() {
				vec.push(total);
				total = 0;
			} else {
				total += line.parse::<i32>().expect("invalid number :(");
			}
		}

		vec.sort();

		vec.into_iter().rev().take(3).sum::<i32>()
	}
}
