use std::collections::HashSet;

const INPUT: &str = std::include_str!("input/day10.txt");

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn apply(&self, (i, j): (usize, usize)) -> Option<((usize, usize), Direction)> {
        match self {
            Direction::N => j.checked_sub(1).map(|j| ((i, j), Direction::S)),
            Direction::S => Some(((i, j + 1), Direction::N)),
            Direction::W => i.checked_sub(1).map(|i| ((i, j), Direction::E)),
            Direction::E => Some(((i + 1, j), Direction::W)),
        }
    }
}

#[derive(Debug)]
enum Node {
    Ground,
    Pipe(Direction, Direction),
    Animal,
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ground,
            'S' => Self::Animal,
            '|' => Self::Pipe(Direction::N, Direction::S),
            '-' => Self::Pipe(Direction::W, Direction::E),
            'L' => Self::Pipe(Direction::N, Direction::E),
            'J' => Self::Pipe(Direction::N, Direction::W),
            '7' => Self::Pipe(Direction::S, Direction::W),
            'F' => Self::Pipe(Direction::S, Direction::E),
            o => panic!("unknown node {o}"),
        }
    }
}

struct Map {
    nodes: Vec<Node>,
    cols: usize,
    animal: (usize, usize),
}

impl Map {
    fn new(lines: &str) -> Self {
        let (nodes, cols, animal) = lines.lines().enumerate().fold(
            (Vec::new(), None, None),
            |(mut nodes, cols, mut animal), (j, l)| {
                nodes.extend(
                    l.chars()
                        .enumerate()
                        .map(|(i, c)| (i, Node::from(c)))
                        .inspect(|(i, node)| match node {
                            Node::Animal => animal = Some((*i, j)),
                            _ => {}
                        })
                        .map(|(_, c)| c),
                );
                let cols = cols.or(Some(nodes.len()));
                (nodes, cols, animal)
            },
        );
        Self {
            nodes,
            cols: cols.unwrap(),
            animal: animal.unwrap(),
        }
    }

    fn at(&self, (i, j): (usize, usize)) -> Option<&Node> {
        self.nodes.get(j * self.cols + i)
    }

    fn at_mut(&mut self, (i, j): (usize, usize)) -> Option<&mut Node> {
        self.nodes.get_mut(j * self.cols + i)
    }

    fn iter(&self, dir: Direction) -> NodeIter<'_> {
        NodeIter {
            map: self,
            pos: self.animal,
            dir,
        }
    }
}

struct NodeIter<'a> {
    map: &'a Map,
    pos: (usize, usize),
    dir: Direction,
}

#[derive(Debug)]
enum BadPipe {
    HitGround,
    DirMismatch,
    CantMove,
    EndOfMap,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = Result<(usize, usize), BadPipe>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { map, pos, dir } = self;
        let (nxt_pos, match_dir) = if let Some(p) = dir.apply(*pos) {
            p
        } else {
            return Some(Err(BadPipe::CantMove));
        };
        let node = if let Some(n) = map.at(nxt_pos) {
            n
        } else {
            return Some(Err(BadPipe::EndOfMap));
        };

        let (a, b) = match node {
            Node::Animal => {
                return None;
            }
            Node::Ground => {
                return Some(Err(BadPipe::HitGround));
            }
            Node::Pipe(a, b) => (a, b),
        };

        let nxt_dir = if *a == match_dir {
            *b
        } else if *b == match_dir {
            *a
        } else {
            println!("{dir:?} {match_dir:?}, {node:?}");
            return Some(Err(BadPipe::DirMismatch));
        };

        *pos = nxt_pos;
        *dir = nxt_dir;
        Some(Ok(nxt_pos))
    }
}

#[test]
fn part_1() {
    let map = Map::new(INPUT);
    let ans = [Direction::N, Direction::S, Direction::W, Direction::E]
        .iter()
        .find_map(|d| {
            map.iter(*d)
                .try_fold(0, |sum, it| {
                    let _ = it?;
                    Ok::<_, BadPipe>(sum + 1)
                })
                .map(Some)
                .unwrap_or_else(|e| {
                    println!("{e:?} on {d:?}");
                    None
                })
        })
        .expect("found answer");
    let ans = (ans + 1) / 2;

    println!("day 10 part 1 = {ans}");
}

#[test]
fn part_2() {
    let mut map = Map::new(INPUT);
    let ans = [Direction::N, Direction::S, Direction::W, Direction::E]
        .iter()
        .find_map(|d| {
            let mut map_iter = map.iter(*d);
            match map_iter.by_ref().collect::<Result<HashSet<_>, _>>() {
                Ok(steps) => Some((steps, *d, map_iter.dir)),
                Err(e) => {
                    println!("{e:?} on {d:?}");
                    None
                }
            }
        })
        .expect("found answer");
    let (mut steps, da, db) = ans;
    let (_, db) = db.apply((1, 1)).unwrap();
    *map.at_mut(map.animal).unwrap() = Node::Pipe(da, db);
    // Animal was not part of the iterator.
    assert!(steps.insert(map.animal));
    let (_, ans, _) = map.nodes.iter().enumerate().fold(
        (false, 0, None),
        |(inloop, count, flipcond), (idx, node)| {
            let idx = ((idx % map.cols), (idx / map.cols));
            if idx.0 == 0 {
                assert!(!inloop, "at {idx:?}");
            }
            match node {
                Node::Ground => {
                    if inloop {
                        (inloop, count + 1, None)
                    } else {
                        (inloop, count, None)
                    }
                }
                Node::Pipe(a, b) => {
                    if steps.contains(&idx) {
                        let (flip, cond) = match (a, b) {
                            (Direction::W, Direction::E) => (false, flipcond),
                            (Direction::N, Direction::S) => (true, None),
                            (o, Direction::E) => (true, Some(o)),
                            (o, Direction::W) => (o == flipcond.expect("cond"), None),
                            o => panic!("unexpected pipe {o:?} {a:?}, {b:?} {inloop:?}"),
                        };
                        (inloop ^ flip, count, cond)
                    } else {
                        if inloop {
                            (inloop, count + 1, None)
                        } else {
                            (inloop, count, None)
                        }
                    }
                }
                o => panic!("unexpected node {o:?}"),
            }
        },
    );

    println!("day 10 part 2 = {ans}");
}
