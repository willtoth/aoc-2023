#![feature(ascii_char)]
#![feature(ascii_char_variants)]

use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let instructions = input.lines().nth(0).unwrap().as_ascii().unwrap().to_vec();
    let mut rows: Vec<(usize, usize)> = vec![(0, 0); input.lines().skip(2).count()];

    let row_names = input
        .lines()
        .skip(2)
        .map(|line| line.split(" = ").nth(0).unwrap())
        .collect::<Vec<&str>>();

    let rows_ending_in_z = row_names
        .iter()
        .enumerate()
        .filter(|(_, x)| x.ends_with("Z"))
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();
    let lookup = input
        .lines()
        .skip(2)
        .enumerate()
        .map(|(idx, line)| (line.split(" = ").nth(0).unwrap(), idx))
        .collect::<HashMap<&str, usize>>();

    for (i, line) in input.lines().skip(2).enumerate() {
        let re = Regex::new(r"(?<L>[A-Z]*), (?<R>[A-Z]*)")
            .unwrap()
            .captures(line)
            .unwrap();
        let l = &re["L"];
        let r = &re["R"];
        rows[i] = (*lookup.get(l).unwrap(), *lookup.get(r).unwrap());
    }

    fn run_puzzle(
        start_row: usize,
        lookup: &HashMap<&str, usize>,
        instructions: &Vec<std::ascii::Char>,
        rows: &Vec<(usize, usize)>,
        rows_ending_in_z: Option<&Vec<usize>>,
    ) -> u32 {
        let mut current_row = start_row;
        let part2 = rows_ending_in_z.is_some();
        let mut cnt = 0u32;
        let mut goal = lookup["ZZZ"];

        while current_row != goal {
            for inst in instructions.iter() {
                cnt += 1;
                match inst {
                    std::ascii::Char::CapitalL => current_row = rows[current_row].0,
                    std::ascii::Char::CapitalR => current_row = rows[current_row].1,
                    _ => panic!(),
                }

                if !part2 && current_row == goal {
                    break;
                }

                if part2 && rows_ending_in_z.unwrap().contains(&current_row) {
                    goal = current_row;
                    break;
                }
            }
        }
        cnt
    }

    let part1 = run_puzzle(lookup["AAA"], &lookup, &instructions, &rows, None);

    let ends_in_a = row_names
        .iter()
        .filter(|x| x.ends_with("A"))
        .map(|x| lookup[x])
        .collect::<Vec<usize>>();

    let part2 = ends_in_a
        .iter()
        .map(|row: &usize| run_puzzle(*row, &lookup, &instructions, &rows, Some(&rows_ending_in_z)))
        .fold(1u64, |lcm, val| num::integer::lcm(lcm, val as u64));

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
