use std::collections::{HashMap, HashSet, VecDeque};

use super::{Direction, Map};

const INPUT: &str = std::include_str!("input/day23.txt");

enum Tile {
    Ground,
    Rock,
    Slope(Direction),
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Ground,
            '#' => Self::Rock,
            '>' => Self::Slope(Direction::Right),
            '^' => Self::Slope(Direction::Up),
            '<' => Self::Slope(Direction::Left),
            'v' => Self::Slope(Direction::Down),
            o => panic!("unknown tile {o}"),
        }
    }
}

fn input(input: &str) -> Map<Tile> {
    let cols = input.lines().next().unwrap().len();
    let tiles = input
        .lines()
        .map(|l| l.chars())
        .flatten()
        .map(Tile::new)
        .collect();
    Map::new(tiles, cols)
}

fn walk(pos: usize, map: &Map<Tile>, slopes: bool) -> usize {
    let mut stack = VecDeque::new();
    stack.push_back((HashSet::new(), pos));
    let mut max = 0;
    while let Some((mut visited, pos)) = stack.pop_back() {
        let dirs = match map.at(pos).unwrap() {
            Tile::Ground => itertools::Either::Left(Direction::all()),
            Tile::Slope(d) => {
                if slopes {
                    itertools::Either::Right(std::iter::once(*d))
                } else {
                    itertools::Either::Left(Direction::all())
                }
            }
            Tile::Rock => panic!("can't visit rocks"),
        };
        assert!(visited.insert(pos));
        let v = map.converter().to_vector(pos);
        if v.y == map.bounds().y - 1 {
            max = max.max(visited.len());
            continue;
        }

        for d in dirs {
            let n = v + d.to_vector();
            let ni = if let Some(i) = map.converter().to_idx(n) {
                i
            } else {
                continue;
            };
            if visited.contains(&ni) {
                continue;
            }
            match map.at(ni).unwrap() {
                Tile::Rock => continue,
                _ => (),
            }
            stack.push_back((visited.clone(), ni));
        }
    }
    max
}

fn visit_and_acc(
    pos: usize,
    count: usize,
    map: &Map<Tile>,
    direction: Direction,
    inclusive: bool,
) -> Option<(usize, usize)> {
    let v = map.converter().to_vector(pos);
    if v.y == map.bounds().y - 1 {
        return Some((pos, count));
    }

    let mut next = None;
    for d in Direction::all() {
        if inclusive {
            if d != direction {
                continue;
            }
        } else {
            if d == direction {
                continue;
            }
        }
        let v = d.to_vector() + v;
        let idx = map.converter().to_idx(v);
        match idx.and_then(|v| map.at(v)) {
            None | Some(Tile::Rock) => continue,
            Some(Tile::Ground) | Some(Tile::Slope(_)) => {
                if let Some(_) = std::mem::replace(&mut next, Some((idx.unwrap(), d.opposite()))) {
                    return Some((pos, count));
                }
            }
        }
    }
    next.and_then(|(n, d)| visit_and_acc(n, count + 1, map, d, false))
}

fn compress(map: &Map<Tile>) -> HashMap<usize, Vec<(usize, usize)>> {
    let mut r = HashMap::<usize, Vec<(usize, usize)>>::new();
    for (i, n) in map.nodes.iter().enumerate() {
        match n {
            Tile::Ground | Tile::Slope(_) => (),
            Tile::Rock => continue,
        }
        let vs = Direction::all()
            .filter_map(|d| visit_and_acc(i, 0, map, d, true))
            .collect::<Vec<_>>();
        assert!(r.insert(i, vs).is_none());
    }
    r
}

fn walk_compressed(start: usize, end: usize, map: &HashMap<usize, Vec<(usize, usize)>>) -> usize {
    let mut stack = VecDeque::new();
    stack.push_back((HashSet::new(), 0, start));
    let mut max = 0;
    while let Some((mut visited, len, pos)) = stack.pop_back() {
        if pos == end {
            max = max.max(len);
            continue;
        }
        assert!(visited.insert(pos));

        let v = map.get(&pos).unwrap();
        for (nxt, nlen) in v.iter().copied() {
            if visited.contains(&nxt) {
                continue;
            }
            stack.push_back((visited.clone(), len + nlen, nxt));
        }
    }
    max
}
#[test]
fn part_1() {
    let map = input(INPUT);
    let start = map
        .nodes
        .iter()
        .enumerate()
        .find_map(|(i, n)| match n {
            Tile::Ground => Some(i),
            _ => None,
        })
        .unwrap();
    let ans = walk(start, &map, true) - 1;
    println!("day 23 part 1 = {ans}");
}

// This is rather slow, couldn't figure out any memoization to improve, but
// compressing the map makes it run to completion in < 1min.
#[test]
fn part_2() {
    let map = input(INPUT);
    let start = map
        .nodes
        .iter()
        .enumerate()
        .find_map(|(i, n)| match n {
            Tile::Ground => Some(i),
            _ => None,
        })
        .unwrap();
    let end = map
        .nodes
        .iter()
        .enumerate()
        .find_map(|(i, n)| {
            if map.converter().to_vector(i).y != map.bounds().y - 1 {
                return None;
            }
            match n {
                Tile::Ground => Some(i),
                _ => None,
            }
        })
        .unwrap();
    let compressed = compress(&map);
    let ans = walk_compressed(start, end, &compressed);
    println!("day 23 part 2 = {ans}");
}
