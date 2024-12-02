use aoc24::{get_input, Day};

#[derive(Debug)]
struct DayTwo {}

impl Day<usize> for DayTwo {
    fn part_one(&self) -> usize {
        todo!()
    }
    fn part_two(&self) -> usize {
        todo!()
    }

    fn from_input() -> Result<Self, std::io::Error> {
        let data = get_input(2)?;
        todo!()
    }
}
fn main() -> Result<(), std::io::Error> {
    let day_one = DayTwo::from_input()?;
    aoc24::run(day_one)?;

    Ok(())
}
