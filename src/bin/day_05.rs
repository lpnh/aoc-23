use aoc_23::*;

use anyhow::{Error, Result};

const CURRENT_DAY: Day = Day::Day05;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_05.txt");

fn solve_part_1(input: &str) -> Result<String, Error> {
    let almanac = Almanac::new(input);

    let locations = almanac
        .maps
        .iter()
        .fold(almanac.seeds, |new_seeds, each_map| {
            convert(each_map, &new_seeds)
        });

    let lowest_location = locations.into_iter().min().unwrap();

    Ok(lowest_location.to_string())
}

fn solve_part_2(input: &str) -> Result<String, Error> {
    good_luck!(input)
}

struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<(i64, i64, i64)>>,
}

impl Almanac {
    fn new(input: &str) -> Self {
        let seeds_section = input.lines().nth(0).unwrap();
        let seeds = parse_seeds(seeds_section);

        let maps = parse_to_vec(input);

        Self { seeds, maps }
    }
}

fn convert(each_map: &[(i64, i64, i64)], seeds: &[i64]) -> Vec<i64> {
    seeds
        .iter()
        .map(|&seed| {
            each_map
                .iter()
                .find_map(|&(dest_start, src_start, length)| {
                    check_map_line(dest_start, src_start, length, seed)
                })
                .unwrap_or(seed)
        })
        .collect()
}

fn check_map_line(dest_start: i64, src_start: i64, length: i64, seed: i64) -> Option<i64> {
    if seed >= src_start && seed < src_start + length {
        Some(dest_start + (seed - src_start))
    } else {
        None
    }
}

fn parse_to_vec(input: &str) -> Vec<Vec<(i64, i64, i64)>> {
    let mut sections = Vec::new();
    let mut current_section = Vec::new();

    for line in input.lines() {
        if line.contains(':') {
            if !current_section.is_empty() {
                sections.push(current_section);
                current_section = Vec::new();
            }
        } else {
            let nums: Vec<i64> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect();
            if nums.len() == 3 {
                current_section.push((nums[0], nums[1], nums[2]));
            }
        }
    }

    if !current_section.is_empty() {
        sections.push(current_section);
    }

    sections
}

fn parse_seeds(input: &str) -> Vec<i64> {
    input
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect()
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
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        ";

        const EXPECTED_ANSWER_1: &str = "35";

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
