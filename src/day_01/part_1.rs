use std::{error::Error, io};

use crate::advent::*;

pub fn update(advent: &mut Advent) -> Result<(), Box<dyn Error>> {
    let input = include_str!("./io/puzzle.txt");
    let output = solution(input)?;

    let _ = advent.solve(Day::Day01, Part::Part1, Some(output));

    Ok(())
}

fn solution(input: &str) -> Result<String, io::Error> {
    let mut sum = 0;

    for line in input.lines() {
        let first_digit = line.chars().find(|&c| c.is_ascii_digit()).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "No digit found in the line")
        })?;
        let last_digit = line
            .chars()
            .rev()
            .find(|&c| c.is_ascii_digit())
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "No digit found in the line")
            })?;

        let calibration_value = format!("{}{}", first_digit, last_digit)
            .parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        sum += calibration_value;
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = include_str!("./io/test_1.txt");
        let output = solution(input).unwrap();

        let expected_answer = "142";

        assert_eq!(output, expected_answer);
    }
}
