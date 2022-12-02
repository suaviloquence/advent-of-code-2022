pub mod first;
pub mod second;

pub trait Solution {
	fn run_one();
	// default impl so not necessary
	fn run_two() {}

	fn run() {
		Self::run_one();
		Self::run_two();
	}
}
