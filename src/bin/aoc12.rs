#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(entry_insert)]
use core::ascii;

use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt::Debug, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Field {
    UNKNOWN,
    EMPTY,
    SPRING,
}

#[derive(Debug, Clone, Hash, PartialEq)]
struct PuzzleField {
    fields: Vec<Field>,
    lengths: Vec<usize>,
}

impl From<ascii::Char> for Field {
    fn from(value: ascii::Char) -> Self {
        match value {
            ascii::Char::FullStop => Self::EMPTY,
            ascii::Char::QuestionMark => Self::UNKNOWN,
            ascii::Char::NumberSign => Self::SPRING,
            _ => {
                panic!();
            }
        }
    }
}

impl From<Field> for char {
    fn from(value: Field) -> Self {
        match value {
            Field::EMPTY => '.',
            Field::UNKNOWN => '?',
            Field::SPRING => '#',
            _ => {
                panic!();
            }
        }
    }
}

impl From<&str> for PuzzleField {
    fn from(value: &str) -> Self {
        let mut value = value.split(" ");
        let first_half = value.next().unwrap();
        let second_half = value.next().unwrap();

        let fields = first_half
            .as_ascii()
            .unwrap()
            .iter()
            .map(|x| Field::from(*x))
            .collect::<Vec<Field>>();

        PuzzleField {
            fields,
            lengths: second_half
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        }
    }
}

fn fields_to_str(v: &Vec<Field>) -> String {
    format!("{}", v.iter().map(|x| char::from(*x)).collect::<String>())
}

// Hacky memoization...
static mut LOOKUP: Lazy<HashMap<(usize, usize, usize), Option<(usize, bool)>>> =
    Lazy::new(|| HashMap::new());
static mut LOOKUP2: Lazy<HashMap<(String, usize), u64>> = Lazy::new(|| HashMap::new());

// Fill in one size of spring block and the portion of the vector after this found a location
fn fill_in_greedy(v: &Vec<Field>, start_idx: usize, size: usize) -> Option<(usize, bool)> {
    unsafe {
        if let Some(result) = LOOKUP.get(&(v.len(), start_idx, size)) {
            return *result;
        }
    }
    if size > v.len() {
        return None;
    }

    // 'march' starting left to right finding the first possible location to fit 'size' elements
    // Lots of extra checking, but that can be optimized later :shurg:
    for field_idx in start_idx..=(v.len() - size) {
        let prev_char = if field_idx == 0 {
            None
        } else {
            v.get(field_idx - 1)
        };

        if v[field_idx..(field_idx + size)]
            .iter()
            .all(|x| *x == Field::SPRING || *x == Field::UNKNOWN)
            && (*prev_char.unwrap_or(&Field::EMPTY) != Field::SPRING)
            && (*v.get(field_idx + size).unwrap_or(&Field::EMPTY) != Field::SPRING)
        {
            // Return the next possible location, so skip any '.', one '?', or the last field is false
            // if the first element is a '#'
            let had_numsign = v[field_idx] == Field::SPRING;
            unsafe {
                LOOKUP
                    .entry((v.len(), start_idx, size))
                    .insert_entry(Some((field_idx, had_numsign)));
            }
            return Some((field_idx, had_numsign));
        } else if v[field_idx] == Field::SPRING {
            // Can't proceed further, but did not find a match
            unsafe {
                LOOKUP.entry((v.len(), start_idx, size)).insert_entry(None);
            }
            return None;
        }
    }

    unsafe {
        LOOKUP.entry((v.len(), start_idx, size)).insert_entry(None);
    }
    None
}

impl PuzzleField {
    fn unfold(&self) -> PuzzleField {
        let mut fields = self.fields.clone();
        let mut lengths = self.lengths.clone();

        for _ in 0..4 {
            fields.push(Field::UNKNOWN);
            fields.extend_from_slice(&self.fields);

            lengths.extend_from_slice(&self.lengths);
        }

        PuzzleField { fields, lengths }
    }

    fn run_all(fields: &Vec<Field>, sizes: &Vec<usize>) -> u64 {
        unsafe {
            if let Some(result) = LOOKUP2.get(&(fields_to_str(&fields), sizes.len())) {
                return *result;
            }
        }
        let mut result = 0u64;
        if let Some(next_size) = sizes.first() {
            // Step 1. Collect all possible locations for next_size with remaining puzzle
            // This means advance by 1 (or as many .) until either at the end of the line
            // or the first character is a # (can't skip this)
            let mut combos: Vec<usize> = vec![];
            let mut cur_idx = 0usize;
            loop {
                if let Some((idx, quit_early)) = fill_in_greedy(fields, cur_idx, *next_size) {
                    combos.push(idx);
                    if quit_early || (cur_idx + next_size) > fields.len() {
                        break;
                    }
                    cur_idx = idx + 1;
                } else {
                    break;
                }
            }

            // Step 1. Print the results
            for combo in combos.iter() {
                let mut print_fields = fields.clone();
                for idx in *combo..(*combo + next_size) {
                    print_fields[idx] = Field::SPRING;
                }
            }

            // Step 2. For each possible, get the remaining, 1+ the size and call again
            for combo in combos {
                if (combo + next_size + 1) > fields.len() {
                    if sizes.len() == 1 {
                        result += Self::run_all(&vec![], &sizes[1..].to_vec());
                    }
                    continue;
                }
                result += Self::run_all(
                    &fields[(combo + next_size + 1)..].to_vec(),
                    &sizes[1..].to_vec(),
                );
            }
        } else {
            // Step 3. any time sizes is exhausted, increment one
            // Get all possible ways to fill in the size from remaining fields
            if fields.iter().any(|x| *x == Field::SPRING) {
                unsafe {
                    LOOKUP2
                        .entry((fields_to_str(&fields), sizes.len()))
                        .insert_entry(0);
                }
                return 0;
            }
            unsafe {
                LOOKUP2
                    .entry((fields_to_str(&fields), sizes.len()))
                    .insert_entry(1);
            }
            return 1;
        }

        unsafe {
            LOOKUP2
                .entry((fields_to_str(&fields), sizes.len()))
                .insert_entry(result);
        }
        result
    }

    fn run(&self) -> u64 {
        unsafe {
            LOOKUP.clear();
            LOOKUP2.clear();
        }
        Self::run_all(&self.fields, &self.lengths)
    }
}

fn main() {
    let input = fs::read_to_string("input_aoc12.txt").expect("Unable to read file");

    let input = input
        .lines()
        .map(|line| PuzzleField::from(line))
        .collect::<Vec<PuzzleField>>();

    let part1 = input.iter().map(|p| p.run() as u32).sum::<u32>();
    println!("Part 1: {:?}", part1);

    let part2 = input.iter().map(|p| p.unfold().run() as u64).sum::<u64>();

    println!("Part 2: {:?}", part2);
}
