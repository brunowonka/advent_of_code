use std::{cmp, collections::HashMap, fmt::Debug, str::FromStr};

use itertools::Itertools;

const INPUT: &str = std::include_str!("input/day7.txt");

#[derive(Ord, PartialEq, PartialOrd, Eq, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    W,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::C2,
            '3' => Self::C3,
            '4' => Self::C4,
            '5' => Self::C5,
            '6' => Self::C6,
            '7' => Self::C7,
            '8' => Self::C8,
            '9' => Self::C9,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            c => panic!("invalid {c}"),
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::C2 => write!(f, "2"),
            Self::C3 => write!(f, "3"),
            Self::C4 => write!(f, "4"),
            Self::C5 => write!(f, "5"),
            Self::C6 => write!(f, "6"),
            Self::C7 => write!(f, "7"),
            Self::C8 => write!(f, "8"),
            Self::C9 => write!(f, "9"),
            Self::T => write!(f, "T"),
            Self::J => write!(f, "J"),
            Self::Q => write!(f, "Q"),
            Self::K => write!(f, "K"),
            Self::A => write!(f, "A"),
            Self::W => write!(f, "W"),
        }
    }
}

type Game = [Card; 5];

fn input(f: impl Fn(Card) -> Card) -> impl Iterator<Item = (Game, usize)> {
    INPUT.lines().map(move |l| {
        let (cards, value) = l.split_once(" ").unwrap();
        assert_eq!(cards.len(), 5);
        let (a, b, c, d, e) = cards
            .chars()
            .map(|c| f(Card::from(c)))
            .collect_tuple()
            .unwrap();
        let game = [a, b, c, d, e];
        let value = usize::from_str(value).unwrap();
        (game, value)
    })
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Outcome {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    Pair,
    None,
}

impl Outcome {
    fn from_game(game: &Game) -> Self {
        let (map, max) = game
            .iter()
            .fold((HashMap::new(), 0), |(mut map, max), card| {
                let count = match map.entry(card) {
                    std::collections::hash_map::Entry::Occupied(mut o) => {
                        *o.get_mut() += 1;
                        *o.get_mut()
                    }
                    std::collections::hash_map::Entry::Vacant(v) => {
                        v.insert(1);
                        1
                    }
                };
                let max = if *card == Card::W {
                    max
                } else {
                    cmp::max(max, count)
                };
                (map, max)
            });

        let (len, max) = match map.get(&Card::W).copied() {
            Some(v) => (cmp::max(map.len() - 1, 1), max + v),
            None => (map.len(), max),
        };

        match (len, max) {
            (5, 1) => Self::None,
            (4, 2) => Self::Pair,
            (3, 2) => Self::TwoPair,
            (3, 3) => Self::Three,
            (2, 3) => Self::FullHouse,
            (2, 4) => Self::Four,
            (1, 5) => Self::Five,
            x => panic!("unexpected game {x:?} {map:?}"),
        }
    }
}

#[test]
fn part1() {
    let mut vec = input(|c| c)
        .map(|(game, value)| (Outcome::from_game(&game), game, value))
        .collect::<Vec<_>>();
    vec.sort_by(|(outcome, game, _), (o, g, _)| cmp::Ord::cmp(&(outcome, game), &(o, g)).reverse());
    let ans: usize = vec
        .iter()
        .enumerate()
        .map(|(rank, (outcome, game, value))| {
            // println!("{outcome:?} {game:?}");
            let _ = outcome;
            let _ = game;
            (rank + 1) * *value
        })
        .sum();
    println!("day 7 part 1 = {ans}");
}

#[test]
fn part2() {
    let mut vec = input(|c| match c {
        Card::J => Card::W,
        c => c,
    })
    .map(|(game, value)| (Outcome::from_game(&game), game, value))
    .collect::<Vec<_>>();
    vec.sort_by(|(outcome, game, _), (o, g, _)| cmp::Ord::cmp(&(outcome, game), &(o, g)).reverse());
    let ans: usize = vec
        .iter()
        .enumerate()
        .map(|(rank, (outcome, game, value))| {
            // println!("{outcome:?} {game:?}");
            let _ = outcome;
            let _ = game;
            (rank + 1) * *value
        })
        .sum();
    println!("day 7 part 2 = {ans}");
}
