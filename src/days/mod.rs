#[feature(first)]
#[path = "first.rs"]
pub mod day;

pub use day::Solution;

pub trait Puzzle {
    fn run();
}
