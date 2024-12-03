use aoc24::{get_input, Day};

#[derive(Debug)]
struct DayTwo {
    data: Vec<Vec<usize>>,
}

impl DayTwo {
    fn is_valid(row: &[usize]) -> bool {
        let diffs = row
            .windows(2)
            .map(|w| w[1] as isize - w[0] as isize)
            .collect::<Vec<_>>();

        let first_diff = diffs[0];

        let is_increasing = first_diff > 0;
        let is_valid_diff = |&d: &isize| d != 0 && d.abs() <= 3;

        diffs
            .iter()
            .all(|&d| is_valid_diff(&d) && (d > 0) == is_increasing)
    }

    fn is_valid_one_removed(row: &[usize]) -> bool {
        (0..row.len()).any(|i| {
            let new_row = row
                .iter()
                .enumerate()
                .filter_map(|(j, &val)| if j != i { Some(val) } else { None })
                .collect::<Vec<_>>();
            Self::is_valid(&new_row)
        })
    }
}

impl Day for DayTwo {
    type Res = usize;
    fn part_one(&self) -> Self::Res {
        self.data.iter().filter(|row| Self::is_valid(row)).count()
    }
    fn part_two(&self) -> Self::Res {
        self.data
            .iter()
            .filter(|row| Self::is_valid(row) || Self::is_valid_one_removed(row))
            .count()
    }

    fn from_input() -> Result<Self, std::io::Error> {
        let data = get_input(2)?;

        let data: Vec<Vec<usize>> = data
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect()
            })
            .collect();

        Ok(DayTwo { data })
    }
}
fn main() -> Result<(), std::io::Error> {
    let day_two = DayTwo::from_input()?;
    aoc24::run(day_two)?;

    Ok(())
}
