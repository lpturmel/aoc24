use std::io::Error;

const INPUT_FOLDER: &str = "inputs";

/// Gets the specific day input from the input folder
/// based on the day number
pub fn get_input(day: u32) -> Result<String, Error> {
    let path = format!("{}/{:02}.txt", INPUT_FOLDER, day);
    std::fs::read_to_string(path)
}

/// Represents a day of the AoC challenge
pub trait Day {
    type Res;
    /// Constructs the day from it's input
    fn from_input() -> Result<Self, std::io::Error>
    where
        Self: std::marker::Sized;
    fn part_one(&self) -> Self::Res;
    fn part_two(&self) -> Self::Res;
}

/// Run the day challenge with standard output
pub fn run<D>(day: D) -> Result<(), Error>
where
    D: Day,
    D::Res: std::fmt::Debug,
{
    let part_one = day.part_one();
    println!("P1: {:?}", part_one);

    let part_two = day.part_two();
    println!("P2: {:?}", part_two);
    Ok(())
}
