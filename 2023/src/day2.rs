use itertools::{FoldWhile, Itertools};
use regex::Regex;
use std::{cmp, str::FromStr};

enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            o => Err(o.to_string()),
        }
    }
}

const INPUT: &str = std::include_str!("input/day2.txt");

#[test]
fn part_1() {
    let search = Regex::new("(?:Game ([0-9]+):)? ([0-9]+) ([a-z]+)([,;])?").unwrap();
    let lines = INPUT.lines();
    let ans: usize = lines
        .map(|l| {
            let r = search
                .captures_iter(&l)
                .map(|capture| {
                    let _ = capture.get(0).expect("no match");
                    let game = capture
                        .get(1)
                        .map(|game| usize::from_str_radix(game.as_str(), 10).expect("parse game"));
                    let count =
                        usize::from_str_radix(capture.get(2).expect("capture count").as_str(), 10)
                            .expect("parse count");
                    let color = Color::from_str(capture.get(3).expect("capture color").as_str())
                        .expect("color");
                    let end = capture.get(4).map(|m| m.as_str() == ";").unwrap_or(true);
                    (game, count, color, end)
                })
                .fold_while(
                    (None, 0, 0, 0),
                    |(cur_game, mut red, mut green, mut blue), (game, count, color, end)| {
                        let game = match (cur_game, game) {
                            (None, Some(g)) | (Some(g), None) => Some(g),
                            (cur, game) => panic!("{cur:?} {game:?}"),
                        };
                        match color {
                            Color::Red => red += count,
                            Color::Blue => blue += count,
                            Color::Green => green += count,
                        }
                        if red > 12 || green > 13 || blue > 14 {
                            return FoldWhile::Done((None, 0, 0, 0));
                        }
                        if end {
                            red = 0;
                            blue = 0;
                            green = 0;
                        }
                        FoldWhile::Continue((game, red, green, blue))
                    },
                );
            let partial = match r {
                FoldWhile::Done(_) => 0,
                FoldWhile::Continue((game, _, _, _)) => game.expect("no game at end"),
            };

            println!("{l} => {partial}");
            partial
        })
        .sum();
    println!("day 2 part 1 = {ans}");
}

#[test]
fn part_2() {
    let search = Regex::new("(?:Game ([0-9]+):)? ([0-9]+) ([a-z]+)([,;])?").unwrap();
    let lines = INPUT.lines();
    let ans: usize = lines
        .map(|l| {
            let colors = search
                .captures_iter(&l)
                .map(|capture| {
                    let _ = capture.get(0).expect("no match");
                    let count =
                        usize::from_str_radix(capture.get(2).expect("capture count").as_str(), 10)
                            .expect("parse count");
                    let color = Color::from_str(capture.get(3).expect("capture color").as_str())
                        .expect("color");
                    (count, color)
                })
                .fold(
                    (0, 0, 0),
                    |(mut red, mut green, mut blue), (count, color)| {
                        let target = match color {
                            Color::Red => &mut red,
                            Color::Blue => &mut blue,
                            Color::Green => &mut green,
                        };
                        *target = cmp::max(*target, count);
                        (red, green, blue)
                    },
                );
            println!("{l} => {colors:?}");
            let (r, g, b) = colors;
            r * g * b
        })
        .sum();
    println!("day 2 part 2 = {ans}");
}
