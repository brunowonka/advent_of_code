use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const INPUT: &str = std::include_str!("input/day17.txt");

type Vector = euclid::Vector2D<isize, ()>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Candidate {
    distance: usize,
    dir: Vector,
    dir_count: usize,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Node {
    value: usize,
    state: HashMap<Vector, (usize, bool)>,
}

impl Node {
    fn new(value: usize) -> Self {
        Self {
            value,
            state: HashMap::new(),
        }
    }
}

#[derive(Eq, PartialEq)]
struct HeapEntry(Candidate, Vector);

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

struct Map {
    nodes: Vec<Node>,
    cols: usize,
}

impl Map {
    fn new(s: &str) -> Self {
        s.lines()
            .fold(None, |mut map, l| {
                let m = map.get_or_insert_with(|| Map {
                    nodes: Vec::new(),
                    cols: l.len(),
                });
                m.nodes.extend(
                    l.chars()
                        .map(|c| Node::new(c.to_digit(10).unwrap() as usize)),
                );
                map
            })
            .unwrap()
    }

    fn at(&mut self, Vector { x, y, .. }: Vector) -> Option<&mut Node> {
        let x = usize::try_from(x).ok()?;
        if x >= self.cols {
            return None;
        }
        let y = usize::try_from(y).ok()?;
        self.nodes.get_mut(y * self.cols + x)
    }

    fn lines(&self) -> usize {
        self.nodes.len() / self.cols
    }

    fn exit(&self) -> Vector {
        let x = self.cols as isize;
        let y = self.lines() as isize;
        Vector::new(x - 1, y - 1)
    }
}

fn run_crucible(input: &str, can_go: impl Fn(bool, usize) -> bool) -> usize {
    let mut map = Map::new(input);
    let mut heap = BinaryHeap::new();
    heap.push(HeapEntry(
        Candidate {
            distance: 0,
            dir: Vector::new(1, 0),
            dir_count: 0,
        },
        Vector::new(0, 0),
    ));
    let directions = [
        Vector::new(1, 0),
        Vector::new(-1, 0),
        Vector::new(0, -1),
        Vector::new(0, 1),
    ];
    let exit = map.exit();
    let mut ans = usize::MAX;
    while let Some(HeapEntry(candidate, pos)) = heap.pop() {
        let candidate_key = candidate.dir * (candidate.dir_count as isize);
        let (cur_val, cur_visited) = map
            .at(pos)
            .unwrap()
            .state
            .entry(candidate_key)
            .or_insert((usize::MAX, false));
        if std::mem::replace(cur_visited, true) {
            continue;
        }

        if pos == exit {
            if *cur_val < ans && can_go(false, candidate.dir_count) {
                ans = *cur_val;
            }
            continue;
        }

        for d in directions.iter().copied() {
            let same_dir = match d.dot(candidate.dir) {
                -1 => continue,
                0 => false,
                1 => true,
                o => panic!("bad dot {d:?}, {candidate:?}, {o}"),
            };
            if !can_go(same_dir, candidate.dir_count) {
                continue;
            }
            let consider = pos + d;
            let node = if let Some(n) = map.at(consider) {
                n
            } else {
                continue;
            };
            let tentative = Candidate {
                distance: candidate.distance + node.value,
                dir: d,
                dir_count: same_dir.then_some(candidate.dir_count + 1).unwrap_or(1),
            };
            let tentative_key = tentative.dir * (tentative.dir_count as isize);
            let (best, visited) = node
                .state
                .entry(tentative_key)
                .or_insert((usize::MAX, false));
            if *visited {
                continue;
            }
            if *best < tentative.distance {
                continue;
            }
            *best = tentative.distance;
            heap.push(HeapEntry(tentative, consider));
        }
    }
    ans
}

#[test]
fn part_1() {
    let ans = run_crucible(INPUT, |same_dir, d| !same_dir || d < 3);
    println!("day 17 part 1 = {ans}");
}

#[test]
fn part_2() {
    let ans = run_crucible(INPUT, |same_dir, d| if same_dir { d < 10 } else { d >= 4 });
    println!("day 17 part 2 = {ans}");
}
