use crate::{Direction, Map};
use std::collections::{HashMap, VecDeque};

const INPUT: &str = std::include_str!("input/day21.txt");

struct Node {
    available: bool,
    candidate_for_generation: Option<usize>,
}

impl Node {
    fn new(c: char) -> Self {
        let (available, candidate_for_generation) = match c {
            '.' => (true, None),
            '#' => (false, None),
            'S' => (true, Some(0)),
            o => panic!("unexpected char {o}"),
        };
        Self {
            available,
            candidate_for_generation,
        }
    }
}

fn input(str: &str) -> Map<Node> {
    let cols = str.lines().next().unwrap().len();
    let v = str
        .lines()
        .map(|l| l.chars().map(|c| Node::new(c)))
        .flatten()
        .collect();
    Map::new(v, cols)
}

#[test]
fn part_1() {
    let mut map = input(INPUT);
    let mut queue = VecDeque::new();
    let start = map
        .nodes
        .iter()
        .enumerate()
        .find_map(|(i, n)| n.candidate_for_generation.as_ref().map(|_| i))
        .unwrap();
    queue.push_back(map.converter().to_vector(start));

    let mut cur_generation = 0;
    while let Some(i) = queue.pop_front() {
        let i_gen = map.at(i).unwrap().candidate_for_generation.unwrap();
        if i_gen == 64 {
            break;
        }
        if i_gen == cur_generation {
            cur_generation = i_gen + 1;
        }

        for d in Direction::all() {
            let d = d.to_vector();
            let candidate = i + d;
            let node = if let Some(c) = map.at_mut(candidate) {
                c
            } else {
                continue;
            };
            if !node.available {
                continue;
            }
            let gen = std::mem::replace(&mut node.candidate_for_generation, Some(cur_generation));
            if gen.map(|g| g != cur_generation).unwrap_or(true) {
                queue.push_back(candidate);
            }
        }
    }

    let ans = queue.len() + 1;
    println!("day 21 part 1 = {ans}");
}

#[test]
fn part_2() {
    let mut map = input(INPUT);
    let mut queue = VecDeque::new();
    let start = map
        .nodes
        .iter()
        .enumerate()
        .find_map(|(i, n)| n.candidate_for_generation.as_ref().map(|_| i))
        .unwrap();
    queue.push_back((0, map.converter().to_vector(start)));

    let mut mem = HashMap::new();

    while let Some((steps, i)) = queue.pop_front() {
        for d in Direction::all() {
            let d = d.to_vector();
            let candidate = i + d;
            let node = if let Some(c) = map.at_mut(candidate) {
                c
            } else {
                continue;
            };
            if !node.available {
                continue;
            }

            match mem.entry(candidate) {
                std::collections::hash_map::Entry::Occupied(_) => {
                    continue;
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    let _ = v.insert(steps + 1);
                    queue.push_back((steps + 1, candidate));
                }
            }
        }
    }

    // Copied from
    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21

    let even_corners = mem.values().filter(|v| **v % 2 == 0 && **v > 65).count();
    let odd_corners = mem.values().filter(|v| **v % 2 == 1 && **v > 65).count();

    let even_full = mem.values().filter(|v| **v % 2 == 0).count();
    let odd_full = mem.values().filter(|v| **v % 2 == 1).count();

    // This is 202300 but im writing it out here to show the process
    let n = ((26501365 - (map.cols / 2)) / map.cols) as usize;
    assert_eq!(n, 202300);

    let ans = ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners
        + n * even_corners;

    println!("day 21 part 2 = {ans}");
}
