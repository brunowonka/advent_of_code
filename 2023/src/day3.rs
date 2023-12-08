use regex::Regex;
use std::{collections::HashMap, str::FromStr};

const INPUT: &str = std::include_str!("input/day3.txt");

#[test]
fn part_1() {
    let matrix = INPUT.lines().collect::<Vec<_>>();

    let is_symbol = |row: usize, col: usize| {
        matrix
            .get(row)
            .and_then(|s| s.as_bytes().get(col).map(|c: &u8| *c as char))
            .map(|c| c != '.' && !c.is_ascii_digit())
            .unwrap_or(false)
    };

    let regex = Regex::new("[0-9]+").unwrap();

    let ans: usize = matrix
        .iter()
        .enumerate()
        .map(|(row, line)| {
            regex
                .find_iter(line)
                .map(|m| {
                    let v = usize::from_str(m.as_str()).unwrap();
                    let mut range = m.range();
                    if range.start > 0 {
                        range.start -= 1;
                        if is_symbol(row, range.start) {
                            return v;
                        }
                    }
                    if is_symbol(row, range.end) {
                        return v;
                    }
                    range.end += 1;

                    if let Some(r) = row.checked_sub(1) {
                        if range.clone().any(|i| is_symbol(r, i)) {
                            return v;
                        }
                    }
                    let next_row = row + 1;
                    if next_row < matrix.len() {
                        if range.clone().any(|i| is_symbol(next_row, i)) {
                            return v;
                        }
                    }
                    0
                })
                .sum::<usize>()
        })
        .sum();

    println!("day 3 part 1 = {ans}");
}

#[test]
fn part_2() {
    let matrix = INPUT.lines().collect::<Vec<_>>();

    let is_gear = |row: usize, col: usize| {
        matrix
            .get(row)
            .and_then(|s| s.as_bytes().get(col).map(|c: &u8| *c as char))
            .map(|c| c == '*')
            .unwrap_or(false)
    };

    let regex = Regex::new("[0-9]+").unwrap();

    let mut gears = HashMap::new();
    let mut ans = 0;
    for (row, line) in matrix.iter().enumerate() {
        let look = regex
            .find_iter(line)
            .map(|m| {
                let v = usize::from_str(m.as_str()).unwrap();
                let mut range = m.range();
                range.start = range.start.saturating_sub(1);
                range.end += 1;

                let prev = if let Some(r) = row.checked_sub(1) {
                    itertools::Either::Left(range.clone().map(move |i| (r, i)))
                } else {
                    itertools::Either::Right(std::iter::empty())
                };
                let next_row = row + 1;
                let nxt = if next_row < matrix.len() {
                    itertools::Either::Left(range.clone().map(move |i| (next_row, i)))
                } else {
                    itertools::Either::Right(std::iter::empty())
                };
                prev.chain(nxt)
                    .chain(std::iter::once((row, range.start)))
                    .chain(std::iter::once((row, range.end - 1)))
                    .map(move |(r, i)| (v, r, i))
            })
            .flatten();
        for (value, r, i) in look {
            if !is_gear(r, i) {
                continue;
            }
            match gears.entry((r, i)) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    let (count, val) = *o.get();
                    let val = if count == 1 {
                        let val = val * value;
                        ans += val;
                        val
                    } else if count == 2 {
                        ans -= val;
                        0
                    } else {
                        0
                    };
                    *o.get_mut() = (count + 1, val);
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((1, value));
                }
            }
        }
    }

    println!("day 3 part 2 = {ans}");
}
