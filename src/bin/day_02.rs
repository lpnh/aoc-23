use aoc_23::*;

use anyhow::{Error, Result};

const CURRENT_DAY: Day = Day::Day02;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_02.txt");

fn solve_part_1(input: &str) -> Result<String, Error> {
    let mut games_sum = 0;
    let mut game_id = 1;

    for game in input.lines() {
        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;

        let colon_position = game.find(':').unwrap();
        let rounds = game[(colon_position + 1)..].split(';').collect::<Vec<_>>();

        for round in rounds {
            let cubes = round.split(',').collect::<Vec<_>>();

            for cube in cubes {
                let cube = cube.trim();

                if cube.ends_with("red") {
                    let current_red = cube.split(' ').collect::<Vec<_>>()[0].parse::<u32>()?;
                    if current_red > red_cubes {
                        red_cubes = current_red;
                    }
                } else if cube.ends_with("green") {
                    let current_green = cube.split(' ').collect::<Vec<_>>()[0].parse::<u32>()?;
                    if current_green > green_cubes {
                        green_cubes = current_green;
                    }
                } else if cube.ends_with("blue") {
                    let current_blue = cube.split(' ').collect::<Vec<_>>()[0].parse::<u32>()?;
                    if current_blue > blue_cubes {
                        blue_cubes = current_blue;
                    }
                }

            }
        }

        if red_cubes <= 12 && green_cubes <= 13 && blue_cubes <= 14 {
            games_sum += game_id;
        } 

        game_id += 1;
    }
    
    Ok(games_sum.to_string())
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
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        const EXPECTED_ANSWER_1: &str = "8";

        test_part_one!();
    }

    #[test]
    fn check_part_2() {
        const EXAMPLE_2: &str = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        const EXPECTED_ANSWER_2: &str = "2286";

        test_part_two!();
    }
}
