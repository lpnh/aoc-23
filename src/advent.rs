use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    default::Default,
    fmt::{self, Display, Formatter},
};
use std::{error::Error, fs, path::Path};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
struct Solution {
    part_1: Option<String>,
    part_2: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Advent {
    advent: HashMap<Day, Solution>,
}

pub struct SantaPackage {
    pub day: Day,
    pub puzzle_input: String,
    pub solution_part_1: fn(&str) -> Result<String, Box<dyn Error>>,
    pub solution_part_2: fn(&str) -> Result<String, Box<dyn Error>>,
}

impl Advent {
    const SOLUTION_PATH: &'static str = "solution.yaml";

    pub fn ho_ho_ho() -> Result<Self, Box<dyn Error>> {
        if Path::new(Self::SOLUTION_PATH).exists() {
            let contents = fs::read_to_string(Self::SOLUTION_PATH)?;
            let advent = serde_yaml::from_str(&contents)?;
            Ok(advent)
        } else {
            Ok(Self {
                advent: HashMap::new(),
            })
        }
    }

    pub fn get_package(&mut self, package: SantaPackage) -> Result<(), Box<dyn Error>> {
        let entry = self.advent.entry(package.day).or_default();

        let solution_part_1 = (package.solution_part_1)(&package.puzzle_input)?;
        let solution_part_2 = (package.solution_part_2)(&package.puzzle_input)?;
        entry.part_1 = Some(solution_part_1);
        entry.part_2 = Some(solution_part_2);

        self.write_solution()?;

        Ok(())
    }

    fn write_solution(&self) -> Result<(), Box<dyn Error>> {
        let advent_yaml = serde_yaml::to_string(&self)?;
        fs::write(Self::SOLUTION_PATH, advent_yaml)?;
        Ok(())
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Solution:\nPart_1: {:?}\nPart_2: {:?}",
            self.part_1, self.part_2
        )
    }
}

impl Display for Advent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (day, solution) in &self.advent {
            writeln!(f, "{}\n{}", day, solution)?;
        }

        Ok(())
    }
}
