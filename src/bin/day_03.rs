use aoc_23::*;

use anyhow::{Error, Result};
use std::collections::HashSet;

const CURRENT_DAY: Day = Day::Day03;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_03.txt");

fn solve_part_1(input: &str) -> Result<String, Error> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    let mut visited = HashSet::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() && !visited.contains(&(row_idx, col_idx)) {
                let mut number = 0;
                let mut positions = HashSet::new(); // useless. but required for shared dfs function
                let mut is_adjacent_to_symbol = false;
                dfs(
                    &grid,
                    (row_idx, col_idx),
                    &mut visited,
                    &mut number,
                    &mut positions, // idem
                    &mut is_adjacent_to_symbol,
                );
                if is_adjacent_to_symbol {
                    sum += number;
                }
            }
        }
    }

    Ok(sum.to_string())
}

fn dfs(grid: &[Vec<char>], vertex: (usize, usize), visited: &mut HashSet<(usize, usize)>, number: &mut i32, positions: &mut HashSet<(usize, usize)>, include_number: &mut bool) {
    if visited.insert(vertex) {
        if let Some(digit) = grid[vertex.0][vertex.1].to_digit(10) {
            *number = *number * 10 + digit as i32;
            positions.insert(vertex);
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    let new_x = vertex.0 as i32 + dx;
                    let new_y = vertex.1 as i32 + dy;
                    if new_x >= 0 && (new_x as usize) < grid.len() && new_y >= 0 && (new_y as usize) < grid[0].len() {
                        let neighbor = (new_x as usize, new_y as usize);
                        let neighbor_cell = grid[neighbor.0][neighbor.1];
                        if neighbor_cell.is_ascii_digit() {
                            dfs(grid, neighbor, visited, number, positions, include_number);
                        } else if neighbor_cell != '.' {
                            *include_number = true;
                        }
                    }
                }
            }
        }
    }
}

fn is_adjacent(positions: &HashSet<(usize, usize)>, x: usize, y: usize) -> bool {
    positions.contains(&(x, y))
        || positions.contains(&(x.saturating_sub(1), y))
        || positions.contains(&(x, y.saturating_sub(1)))
        || positions.contains(&(x.saturating_sub(1), y.saturating_sub(1)))
        || positions.contains(&(x + 1, y))
        || positions.contains(&(x, y + 1))
        || positions.contains(&(x + 1, y + 1))
        || positions.contains(&(x + 1, y.saturating_sub(1)))
        || positions.contains(&(x.saturating_sub(1), y + 1))
}

fn solve_part_2(input: &str) -> Result<String, Error> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut part_numbers: Vec<(i32, HashSet<(usize, usize)>)> = Vec::new();
    let mut visited = HashSet::new();

    for (x, row) in grid.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() && !visited.contains(&(x, y)) {
                let mut number = 0;
                let mut positions = HashSet::new();
                let mut include_number = false;
                dfs(&grid, (x, y), &mut visited, &mut number, &mut positions, &mut include_number);
                if include_number {
                    part_numbers.push((number, positions));
                }
            }
        }
    }

    let mut total_gear_ratio = 0;
    for (x, row) in grid.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if cell == '*' {
                let adj_part_numbers: Vec<i32> = part_numbers.iter()
                    .filter(|&(_, positions)| is_adjacent(positions, x, y))
                    .map(|&(num, _)| num)
                    .collect();
                
                if adj_part_numbers.len() == 2 {
                    total_gear_ratio += adj_part_numbers[0] * adj_part_numbers[1];
                }
            }
        }
    }

    Ok(total_gear_ratio.to_string())
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

        const EXPECTED_ANSWER_2: &str = "467835";

        test_part_two!();
    }
}
