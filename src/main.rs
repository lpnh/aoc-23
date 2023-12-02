use clap::{arg, Command};
use std::error::Error;

use aoc_23::*;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Advent of Code")
        .version("1.0")
        .author("lpnh")
        .about("Runs solutions for Advent of Code")
        .arg(arg!([day] "Specifies the day to run"))
        .get_matches();

    match matches.get_one::<String>("day") {
        Some(day) => {
            match day.as_str() {
                "day01" => day_01::x_mas()?,
                // "day02" => day_02::run()?,
                // ... other days
                _ => return Err("Invalid day".into()),
            }
        }
        None => return Err("Please specify a day (e.g., day01)".into()),
    }

    Ok(())
}