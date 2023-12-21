use aoc_23::*;

use std::collections::HashMap;
use anyhow::{Error, Result, anyhow};

const CURRENT_DAY: Day = Day::Day04;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_04.txt");

fn solve_part_1(input: &str) -> Result<String, Error> {
    let mut sum = 0;

    for line in input.lines() {
        let mut elf_winning_numbers = 0;

        let start = line.find(':').ok_or(anyhow!("':' not found"))? + 1;
        let end = line.find('|').ok_or(anyhow!("'|' not found"))?;

        let winning_numbers = &line[start..end];
        let winning_numbers_array = get_array(winning_numbers)?;        

        let elf_numbers = &line[(end + 1)..];
        let elf_numbers_array = get_array(elf_numbers)?;

        for number in elf_numbers_array {
            if winning_numbers_array.contains(&number) {
                elf_winning_numbers += 1;
            }
        }

        sum += match elf_winning_numbers {
            1 => 1,
            n if n > 1 => 2_i32.pow(n - 1),
            _ => 0,
        };
    }

    Ok(sum.to_string())
}

fn solve_part_2(input: &str) -> Result<String, Error> {
    let mut card_count = vec![1; input.lines().count()];
    let mut current_card = 1;

    for line in input.lines() {
        let mut elf_winning_numbers = 0;

        let start = line.find(':').ok_or(anyhow!("':' not found"))? + 1;
        let end = line.find('|').ok_or(anyhow!("'|' not found"))?;

        let winning_numbers = &line[start..end];
        let winning_numbers_array = get_array(winning_numbers)?;        

        let elf_numbers = &line[(end + 1)..];
        let elf_numbers_array = get_array(elf_numbers)?;

        for number in elf_numbers_array {
            if winning_numbers_array.contains(&number) {
                elf_winning_numbers += 1;
            }
        }

        let current_card_amount = card_count[current_card - 1];

        for _each_card in 0..current_card_amount {
            for n in current_card..=(current_card + elf_winning_numbers - 1) {
                card_count[n] += 1
            }
        }

        current_card += 1;

    }

    Ok(card_count.into_iter().sum::<i32>().to_string())
}

fn get_array(numbers: &str) -> Result<Vec<i32>, Error> {
    numbers.split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .map_err(|e| anyhow!("Error parsing numbers: {}", e))
}

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
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        const EXPECTED_ANSWER_1: &str = "13";

        test_part_one!();
    }

    #[test]
    fn check_part_2() {
        const EXAMPLE_2: &str = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        const EXPECTED_ANSWER_2: &str = "30";

        test_part_two!();
    }
}
