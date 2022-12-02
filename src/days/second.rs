use super::Solution;

static INPUT: &str = include_str!("../../input/2");

pub struct Second;

impl Solution for Second {
	fn run_one() {
		let mut score = 0u64;

		for line in INPUT.lines() {
			let opponent = line.bytes().nth(0).unwrap() - b'A' + 1;
			let player = line.bytes().nth(2).unwrap() - b'X' + 1;

			match (player + 3 - opponent) % 3 {
				0 => score += 3,
				1 => score += 6,
				2 => score += 0,
				_ => unreachable!(),
			}
			score += player as u64;
		}

		println!("{score}")
	}
	fn run_two() {
		let mut score = 0u64;

		for line in INPUT.lines() {
			let opponent = line.bytes().nth(0).unwrap() - b'A';
			let result = line.bytes().nth(2).unwrap();

			match result {
				b'X' => {
					score += 0;
					score += ((opponent + 2) % 3 + 1) as u64;
				}
				b'Y' => {
					score += 3;
					score += (opponent + 1) as u64;
				}
				b'Z' => {
					score += 6;
					score += ((opponent + 1) % 3 + 1) as u64;
				}
				_ => unreachable!(),
			}
		}
		println!("{score}");
	}
}
