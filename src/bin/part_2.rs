use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = include_str!("../input/input_2.txt");
    let output = solution(input);

    fs::write("solution/res_2.txt", output)?;

    Ok(())
}

fn solution(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = include_str!("../input/test_2.txt");
        let output = solution(input);

        let expected_answer = "";

        assert_eq!(output, expected_answer);
    }
}
