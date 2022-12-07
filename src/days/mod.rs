use std::fmt::Display;

pub mod fifth;
pub mod first;
pub mod fourth;
pub mod second;
pub mod sixth;
pub mod third;

pub trait Solution {
	type Disp: Display;

	fn run_one() -> Self::Disp;
	fn run_two() -> Self::Disp;

	fn print() {
		println!("{}", Self::run_one());
		println!("{}", Self::run_two());
	}
}
