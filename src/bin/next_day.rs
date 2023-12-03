#![allow(clippy::items_after_test_module)]

use aoc_23::*;

use anyhow::{Error, Result};

const CURRENT_DAY: Day = Day::Day01;

fn solve_part_1(input: &str) -> Result<String, Error> {
    good_luck!(input)
}

fn solve_part_2(input: &str) -> Result<String, Error> {
    good_luck!(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_part_1_solution() {
        const EXAMPLE_1: &str = r#"
            example input part 1 goes here
        "#;

        const EXPECTED_ANSWER_1: &str = "";

        elf_test_this!(EXAMPLE_1, solve_part_1, EXPECTED_ANSWER_1);
    }

    #[test]
    fn check_part_2_solution() {
        const EXAMPLE_2: &str = r#"
            example input part 2 goes here
        "#;

        const EXPECTED_ANSWER_2: &str = "";

        elf_test_this!(EXAMPLE_2, solve_part_2, EXPECTED_ANSWER_2);
    }
}

fn main() -> Result<(), Error> {
    let mut advent = Advent::ho_ho_ho()?;

    advent.get_package(elf_magic!())?;

    Ok(())
}

const PUZZLE_INPUT: &str = r#"
    puzzle input goes here
"#;
