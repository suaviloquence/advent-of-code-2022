use super::Solution;

static INPUT: &str = include_str!("../../input/2");

pub struct Second;

impl Solution for Second {
	fn run_one() -> i32 {
		let mut score = 0;

		for line in INPUT.lines() {
			let opponent = line.bytes().nth(0).unwrap() - b'A' + 1;
			let player = line.bytes().nth(2).unwrap() - b'X' + 1;

			match (player + 3 - opponent) % 3 {
				0 => score += 3,
				1 => score += 6,
				2 => score += 0,
				_ => unreachable!(),
			}
			score += player as i32;
		}

		score
	}
	fn run_two() -> i32 {
		let mut score = 0;

		for line in INPUT.lines() {
			let opponent = line.bytes().nth(0).unwrap() - b'A';
			let result = line.bytes().nth(2).unwrap();

			match result {
				b'X' => {
					score += 0;
					score += ((opponent + 2) % 3 + 1) as i32;
				}
				b'Y' => {
					score += 3;
					score += (opponent + 1) as i32;
				}
				b'Z' => {
					score += 6;
					score += ((opponent + 1) % 3 + 1) as i32;
				}
				_ => unreachable!(),
			}
		}
		score
	}
}
