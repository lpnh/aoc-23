use aoc_23::*;

use anyhow::{Error, Result};

const CURRENT_DAY: Day = Day::Day01;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_01.txt");

fn solve_part_1(input: &str) -> Result<String, Error> {
    let mut sum = 0;

    for line in input.lines() {
        let first_digit = line.chars().find(|&c| c.is_ascii_digit()).unwrap();
        let last_digit = line.chars().rev().find(|&c| c.is_ascii_digit()).unwrap();

        let calibration_value = format!("{}{}", first_digit, last_digit).parse::<i32>()?;

        sum += calibration_value;
    }

    Ok(sum.to_string())
}

fn solve_part_2(input: &str) -> Result<String, Error> {
    let res = input
        .lines()
        .map(|line| {
            let first_digit = find_digit(line, Direction::Forward);
            let last_digit = find_digit(line, Direction::Backward);

            format!("{}{}", first_digit, last_digit)
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>()
        .to_string();

    Ok(res)
}

enum Direction {
    Forward,
    Backward,
}

impl Direction {
    fn iter<'a>(&self, input: &'a str) -> Box<dyn Iterator<Item = (usize, char)> + 'a> {
        match self {
            Direction::Forward => Box::new(input.char_indices()),
            Direction::Backward => Box::new(input.char_indices().rev()),
        }
    }
}

fn find_digit(input: &str, direction: Direction) -> char {
    let iter = direction.iter(input);

    for (index, c) in iter {
        if c.is_ascii_digit() {
            return c;
        }

        for &(word, digit) in &SPELLED_WORDS {
            if input[index..].starts_with(word) {
                return digit;
            }
        }
    }

    unreachable!()
}

const SPELLED_WORDS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn main() -> Result<(), Error> {
    Advent::ho_ho_ho(elf_magic!())?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_part_1() {
        const EXAMPLE_1: &str = "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ";

        const EXPECTED_ANSWER_1: &str = "142";

        test_part_one!();
    }

    #[test]
    fn check_part_2() {
        const EXAMPLE_2: &str = "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ";

        const EXPECTED_ANSWER_2: &str = "281";

        test_part_two!();
    }
}
