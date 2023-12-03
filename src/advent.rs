use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    default::Default,
    fmt::{self, Display, Formatter},
};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
struct Solution {
    part_1: Option<String>,
    part_2: Option<String>,
}

/// The Days of Advent
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Day {
    /// 24 days until Christmas 🎄
    Day01,
    /// 23 days until Christmas 🎄
    Day02,
    /// 22 days until Christmas 🎄
    Day03,
    /// 21 days until Christmas 🎄
    Day04,
    /// 20 days until Christmas 🎄
    Day05,
    /// 19 days until Christmas 🎄
    Day06,
    /// 18 days until Christmas 🎄
    Day07,
    /// 17 days until Christmas 🎄
    Day08,
    /// 16 days until Christmas 🎄
    Day09,
    /// 15 days until Christmas 🎄
    Day10,
    /// 14 days until Christmas 🎄
    Day11,
    /// 13 days until Christmas 🎄
    Day12,
    /// 12 days until Christmas 🎄
    Day13,
    /// 11 days until Christmas 🎄
    Day14,
    /// 10 days until Christmas 🎄
    Day15,
    /// 9 days until Christmas 🎄
    Day16,
    /// 8 days until Christmas 🎄
    Day17,
    /// 7 days until Christmas 🎄
    Day18,
    /// 6 days until Christmas 🎄
    Day19,
    /// 5 days until Christmas 🎄
    Day20,
    /// 4 days until Christmas 🎄
    Day21,
    /// 3 days until Christmas 🎄
    Day22,
    /// 2 days until Christmas 🎄
    Day23,
    /// 1 day until Christmas 🎄
    Day24,
    /// 🌟 Merry Christmas! 🌟
    Day25,
}

/// Your Advent Calendar 📆
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Advent {
    advent: HashMap<Day, Solution>,
}

/// Your SantaPackage 🎁
pub struct SantaPackage {
    pub day: Day,
    pub puzzle_input: String,
    pub solution_part_1: fn(&str) -> Result<String, Error>,
    pub solution_part_2: fn(&str) -> Result<String, Error>,
}

impl Advent {
    const SOLUTION_PATH: &'static str = "solution.yaml";

    /// Happy Holidays!
    pub fn ho_ho_ho() -> Result<Self, Error> {
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

    /// Your SantaPackage is here!
    pub fn get_package(&mut self, package: SantaPackage) -> Result<(), Error> {
        let entry = self.advent.entry(package.day).or_default();

        let solution_part_1 = (package.solution_part_1)(&package.puzzle_input)?;
        let solution_part_2 = (package.solution_part_2)(&package.puzzle_input)?;
        entry.part_1 = Some(solution_part_1);
        entry.part_2 = Some(solution_part_2);

        self.write_solution()?;

        Ok(())
    }

    fn write_solution(&self) -> Result<(), Error> {
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
