use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    // Part 2
    let input = input
        .lines()
        .map(|g| {
            g.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .replace("zero", "zero0zero")
        })
        .collect::<Vec<String>>();

    // Part 1
    println!(
        "Result: {:?}",
        input
            .iter()
            .map(|line| {
                let mut itr = line.bytes().filter(|c| c.is_ascii_digit());
                let first = itr.nth(0).unwrap();
                let last = itr.nth_back(0).unwrap_or(first);
                let result = String::from_utf8(vec![first, last]).unwrap();
                result.parse::<i32>().unwrap()
            })
            .sum::<i32>()
    );
}
