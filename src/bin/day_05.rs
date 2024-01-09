use aoc_23::*;

use anyhow::{Error, Result};

const CURRENT_DAY: Day = Day::Day05;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_05.txt");

use std::collections::HashSet;

fn solve_part_1(input: &str) -> Result<String, Error> {
    let maps = get_maps(input);
    let mut seeds = get_seed_ranges(input, Part::One);
    
    for each_map in maps {
        seeds = mapping_seeds(&each_map, &seeds)
    }

    let min_min = seeds.iter()
                    .min_by_key(|seed| seed.min)
                    .map(|seed| seed.min)
                    .unwrap();

    Ok(min_min.to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Error> {  
    let maps = get_maps(input);
    let mut seeds = get_seed_ranges(input, Part::Two);
    
    for each_map in maps {
        seeds = mapping_seeds(&each_map, &seeds)
    }

    let min_min = seeds.iter()
                    .min_by_key(|seed| seed.min)
                    .map(|seed| seed.min)
                    .unwrap();

    Ok(min_min.to_string())
}

enum Part {
    One,
    Two,
}

pub fn mapping_seeds(map: &[MapLine], seeds: &[SeedRange]) -> Vec<SeedRange> {
    let mut new_seeds = HashSet::new();

    for seed_range in seeds {
        let mut found = false;

        for map_line in map {
            let result = SeedAfterEachLine::new(map_line, seed_range);
            
            if let Some(val) = result.transformed_seed {
                new_seeds.insert(val);
            }
            
            if let Some(val) = result.not_transformed_seed {
                new_seeds.insert(val);
            }

            if result.found {
                found = true;
            }
        }

        if !found {
            new_seeds.insert(*seed_range);
        }
    }

    new_seeds.into_iter().collect()
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct SeedRange {
    min: usize,
    max: usize,
}

impl SeedRange {
    fn from_tuple(min: usize, max: usize) -> Self {
        Self {
            min,
            max
        }
    }

    fn from_tuple_into_option(min: usize, max: usize) -> Option<Self> {
        Some(
            Self {
                min,
                max
            }
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MapLine {
    dest_start: usize,
    source_start: usize,
    source_end: usize,
    range_length: usize
}

impl MapLine {
    fn from_tuple(dest_start: usize, source_start: usize, range_length: usize) -> Self {
        let source_end = source_start + range_length;

        Self {
            dest_start,
            source_start,
            source_end,
            range_length,
        }
    }
}

pub struct SeedAfterEachLine {
    found: bool,
    transformed_seed: Option<SeedRange>,
    not_transformed_seed: Option<SeedRange>,
}

#[derive(Debug)]
enum SeedMapState {
    OutsideMappingRange,
    EntirelyInsideMappingRange,
    PartiallyInsideMappingStart,
    PartiallyInsideMappingEnd,
}

impl SeedAfterEachLine {
    pub fn new(map_line: &MapLine, seed_range: &SeedRange) -> Self {
        let state = Self::check_state(map_line, seed_range);

        match state {
            SeedMapState::OutsideMappingRange => 
                Self {
                    found: false,
                    transformed_seed: None,
                    not_transformed_seed: None,
                }
            ,
            SeedMapState::EntirelyInsideMappingRange => 
                Self {
                    found: true,
                    transformed_seed: SeedRange::from_tuple_into_option(
                        seed_range.min + map_line.dest_start - map_line.source_start,
                        seed_range.max + map_line.dest_start - map_line.source_start
                    ),
                    not_transformed_seed: None,
                }
            ,
            SeedMapState::PartiallyInsideMappingStart => 
                Self {
                    found: true,
                    transformed_seed: SeedRange::from_tuple_into_option(
                        map_line.dest_start,
                        seed_range.max + map_line.dest_start - map_line.source_start
                    ),
                    not_transformed_seed: SeedRange::from_tuple_into_option(seed_range.min, map_line.source_start - 1),
                }
            ,
            SeedMapState::PartiallyInsideMappingEnd => 
                Self {
                    found: true,
                    transformed_seed: SeedRange::from_tuple_into_option(
                        seed_range.min + map_line.dest_start - map_line.source_start,
                        map_line.dest_start + map_line.range_length -1
                    ),
                    not_transformed_seed: SeedRange::from_tuple_into_option(map_line.source_end, seed_range.max),
                }
        }
    }

    fn check_state(map_line: &MapLine, seed_range: &SeedRange) -> SeedMapState {
        if seed_range.min >= map_line.source_start && seed_range.max < map_line.source_end {
            SeedMapState::EntirelyInsideMappingRange
        } else if seed_range.max > map_line.source_start && seed_range.max < map_line.source_end {
            SeedMapState::PartiallyInsideMappingStart
        } else if seed_range.min >= map_line.source_start && seed_range.min < map_line.source_end {
            SeedMapState::PartiallyInsideMappingEnd
        } else {
            SeedMapState::OutsideMappingRange
        }
    }
}

// get_maps parses the input into a vector of maps, each map being a vector of tuples.
// Each tuple represents a single mapping line with destination start, source start, and length.
fn get_maps(input: &str) -> Vec<Vec<MapLine>> {
    input
        .split(':')
        .filter_map(|section| {
            let mut tuples = Vec::new();
            for line in section.lines() {
                let nums: Vec<usize> = line.split_whitespace()
                                            .filter_map(|num| num.parse().ok())
                                            .collect();
                if nums.len() == 3 {
                    tuples.push(MapLine::from_tuple(nums[0], nums[1], nums[2]));
                }
            }
            if tuples.is_empty() { None } else { Some(tuples) }
        })
        .collect()
}

fn get_seed_ranges(input: &str, part: Part) -> Vec<SeedRange> {   
    let values_from_str: Vec<usize> = input
        .lines()
        .next()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|word| word.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .unwrap();

    let ranges = match part {
        Part::One => values_from_str
            .into_iter()
            .map(|single_val| SeedRange::from_tuple(single_val, single_val))
            .collect::<Vec<SeedRange>>(),

        Part::Two => values_from_str
            .chunks_exact(2)
            .map(|chunk| SeedRange::from_tuple(chunk[0], chunk[0] + chunk[1]))
            .collect::<Vec<SeedRange>>()
    };

    ranges
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

        const EXPECTED_ANSWER_2: &str = "46";

        test_part_two!();
    }
}
