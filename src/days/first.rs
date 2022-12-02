use super::Puzzle;

pub struct Solution;

static INPUT: &str = include_str!("../../input/1");

impl Puzzle for Solution {
    fn run() {
        let mut max = None::<i32>;
        let mut total = 0;

        for line in INPUT.lines() {
            if line.is_empty() {
                max = max.max(Some(total));
                total = 0;
            } else {
                total += line.parse::<i32>().expect("not a number :(");
            }
        }

        println!("{}", total);
    }
}
