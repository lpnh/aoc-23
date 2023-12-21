use aoc_23::*;

use std::collections::HashSet;
use anyhow::{Error, Result, anyhow};

const CURRENT_DAY: Day = Day::Day04;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_04.txt");

fn solve_part_1(input: &str) -> Result<String, Error> {
    let total: u32 = input.lines()
        .map(|line| {
            let (card_winning_numbers, elf_numbers) = parse_line(line).unwrap();
            let elf_winning_numbers = elf_numbers.intersection(&card_winning_numbers).count() as u32;
            match elf_winning_numbers {
                1 => 1,
                n if n > 1 => 2_u32.pow(n - 1),
                _ => 0,
            }
        })
        .sum();

    Ok(total.to_string())
}

fn solve_part_2(input: &str) -> Result<String, Error> {
    let mut scratchcards_pile = vec![1; input.lines().count()];

    for (index, line) in input.lines().enumerate() {
        let (card_winning_numbers, elf_numbers) = parse_line(line)?;
        let elf_winning_numbers = elf_numbers.intersection(&card_winning_numbers).count();

        let current_card_amount = scratchcards_pile[index];

        scratchcards_pile.iter_mut()
            .skip(index + 1)
            .take(elf_winning_numbers)
            .for_each(|next_card| *next_card += current_card_amount);
    }

    let scratchcards_sum = scratchcards_pile.into_iter().sum::<i32>();

    Ok(scratchcards_sum.to_string())
}

fn parse_line(line: &str) -> Result<(HashSet<i32>, HashSet<i32>), Error> {
    let (winning_part, elf_part) = line.split_once('|').ok_or(anyhow!("'|' not found"))?;
    let card_winning_numbers = winning_part.split_once(':').ok_or(anyhow!("':' not found"))?.1;
    let card_winning_set = get_set(card_winning_numbers)?;
    let elf_set = get_set(elf_part)?;

    Ok((card_winning_set, elf_set))
}

fn get_set(numbers: &str) -> Result<HashSet<i32>, Error> {
    numbers.split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<HashSet<i32>, _>>()
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
