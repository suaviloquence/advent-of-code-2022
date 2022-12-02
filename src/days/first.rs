static INPUT: &str = include_str!("../../input/1");

pub fn run(part: usize) {
	match part {
		1 => part_1(),
		2 => part_2(),
		n => panic!("Unknown part {n}"),
	}
}

fn part_1() {
	let mut max = None::<i32>;
	let mut total = 0;

	for line in INPUT.lines() {
		if line.is_empty() {
			max = max.max(Some(total));
			total = 0;
		} else {
			total += line.parse::<i32>().expect("Invalid number :(");
		}
	}

	println!("{}", max.unwrap_or_default());
}

fn part_2() {
	let mut vec = Vec::new();

	let mut total = 0i32;

	for line in INPUT.lines() {
		if line.is_empty() {
			vec.push(total);
			total = 0;
		} else {
			total += line.parse::<i32>().expect("invalid number :(");
		}
	}

	vec.sort();

	println!("{}", vec.into_iter().rev().take(3).sum::<i32>());
}
