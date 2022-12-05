use super::Solution;
static INPUT: &str = include_str!("../../input/4");

pub struct Fourth;

impl Solution for Fourth {
	fn run_one() -> i32 {
		let mut n = 0;
		for line in INPUT.lines() {
			let (first, second) = line.split_once(",").unwrap();
			let (low_1, high_1) = first.split_once("-").unwrap();
			let (low_2, high_2) = second.split_once("-").unwrap();

			let low_1 = low_1.parse::<i32>().unwrap();
			let high_1 = high_1.parse::<i32>().unwrap();
			let low_2 = low_2.parse::<i32>().unwrap();
			let high_2 = high_2.parse::<i32>().unwrap();

			if (low_1 <= low_2 && high_1 >= high_2) || (low_2 <= low_1 && high_2 >= high_1) {
				n += 1;
			}
		}

		n
	}
	fn run_two() -> i32 {
		let mut n = 0;

		for line in INPUT.lines() {
			let (first, second) = line.split_once(",").unwrap();
			let (low_1, high_1) = first.split_once("-").unwrap();
			let (low_2, high_2) = second.split_once("-").unwrap();

			let low_1 = low_1.parse::<i32>().unwrap();
			let high_1 = high_1.parse::<i32>().unwrap();
			let low_2 = low_2.parse::<i32>().unwrap();
			let high_2 = high_2.parse::<i32>().unwrap();

			if (low_1..=high_1).contains(&low_2)
				|| (low_1..=high_1).contains(&high_2)
				|| (low_2..=high_2).contains(&low_1)
				|| (low_2..=high_2).contains(&high_1)
			{
				n += 1
			}
		}

		n
	}
}
