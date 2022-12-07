use std::collections::HashSet;

use super::Solution;

static INPUT: &str = include_str!("../../input/6");

pub struct Sixth;

impl Solution for Sixth {
	type Disp = usize;

	fn run_one() -> Self::Disp {
		let bytes = INPUT.as_bytes();

		for i in 3..bytes.len() {
			let set: HashSet<_> = bytes[(i - 3)..=i].into_iter().collect();

			if set.len() == 4 {
				return i + 1;
			}
		}

		unreachable!()
	}

	fn run_two() -> Self::Disp {
		let bytes = INPUT.as_bytes();

		for i in 13..bytes.len() {
			let mut set = HashSet::new();

			for j in (i - 13)..=i {
				set.insert(bytes[j]);
			}

			if set.len() == 14 {
				return i + 1;
			}
		}

		unreachable!()
	}
}
