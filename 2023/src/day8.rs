use std::collections::HashMap;

use itertools::{FoldWhile, Itertools};

const INPUT: &str = std::include_str!("input/day8.txt");

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            o => panic!("{o}"),
        }
    }
}

fn input() -> (Vec<Dir>, HashMap<String, (String, String)>) {
    let mut lines = INPUT.lines();
    let dirs = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Dir::from(c))
        .collect::<Vec<_>>();

    let regex = regex::Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap();
    let map = lines
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            let captures = regex.captures(l).unwrap_or_else(|| panic!("{l}"));
            let key = captures.get(1).unwrap().as_str().to_string();
            let left = captures.get(2).unwrap().as_str().to_string();
            let right = captures.get(3).unwrap().as_str().to_string();
            Some((key, (left, right)))
        })
        .collect::<HashMap<_, _>>();
    (dirs, map)
}

#[test]
fn part1() {
    let (dir, map) = input();
    let mut dir = std::iter::repeat(dir.iter()).flatten();

    let (_, steps) = dir
        .fold_while(("AAA", 0), |(cur, count), dir| {
            let count = count + 1;
            let m = map.get(cur).unwrap();
            let nxt = match dir {
                Dir::Left => &m.0,
                Dir::Right => &m.1,
            };
            if nxt == "ZZZ" {
                FoldWhile::Done((nxt, count))
            } else {
                FoldWhile::Continue((nxt, count))
            }
        })
        .into_inner();

    println!("day 8 part 1 = {steps}");
}

#[test]
fn part2() {
    let (dir, map) = input();

    let steps = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|start| {
            let mut dir = std::iter::repeat(dir.iter()).flatten();

            let (_, steps) = dir
                .fold_while((start, 0u64), |(cur, count), dir| {
                    let count = count + 1;
                    let m = map.get(cur).unwrap();
                    let nxt = match dir {
                        Dir::Left => &m.0,
                        Dir::Right => &m.1,
                    };
                    if nxt.ends_with("Z") {
                        FoldWhile::Done((nxt, count))
                    } else {
                        FoldWhile::Continue((nxt, count))
                    }
                })
                .into_inner();
            steps
        })
        .fold(1, |l, v| num::integer::lcm(l, v));

    println!("day 8 part 2 = {steps}");
}
