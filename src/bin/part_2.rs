use std::fs;
use std::io;

const SPELLED_WORDS: [(&str, char); 9] = [
    ("one", '1'), ("two", '2'), ("three", '3'),
    ("four", '4'), ("five", '5'), ("six", '6'),
    ("seven", '7'), ("eight", '8'), ("nine", '9'),
];

fn main() -> io::Result<()> {

    let input = include_str!("../input/input_2.txt");
    let output = solution(input);

    fs::write("solution/res_2.txt", output)?;

    Ok(())
}

fn solution(input: &str) -> String {
   let mut sum = 0;

    for line in input.lines() {
        let first_digit = find_first_digit(line);
        let last_digit = find_last_digit(line);

        let calibration_value = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
        
        sum += calibration_value;
    }

    sum.to_string()
}

fn find_first_digit(input: &str) -> char {
    let mut current_index = 0;

    while current_index < input.len() {
        let c = input.chars().nth(current_index).unwrap();

        if c.is_ascii_digit() {
            return c;
        }

        for (word, digit) in SPELLED_WORDS.iter() {
            if input[current_index..].starts_with(word) {
                return *digit;
            }
        }

        current_index += 1;
    }

    unreachable!()
}

fn find_last_digit(input: &str) -> char {
    let mut current_index = input.len() - 1;

    while current_index > 0 {
        let c = input.chars().nth(current_index).unwrap();

        if c.is_ascii_digit() {
            return c;
        }

        for (word, digit) in SPELLED_WORDS.iter() {
            if input[current_index..].starts_with(word) {
                return *digit;
            }
        }

        current_index -= 1;
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = include_str!("../input/test_2.txt");
        let output = solution(input);

        let expected_answer = "281";

        assert_eq!(output, expected_answer);
    }
}
