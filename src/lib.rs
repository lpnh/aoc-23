mod advent;
pub use advent::{Advent, Day, SantaPackage};

// custom function to format multiline strings
pub fn _fmt(raw_str: &str) -> String {
    let lines: Vec<&str> = raw_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let min_indent = lines
        .iter()
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);

    lines
        .into_iter()
        .map(|line| line.get(min_indent..).unwrap_or(line))
        .collect::<Vec<&str>>()
        .join("\n")
}

// macro magic to test solutions
#[macro_export]
macro_rules! elf_test_this {
    ($input:expr, $solver:expr, $expected:expr) => {
        let input_example = _fmt($input);
        let solution = $solver(&input_example).unwrap();
        assert_eq!(solution, $expected);
    };
}

// macro magic to generate the SantaPackage struct
#[macro_export]
macro_rules! elf_magic {
    () => {
        SantaPackage {
            day: CURRENT_DAY,
            puzzle_input: _fmt(PUZZLE_INPUT),
            solution_part_1: solve_part_1,
            solution_part_2: solve_part_2,
        }
    };
}
