use aoc24::{get_input, Day};
use std::collections::HashMap;

#[derive(Debug)]
struct DayOne {
    left: Vec<usize>,
    right: Vec<usize>,
}
impl Day for DayOne {
    type Res = usize;
    fn part_one(&self) -> Self::Res {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }
    fn part_two(&self) -> Self::Res {
        let mut counts_right = HashMap::new();
        for num in &self.right {
            *counts_right.entry(num).or_insert(0) += 1;
        }
        self.left
            .iter()
            .map(|&num| num * counts_right.get(&num).unwrap_or(&0))
            .sum()
    }

    fn from_input() -> Result<Self, std::io::Error> {
        let data = get_input(1)?;
        let lines: Vec<&str> = data.lines().collect();

        let mut left: Vec<usize> = Vec::with_capacity(lines.len());
        let mut right: Vec<usize> = Vec::with_capacity(lines.len());

        for line in &lines {
            let split = line.split_whitespace().collect::<Vec<&str>>();

            assert_eq!(split.len(), 2);

            let a = split[0].parse::<usize>().unwrap();
            let b = split[1].parse::<usize>().unwrap();

            left.push(a);
            right.push(b);
        }
        assert_eq!(left.len(), right.len());
        assert_eq!(lines.len(), left.len());
        assert_eq!(lines.len(), right.len());

        left.sort();
        right.sort();

        Ok(DayOne { left, right })
    }
}
fn main() -> Result<(), std::io::Error> {
    let day_one = DayOne::from_input()?;
    aoc24::run(day_one)?;

    Ok(())
}
