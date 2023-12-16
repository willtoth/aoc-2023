#![feature(ascii_char)]
use std::fs;

use regex::Regex;

fn run_hash(s: &String) -> u32 {
    s.chars()
        .map(|x| x as u32)
        .fold(0 as u32, |v, x| ((v + x) * 17) % 256)
}

#[derive(Debug, Clone)]
struct LensBox {
    lens: Vec<(String, u32)>,
}

impl LensBox {
    fn new() -> LensBox {
        LensBox { lens: vec![] }
    }

    fn remove(&mut self, label: &String) {
        if let Some(idx) = self.lens.iter().position(|x| x.0 == *label) {
            self.lens.remove(idx);
        }
    }

    fn insert(&mut self, label: &String, value: u32) {
        if let Some(existing) = self.lens.iter_mut().find(|x| x.0 == *label) {
            (*existing).1 = value;
        } else {
            self.lens.push((label.clone(), value));
        }
    }

    fn power(&self, b: usize) -> u32 {
        self.lens
            .iter()
            .enumerate()
            .fold(0, |sum, (idx, l)| sum + (b as u32 * l.1 * (idx + 1) as u32))
    }
}

#[derive(Debug)]
struct Operation {
    lens_label: String,
    lens_operation: char,
    strength: Option<u32>,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    println!(
        "Part 1: {}",
        input
            .split(",")
            .map(|x| run_hash(&x.to_string()))
            .sum::<u32>()
    );

    let operations = input.split(",").map(|x| {
        let regex = Regex::new(r"([a-z]*)([-=])(\d*)")
            .unwrap()
            .captures(x)
            .unwrap();
        let lens_label = regex[1].to_string();
        let lens_operation = regex[2].to_string().as_ascii().unwrap()[0].to_char();
        let strength = if lens_operation == '=' {
            Some(regex[3].to_string().parse::<u32>().unwrap())
        } else {
            None
        };

        Operation {
            lens_label,
            lens_operation,
            strength,
        }
    });

    let mut boxes: Vec<LensBox> = vec![LensBox::new(); 256];

    for op in operations {
        match op.lens_operation {
            '=' => {
                boxes[run_hash(&op.lens_label) as usize]
                    .insert(&op.lens_label, op.strength.unwrap());
            }
            '-' => {
                boxes[run_hash(&op.lens_label) as usize].remove(&op.lens_label);
            }
            _ => {
                panic!("Failed with operation: {}", op.lens_operation)
            }
        };
    }
    let part2 = boxes
        .iter()
        .enumerate()
        .map(|(i, x)| x.power(i + 1))
        .sum::<u32>();
    println!("Part 2: {:?}", part2);
}
