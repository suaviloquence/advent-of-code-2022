pub mod fifth;
pub mod first;
pub mod fourth;
pub mod second;
pub mod third;

pub trait Solution {
	fn run_one() -> i32;
	fn run_two() -> i32;

	fn print() {
		println!("{}", Self::run_one());
		println!("{}", Self::run_two());
	}
}
