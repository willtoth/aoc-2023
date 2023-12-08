#![feature(ascii_char)]
#![feature(array_chunks)]

use std::{collections::HashMap, fs};

const FIVE_OF_A_KIND: u32 = 7;
const FOUR_OF_A_KIND: u32 = 6;
const FULL_HOUSE: u32 = 5;
const THREE_OF_A_KIND: u32 = 4;
const TWO_PAIR: u32 = 3;
const PAIR: u32 = 2;
const HIGH_CARD: u32 = 1;

struct PartOne<'a>(&'a str);

#[derive(Debug, Clone, Copy)]
struct Card(u32);

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card(14),
            'K' => Card(13),
            'Q' => Card(12),
            'J' => Card(if cfg!(feature = "part2") { 1 } else { 11 }),
            'T' => Card(10),
            _ => Card(value.to_digit(10).unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    strength: u32,
}

fn card_strength(cards: &[Card; 5]) -> u32 {
    let mut map = HashMap::new();
    for c in cards {
        map.entry(c.0).and_modify(|f| *f += 1).or_insert(1);
    }

    let jokers = map.remove(&1).unwrap_or_default();

    let mut counts = map.iter().map(|(_, v)| *v).collect::<Vec<u32>>();
    counts.sort();
    counts.reverse();

    let highest = *counts.get(0).unwrap_or(&0) + jokers;
    let second = *counts.get(1).unwrap_or(&0);
    match (highest, second) {
        (5, _) => FIVE_OF_A_KIND,
        (4, _) => FOUR_OF_A_KIND,
        (3, 2) => FULL_HOUSE,
        (3, _) => THREE_OF_A_KIND,
        (2, 2) => TWO_PAIR,
        (2, _) => PAIR,
        _ => HIGH_CARD,
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cards = value
            .split(" ")
            .nth(0)
            .unwrap()
            .as_ascii()
            .unwrap()
            .iter()
            .map(|c| Card::from(c.to_char()))
            .collect::<Vec<Card>>();

        let bid = value.split(" ").nth(1).unwrap().parse::<u32>().unwrap() as usize;
        let cards = cards.array_chunks::<5>().nth(0).unwrap().clone();
        let strength = card_strength(&cards);
        Hand {
            cards,
            bid,
            strength,
        }
    }
}

#[derive(Debug, Clone)]
struct PuzzelState {
    hands: Vec<Hand>,
}

impl<'a> From<PartOne<'a>> for PuzzelState {
    fn from(PartOne(value): PartOne<'a>) -> Self {
        PuzzelState {
            hands: value.lines().map(|l| Hand::from(l)).collect(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut result = PuzzelState::from(PartOne(&input));
    result.hands.sort_unstable_by(|a, b| {
        if a.strength == b.strength {
            let mut result = std::cmp::Ordering::Equal;
            for (idx, _) in a.cards.iter().enumerate() {
                if a.cards[idx].0 > b.cards[idx].0 {
                    result = std::cmp::Ordering::Greater;
                    break;
                }
                if a.cards[idx].0 < b.cards[idx].0 {
                    result = std::cmp::Ordering::Less;
                    break;
                }
            }
            result
        } else {
            a.strength.cmp(&b.strength)
        }
    });

    let result = result
        .hands
        .iter()
        .enumerate()
        .fold(0, |total, (i, hand)| total + (i + 1) * hand.bid);
    println!("{:?}", result);
}
