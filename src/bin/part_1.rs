use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = include_str!("../input/input_1.txt");
    let output = solution(input);

    fs::write("solution/res_1.txt", output)?;

    Ok(())
}

fn solution(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let first_digit = line.chars().find(|&c| c.is_ascii_digit()).unwrap();
        let last_digit = line.chars().rev().find(|&c| c.is_ascii_digit()).unwrap();

        let calibration_value = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
        
        sum += calibration_value;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = include_str!("../input/test_1.txt");
        let output = solution(input);

        let expected_answer = "142";

        assert_eq!(output, expected_answer);
    }
}
