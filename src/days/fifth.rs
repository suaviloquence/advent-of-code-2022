use std::collections::VecDeque;

use super::Solution;

static INPUT: &str = include_str!("../../input/5");

pub struct Fifth;

impl Solution for Fifth {
	type Disp = String;

	fn run_one() -> Self::Disp {
		let mut lines = INPUT.lines();

		let mut stacks: Vec<_> = [(); 9].into_iter().map(|_| VecDeque::new()).collect();

		'outer: while let Some(line) = lines.next() {
			for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
				if chunk[1].is_ascii_digit() {
					break 'outer;
				}
				if chunk[1].is_ascii_alphabetic() {
					stacks[i].push_front(chunk[1]);
				}
			}
		}

		// stacks
		// 	.iter()
		// 	.for_each(|x| println!("{}", String::from_utf8_lossy(&x[..])));

		let _ = lines.next();

		for line in lines {
			let ws: [_; 6] = line
				.split_ascii_whitespace()
				.collect::<Vec<_>>()
				.try_into()
				.unwrap();
			let [_, n, _, from, _, to] = ws;
			let [n, from, to] = [
				n.parse::<usize>().unwrap(),
				from.parse().unwrap(),
				to.parse().unwrap(),
			];

			for _ in 0..n {
				let c = stacks[from - 1].pop_back().unwrap();
				stacks[to - 1].push_back(c);
			}
		}

		let stacks = stacks
			.into_iter()
			.map(|mut c| c.pop_back().unwrap())
			.collect();

		String::from_utf8(stacks).unwrap()
	}

	fn run_two() -> Self::Disp {
		let mut lines = INPUT.lines();

		let mut stacks: Vec<_> = [(); 9].into_iter().map(|_| VecDeque::new()).collect();

		'outer: while let Some(line) = lines.next() {
			for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
				if chunk[1].is_ascii_digit() {
					break 'outer;
				}
				if chunk[1].is_ascii_alphabetic() {
					stacks[i].push_front(chunk[1]);
				}
			}
		}

		// stacks
		// 	.iter()
		// 	.for_each(|x| println!("{}", String::from_utf8_lossy(&x[..])));

		let _ = lines.next();

		for line in lines {
			let ws: [_; 6] = line
				.split_ascii_whitespace()
				.collect::<Vec<_>>()
				.try_into()
				.unwrap();
			let [_, n, _, from, _, to] = ws;
			let [n, from, to] = [
				n.parse::<usize>().unwrap(),
				from.parse().unwrap(),
				to.parse().unwrap(),
			];
			let len = stacks[from - 1].len();
			let elems = stacks[from - 1].drain(len - n..).collect::<Vec<u8>>();
			stacks[to - 1].extend(elems);
		}

		let stacks = stacks
			.into_iter()
			.map(|mut c| c.pop_back().unwrap())
			.collect();

		String::from_utf8(stacks).unwrap()
	}
}
