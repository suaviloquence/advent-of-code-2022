use super::Solution;

static INPUT: &str = include_str!("../../input/3");

pub struct Third;

impl Solution for Third {
	fn run_one() -> i32 {
		let mut sum = 0;

		for line in INPUT.lines() {
			let bytes: Vec<_> = line.bytes().collect();
			let (first, second) = bytes.split_at(line.len() / 2);
			let mut first_items = [0u8; 128];
			let mut second_items = [0u8; 128];

			for b in first {
				first_items[*b as usize] += 1;
			}

			for b in second {
				second_items[*b as usize] += 1;
			}

			let (_, dupe) = first_items
				.into_iter()
				.zip(second_items.into_iter())
				.zip(0u8..)
				.filter(|((a, b), _)| *a > 0 && *b > 0)
				.next()
				.unwrap();

			sum += match dupe {
				b'a'..=b'z' => dupe - b'a',
				b'A'..=b'Z' => dupe - b'A' + 26,
				_ => unreachable!(),
			} as i32 + 1;
		}
		sum
	}

	fn run_two() -> i32 {
		let mut sum = 0;

		for chunk in INPUT.lines().collect::<Vec<_>>().chunks(3) {
			let mut freq = [0u8; 128];

			for (i, line) in chunk.into_iter().enumerate() {
				for b in line.bytes() {
					freq[b as usize] |= 1 << i;
				}
			}
			let (_, badge) = freq
				.into_iter()
				.zip(0u8..)
				.filter(|(v, _)| *v == (1 << chunk.len()) - 1)
				.next()
				.unwrap();

			sum += match badge {
				b'a'..=b'z' => badge - b'a',
				b'A'..=b'Z' => badge - b'A' + 26,
				_ => unreachable!(),
			} as i32 + 1;
		}

		sum
	}

	type Disp = i32;
}
