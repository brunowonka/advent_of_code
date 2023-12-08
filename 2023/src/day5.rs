use itertools::Itertools;
use regex::Regex;
use std::{cmp, str::FromStr};

const INPUT: &str = std::include_str!("input/day5.txt");

#[test]
fn part_1() {
    let mut lines = INPUT.lines();
    let numbers_regex = Regex::new("[0-9]+").unwrap();
    let seeds = lines
        .by_ref()
        .find_map(|l| {
            if !l.starts_with("seeds:") {
                return None;
            }
            Some(
                numbers_regex
                    .find_iter(l)
                    .map(|m| usize::from_str(m.as_str()).unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap();

    let range_regex = Regex::new("([0-9]+) ([0-9]+) ([0-9]+)").unwrap();
    let (src, dst) = lines.fold((seeds, Vec::new()), |(mut src, mut dst), l| {
        if l.is_empty() {
            return (src, dst);
        }
        if l.contains("map") {
            println!("end {l} => {src:?} {dst:?}");
            // New section, drain dst back to src. Things still in src are the
            // same value.
            src.extend(dst.drain(..));
            return (src, dst);
        }
        let captures = range_regex.captures(&l).unwrap();
        let get_val = |i: usize| usize::from_str(captures.get(i).unwrap().as_str()).unwrap();
        let dst_start = get_val(1);
        let src_start = get_val(2);
        let len = get_val(3);
        src.retain(|src| {
            let src = *src;
            let mapped = src
                .checked_sub(src_start)
                .and_then(|delta| (delta < len).then_some(dst_start + delta));
            if let Some(mapped) = mapped {
                println!("{l} {src} => {mapped}");
                dst.push(mapped);
                false
            } else {
                true
            }
        });
        (src, dst)
    });
    let ans = src.into_iter().chain(dst.into_iter()).min().unwrap();
    println!("day 5 part 1 = {ans}");
}

#[test]
fn part_2() {
    let mut lines = INPUT.lines();
    let numbers_regex = Regex::new("[0-9]+").unwrap();
    let seeds = lines
        .by_ref()
        .find_map(|l| {
            if !l.starts_with("seeds:") {
                return None;
            }
            Some(
                numbers_regex
                    .find_iter(l)
                    .tuples()
                    .map(|(start, end)| {
                        let start = usize::from_str(start.as_str()).unwrap();
                        let end = start + usize::from_str(end.as_str()).unwrap();
                        start..end
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap();

    let range_regex = Regex::new("([0-9]+) ([0-9]+) ([0-9]+)").unwrap();
    let (src, dst) = lines.fold((seeds, Vec::new()), |(mut src, mut dst), l| {
        if l.is_empty() {
            return (src, dst);
        }
        if l.contains("map") {
            // New section, drain dst back to src. Things still in src are the
            // same value.
            src.extend(dst.drain(..));
            return (src, dst);
        }
        let captures = range_regex.captures(&l).unwrap();
        let get_val = |i: usize| usize::from_str(captures.get(i).unwrap().as_str()).unwrap();
        let dst_start = get_val(1);
        let src_start = get_val(2);
        let len = get_val(3);
        let src_end = src_start + len;
        let src_range = src_start..src_end;
        let mut split = Vec::new();
        src.retain_mut(|src| {
            let map_start = cmp::max(src.start, src_range.start);
            let map_end = cmp::min(src.end, src_range.end);
            // Disjoint.
            if map_end <= map_start {
                return true;
            }
            // Push mapped.
            let start = map_start - src_range.start + dst_start;
            let end = map_end - src_range.start + dst_start;
            dst.push(start..end);

            let orig_src = src.clone();
            let extra_left = orig_src.start < map_start;
            if extra_left {
                // Part of the range is to the left. Mutate the end.
                src.end = map_start;
            }
            let extra_right = orig_src.end > map_end;
            if extra_right {
                if extra_left {
                    // Twice split.
                    split.push(map_end..orig_src.end);
                } else {
                    // Once split.
                    src.start = map_end;
                }
            }
            extra_left || extra_right
        });
        src.extend(split.into_iter());
        (src, dst)
    });
    let ans = src
        .into_iter()
        .chain(dst.into_iter())
        .map(|r| r.start)
        .min()
        .unwrap();
    println!("day 5 part 2 = {ans}");
}
