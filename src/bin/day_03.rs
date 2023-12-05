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

    let mut sum = 0;
    let mut visited = HashSet::new();

    for (x, row) in grid.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() && !visited.contains(&(x, y)) {
                let mut number = 0;
                let mut include_number = false;
                dfs(&grid, x, y, &mut number, &mut include_number, &mut visited);
                if include_number {
                    sum += number;
                }
            }
        }
    }

    Ok(sum.to_string())
}

fn dfs(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    number: &mut i32,
    include_number: &mut bool,
    visited: &mut HashSet<(usize, usize)>,
) {
    if visited.insert((x, y)) {
        if let Some(digit) = grid[x][y].to_digit(10) {
            *number = *number * 10 + digit as i32;
            let directions = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];

            for (dx, dy) in directions.iter() {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                if new_x >= 0
                    && new_x < grid.len() as i32
                    && new_y >= 0
                    && new_y < grid[0].len() as i32
                {
                    let (new_x, new_y) = (new_x as usize, new_y as usize);
                    let neighbor_cell = grid[new_x][new_y];
                    if neighbor_cell.is_ascii_digit() {
                        dfs(grid, new_x, new_y, number, include_number, visited);
                    } else if neighbor_cell != '.' {
                        *include_number = true;
                    }
                }
            }
        }
    }
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
