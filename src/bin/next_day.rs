#![allow(clippy::items_after_test_module)] // you can ignore this

use aoc_23::*; // you can ignore this too

use std::error::Error;

const CURRENT_DAY: Day = Day::Day01; // update this with the current day

fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    // your solution to part 1 goes here.
    let _ = input;
    let todo = "".to_string();
    Ok(todo)
    // good luck! :)
}

fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    // your solution to part 2 goes here.
    let _ = input;
    let todo = "".to_string();
    Ok(todo)
    // good luck! :)
}

// Optional: use this to test your solutions.
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_part_1_solution() {
        const EXAMPLE_1: &str = r#"
            example input part 1 goes here
            note: you can use indented
            multiline strings just like this.
            or you can just simply copy-paste.
            do as you wish,
            the elves will handle everything!
        "#;

        const EXPECTED_ANSWER_1: &str = ""; // update this with the expected answer

        // that's it! run `cargo test --bin <day>` and let the elves do the rest

        elf_test_this!(EXAMPLE_1, solve_part_1, EXPECTED_ANSWER_1);
    }

    #[test]
    fn check_part_2_solution() {
        const EXAMPLE_2: &str = r#"
            example input part 2 goes here
            note: you can use indented
            multiline strings just like this.
            or you can just simply copy-paste.
            do as you wish,
            the elves will handle everything!
        "#;

        const EXPECTED_ANSWER_2: &str = ""; // update this with the expected answer

        // that's it! run `cargo test --bin <day>` and let the elves do the rest

        elf_test_this!(EXAMPLE_2, solve_part_2, EXPECTED_ANSWER_2);
    }
}

// simply run `cargo run --bin <day>` and check the answer inside `solution.yaml`
fn main() -> Result<(), Box<dyn Error>> {
    let mut advent = Advent::ho_ho_ho()?;

    advent.get_package(elf_magic!())?;

    Ok(())
}

// Puzzle Input
const PUZZLE_INPUT: &str = r#"
    puzzle input goes here
    note: you can use indented
    multiline strings just like this.
    or you can just simply copy-paste.
    do as you wish,
    the elves will handle everything!
"#;
