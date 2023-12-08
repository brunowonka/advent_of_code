use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

const INPUT: &str = std::include_str!("input/day4.txt");

#[test]
fn part_1() {
    let lines = INPUT.lines();
    let line_regex = Regex::new(r"Card +[0-9]+: ([0-9 ]+) \| ([0-9 ]+)").unwrap();
    let number_regex = Regex::new("[0-9]+").unwrap();
    let ans = lines
        .map(|l| {
            let capture = line_regex.captures(l).expect("captures");
            let winning = number_regex.find_iter(capture.get(2).unwrap().as_str());
            let winning = winning
                .map(|m| usize::from_str(m.as_str()).unwrap())
                .collect::<HashSet<_>>();

            let count = number_regex
                .find_iter(capture.get(1).unwrap().as_str())
                .filter_map(|m| {
                    let num = usize::from_str(m.as_str()).unwrap();
                    winning.contains(&num).then_some(())
                })
                .count();
            count.checked_sub(1).map(|c| 1 << c).unwrap_or(0)
        })
        .sum::<usize>();
    println!("day 4 part 1 = {ans}");
}

#[test]
fn part_2() {
    let lines = INPUT.lines();
    let line_regex = Regex::new(r"Card +([0-9]+): ([0-9 ]+) \| ([0-9 ]+)").unwrap();
    let number_regex = Regex::new("[0-9]+").unwrap();
    let (_, ans) = lines.fold((VecDeque::new(), 0usize), |(mut mem, score), l| {
        let capture = line_regex.captures(l).expect("captures");
        let winning = number_regex.find_iter(capture.get(3).unwrap().as_str());
        let winning = winning
            .map(|m| usize::from_str(m.as_str()).unwrap())
            .collect::<HashSet<_>>();

        let count = number_regex
            .find_iter(capture.get(2).unwrap().as_str())
            .filter_map(|m| {
                let num = usize::from_str(m.as_str()).unwrap();
                winning.contains(&num).then_some(())
            })
            .count();

        let this_score = 1 + mem.pop_front().unwrap_or(0);
        for i in 0..count {
            if let Some(x) = mem.get_mut(i) {
                *x += this_score;
            } else {
                mem.push_back(this_score);
            }
        }
        println!("{l} => {count} {this_score} {mem:?}");
        (mem, score + this_score)
    });
    println!("day 4 part 2 = {ans}");
}
