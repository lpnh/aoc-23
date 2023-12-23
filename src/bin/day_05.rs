use aoc_23::*;

use anyhow::{anyhow, Error, Result};

use std::ops::Range;

const CURRENT_DAY: Day = Day::Day05;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_05.txt");

fn solve_part_1(input: &str) -> Result<String, Error> {
    let seeds_section = input.lines().next().ok_or(anyhow!("Invalid input: Missing seeds section"))?;
    let seeds = parse_seeds(seeds_section, Part::One);
    let mappings = Mappings::build_mappings(&seeds, &parse_mappings(input)?.iter().map(|m| m.as_slice()).collect::<Vec<_>>());

    seeds.iter()
        .filter_map(|&seed| mappings.find_lowest_location(seed))
        .min()
        .map(|min_location| min_location.to_string())
        .ok_or_else(|| anyhow!("No minimum location found"))
}

pub fn solve_part_2(input: &str) -> Result<String, Error> {
    let seeds_section = input.lines().next().ok_or(anyhow!("Invalid input: Missing seeds section"))?;
    let seeds = parse_seeds(seeds_section, Part::Two);
    let mappings = Mappings::build_mappings(&seeds, &parse_mappings(input)?.iter().map(|m| m.as_slice()).collect::<Vec<_>>());

    seeds.iter()
        .filter_map(|&seed| mappings.find_lowest_location(seed))
        .min()
        .map(|min_location| min_location.to_string())
        .ok_or_else(|| anyhow!("No minimum location found"))
}

fn parse_seeds(input: &str, part: Part) -> Vec<usize> {
    match part {
        Part::One => parse_numbers(input),
        Part::Two => parse_numbers(input)
            .chunks_exact(2)
            .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .collect(),
    }
}

fn parse_mappings(input: &str) -> Result<Vec<Vec<(usize, usize, usize)>>> {
    let maps = input
        .split(':')
        .filter_map(parse_tuples)
        .collect::<Vec<_>>();

    if maps.len() == 7 { Ok(maps) } else { Err(anyhow!("Incorrect number of maps provided")) }
}

fn parse_tuples(section: &str) -> Option<Vec<(usize, usize, usize)>> {
    let tuples: Vec<_> = section
        .lines()
        .filter_map(|line| {
            let nums: Vec<usize> = line.split_whitespace().filter_map(|num| num.parse().ok()).collect();
            if nums.len() == 3 {
                Some((nums[0], nums[1], nums[2]))
            } else {
                None
            }
        })
        .collect();
    if tuples.is_empty() { None } else { Some(tuples) }
}

fn parse_numbers(input: &str) -> Vec<usize> {
    input.split_whitespace().filter_map(|num| num.parse().ok()).collect()
}

enum Part {
    One,
    Two,
}

struct Mappings {
    segment_trees: Vec<SegmentTree<usize>>,
}

impl Mappings {
    pub fn build_mappings(seeds: &[usize], maps: &[&[(usize, usize, usize)]]) -> Self {
        assert_eq!(maps.len(), 7, "There must be exactly 7 maps");
        
        let segment_trees = maps.iter().map(|&map| {
            SegmentTree::from_map(seeds, map)
        }).collect::<Vec<_>>();

        Mappings { segment_trees }
    }

    pub fn find_lowest_location(&self, seed: usize) -> Option<usize> {
        let mut current_value = seed;

        for tree in &self.segment_trees {
            current_value = tree.query(current_value..current_value + 1).unwrap_or(current_value);
        }

        Some(current_value)
    }
}

pub struct SegmentTree<T: Default + Copy> {
    len: usize,
    tree: Vec<T>,
    merge: fn(T, T) -> T,
}

impl<T: Default + Copy> SegmentTree<T> {
    pub fn from_vec(arr: &[T], merge: fn(T, T) -> T) -> Self {
        let len = arr.len();
        let mut tree = SegmentTree {
            len,
            tree: vec![T::default(); 4 * len],
            merge,
        };
        if !arr.is_empty() {
            tree.build_recursive(arr, 1, 0..len);
        }
        tree
    }

    fn build_recursive(&mut self, arr: &[T], idx: usize, range: Range<usize>) {
        if range.end - range.start == 1 {
            self.tree[idx] = arr[range.start];
        } else {
            let mid = range.start + (range.end - range.start) / 2;
            self.build_recursive(arr, 2 * idx, range.start..mid);
            self.build_recursive(arr, 2 * idx + 1, mid..range.end);
            self.tree[idx] = (self.merge)(self.tree[2 * idx], self.tree[2 * idx + 1]);
        }
    }

    pub fn query(&self, range: Range<usize>) -> Option<T> {
        self.query_recursive(1, 0..self.len, &range)
    }

    fn query_recursive(&self, idx: usize, element_range: Range<usize>, query_range: &Range<usize>) -> Option<T> {
        if element_range.start >= query_range.end || element_range.end <= query_range.start {
            return None;
        }
        if element_range.start >= query_range.start && element_range.end <= query_range.end {
            return Some(self.tree[idx]);
        }
        let mid = element_range.start + (element_range.end - element_range.start) / 2;
        let left = self.query_recursive(2 * idx, element_range.start..mid, query_range);
        let right = self.query_recursive(2 * idx + 1, mid..element_range.end, query_range);
        match (left, right) {
            (None, None) => None,
            (Some(l), None) | (None, Some(l)) => Some(l),
            (Some(l), Some(r)) => Some((self.merge)(l, r)),
        }
    }

    pub fn update(&mut self, idx: usize, val: T) {
        if idx < self.len {
            self.update_recursive(1, 0..self.len, idx, val);
        }
    }

    fn update_recursive(&mut self, idx: usize, element_range: Range<usize>, target_idx: usize, val: T) {
        if element_range.start > target_idx || element_range.end <= target_idx {
            return;
        }
        if element_range.end - element_range.start == 1 && element_range.start == target_idx {
            self.tree[idx] = val;
            return;
        }
        let mid = element_range.start + (element_range.end - element_range.start) / 2;
        self.update_recursive(2 * idx, element_range.start..mid, target_idx, val);
        self.update_recursive(2 * idx + 1, mid..element_range.end, target_idx, val);
        self.tree[idx] = (self.merge)(self.tree[2 * idx], self.tree[2 * idx + 1]);
    }
}

impl SegmentTree<usize> {
    pub fn from_map(seeds: &[usize], map: &[(usize, usize, usize)]) -> Self {
        let merge_fn = std::cmp::min;
        let mut tree = SegmentTree::from_vec(seeds, merge_fn);
        for &(src_start, dest_start, length) in map {
            for i in 0..length {
                tree.update(src_start + i, dest_start + i);
            }
        }
        tree
    }
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
