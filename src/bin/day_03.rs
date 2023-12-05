use aoc_23::*;

use anyhow::{Error, Result};
use std::collections::HashSet;

const CURRENT_DAY: Day = Day::Day03;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_03.txt");

fn solve_part_1(input: &str) -> Result<String, Error> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let graph = Graph::new(grid);
    let mut sum = 0;
    let mut visited = HashSet::new();

    for (x, row) in graph.grid.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() && !visited.contains(&Vertex(x, y)) {
                let mut number = 0;
                let mut include_number = false;
                graph.dfs(Vertex(x, y), &mut visited, &mut number, &mut include_number);
                if include_number {
                    sum += number;
                }
            }
        }
    }

    Ok(sum.to_string())
}

fn solve_part_2(input: &str) -> Result<String, Error> {
    good_luck!(input)
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
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";

        const EXPECTED_ANSWER_1: &str = "4361";

        test_part_one!();
    }

    #[test]
    fn check_part_2() {
        const EXAMPLE_2: &str = "
            example input part 2 goes here
        ";

        const EXPECTED_ANSWER_2: &str = "";

        test_part_two!();
    }
}
