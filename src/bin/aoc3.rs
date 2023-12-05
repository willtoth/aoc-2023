use regex::Regex;
use std::fs;

#[derive(Debug, Clone)]
struct Symbol {
    col: usize,
    gears: Vec<i32>,
}

impl Symbol {
    fn new(col: usize) -> Symbol {
        Symbol { col, gears: vec![] }
    }
}

fn is_adjacent(
    symbol_table: &mut Vec<Vec<Symbol>>,
    number: i32,
    row: usize,
    col_start: usize,
) -> bool {
    let first_row = std::cmp::max(row as i32 - 1, 0) as usize;
    let last_row = std::cmp::min(row + 1, symbol_table.len() - 1);

    // no need to bounds check these
    let first_col = col_start as i32 - 1;
    let last_col = col_start + number.to_string().len();

    symbol_table[first_row..last_row + 1]
        .iter_mut()
        .any(|symbols| {
            symbols.iter_mut().any(|index| {
                let result = index.col as i32 >= first_col && index.col <= last_col;
                if result {
                    index.gears.push(number);
                }
                result
            })
        })
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    // i = row, j = column
    let mut symbol_table = vec![vec![]; input.lines().count()];
    let mut gear_table = vec![vec![]; input.lines().count()];

    // Part 1
    for (i, line) in input.lines().enumerate() {
        line.char_indices()
            .filter(|(_, c)| *c != '.' && c.is_ascii_punctuation())
            .for_each(|(j, _)| symbol_table[i].push(Symbol::new(j)));
    }

    let result = input.lines().enumerate().map(|(i, line)| {
        Regex::new(r"\d*")
            .unwrap()
            .find_iter(line)
            .filter_map(|capture| {
                if capture.is_empty() {
                    return None;
                }
                let val = capture.as_str().parse::<i32>().unwrap();
                let col = capture.start();
                if is_adjacent(&mut symbol_table, val, i, col) {
                    Some(val)
                } else {
                    None
                }
            })
            .sum::<i32>()
    });
    println!("Part 1: {:?}", result.sum::<i32>());

    // Part 2
    for (i, line) in input.lines().enumerate() {
        line.char_indices()
            .filter(|(_, c)| *c == '*')
            .for_each(|(j, _)| gear_table[i].push(Symbol::new(j)));
    }

    let result = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            Regex::new(r"\d*")
                .unwrap()
                .find_iter(line)
                .filter_map(|capture| {
                    if capture.is_empty() {
                        return None;
                    }
                    let val = capture.as_str().parse::<i32>().unwrap();
                    let col = capture.start();
                    if is_adjacent(&mut gear_table, val, i, col) {
                        Some(val)
                    } else {
                        None
                    }
                })
                .collect::<Vec<i32>>()
        })
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum::<i32>();

    let result = gear_table
        .iter()
        .map(|inner| {
            inner
                .iter()
                .filter(|x| x.gears.len() == 2)
                .map(|x| x.gears[0] * x.gears[1])
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Part 2: {:?}", result);
}
