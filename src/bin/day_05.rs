use aoc_23::*;

use anyhow::{Error, Result};

const CURRENT_DAY: Day = Day::Day00;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_00.txt");

fn solve_part_1(input: &str) -> Result<String, Error> {
    let (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location
    ) = map_seeds_to_location(input);
    
    let almanac_maps = [
        &seed_to_soil,
        &soil_to_fertilizer,
        &fertilizer_to_water,
        &water_to_light,
        &light_to_temperature,
        &temperature_to_humidity,
        &humidity_to_location
    ];

    let mut locations = vec![0; seeds.len()];


    for each_map in almanac_maps {
        locations = convert_map(each_map, &seeds)
    }

    let lowest_location = locations.into_iter().min().unwrap();

    Ok(lowest_location.to_string())
}

fn solve_part_2(input: &str) -> Result<String, Error> {
    good_luck!(input)
}

fn convert_map(map: &Vec<(i32, i32, i32)>, seeds: &Vec<i32>) -> Vec<i32> {
    let mut new_map: Vec<i32> = vec![];

    for seed in seeds {
        let mut index = 0;

        for line in map {
            let (new_seed, found) = check_map_line(line, seed);
            new_map[index] = new_seed;

            if found { break }

            index += 1;
        }
    }

    new_map
}

fn check_map_line(map_line: &(i32, i32, i32), seed: &i32) -> (i32, bool) {
    let index = 0;

    for i in (map_line.1)..(map_line.1 + map_line.2) {
        if i == *seed {
            return (map_line.0 + index, true);
        }
    }

    (*seed, false)
}

fn parse_to_vec(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line.split_whitespace()
                                     .filter_map(|num| num.parse::<i32>().ok())
                                     .collect();
            (nums[0], nums[1], nums[2])
        })
        .collect()
}

fn parse_seeds(input: &str) -> Vec<i32> {
    input.split_whitespace()
         .filter_map(|num| num.parse::<i32>().ok())
         .collect()
}

fn map_seeds_to_location(input: &str) -> (
    Vec<i32>, Vec<(i32, i32, i32)>, Vec<(i32, i32, i32)>, Vec<(i32, i32, i32)>,
    Vec<(i32, i32, i32)>, Vec<(i32, i32, i32)>, Vec<(i32, i32, i32)>, Vec<(i32, i32, i32)>
) {
    let mut sections = input.split("\n\n");
    let seeds_section = sections.next().unwrap_or("");
    let seed_vec = parse_seeds(seeds_section.lines().nth(1).unwrap_or_default());
    
    let maps: Vec<Vec<(i32, i32, i32)>> = sections.map(parse_to_vec).collect();
    (
        seed_vec,
        maps.get(0).cloned().unwrap_or_default(),
        maps.get(1).cloned().unwrap_or_default(),
        maps.get(2).cloned().unwrap_or_default(),
        maps.get(3).cloned().unwrap_or_default(),
        maps.get(4).cloned().unwrap_or_default(),
        maps.get(5).cloned().unwrap_or_default(),
        maps.get(6).cloned().unwrap_or_default(),
    )
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
