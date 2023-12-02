use std::io;

use crate::advent::*;

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

pub fn update(advent: &mut Advent) -> Result<(), io::Error> {
    let input = include_str!("./io/puzzle.txt");
    let output = solution(input);

    let _ = advent.solve(Day::Day01, Part::Part2, Some(output));

    Ok(())
}

fn solution(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let first_digit = find_digit(line, Direction::Forward);
            let last_digit = find_digit(line, Direction::Backward);

            format!("{}{}", first_digit, last_digit)
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>()
        .to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = include_str!("./io/test_2.txt");
        let output = solution(input);

        let expected_answer = "281";

        assert_eq!(output, expected_answer);
    }
}
