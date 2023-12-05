#![feature(iter_array_chunks)]

use std::fs;

#[derive(Debug, Clone)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, len: u32) -> Range {
        Range {
            start: start,
            end: start + (len - 1),
        }
    }
    fn contains(&self, val: u32) -> bool {
        val >= self.start && val <= self.end
    }

    fn map(&self, other: &Range, val: u32) -> u32 {
        let delta = val - self.start;
        other.start + delta
    }
}

#[derive(Debug, Clone)]
struct Mapping {
    // (src range, dest range)
    mappings: Vec<(Range, Range)>,
}

impl Mapping {
    fn new() -> Mapping {
        Mapping { mappings: vec![] }
    }

    fn append(&mut self, raw: &str) {
        let values = raw
            .split(" ")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let cnt = values[2];
        self.mappings
            .push((Range::new(values[1], cnt), Range::new(values[0], cnt)));
    }

    fn get(&self, input: u32) -> u32 {
        if let Some(mapping) = self
            .mappings
            .iter()
            .find(|mapping| mapping.0.contains(input))
        {
            mapping.0.map(&mapping.1, input)
        } else {
            input
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    // Part 1
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|f| f.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let maps = input
        .split("\n\n")
        .skip(1)
        .map(|x| {
            let ranges = x
                .lines()
                .filter(|line| !line.contains(":"))
                .collect::<Vec<&str>>();
            let mut result = Mapping::new();
            for range in ranges {
                result.append(range);
            }
            result
        })
        .collect::<Vec<Mapping>>();

    let part1 = seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |a, f| f.get(a)))
        .min()
        .unwrap();

    // Part 2
    let seed_ranges = seeds
        .iter()
        .array_chunks::<2>()
        .map(|c| Range::new(*c[0], *c[1]))
        .collect::<Vec<Range>>();

    let part2 = seed_ranges
        .iter()
        .map(|seed_range| {
            let mut result = u32::MAX;
            for seed in seed_range.start..seed_range.end {
                result = std::cmp::min(maps.iter().fold(seed, |a, f| f.get(a)), result);
            }
            result
        })
        .min()
        .unwrap();

    println!("Part 1: {:?}\r\nPart 2: {:?}", part1, part2);
}
