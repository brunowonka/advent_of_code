use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;

const INPUT: &str = std::include_str!("input/day14.txt");

fn load<'a>(lines: usize, iter: impl Iterator<Item = &'a str>) -> usize {
    let (_, a, _) = iter.fold((None, 0, lines), |(mut omem, mut sum, w), l| {
        let mem =
            omem.get_or_insert_with(|| std::iter::repeat(w).take(l.len()).collect::<Vec<_>>());
        let w = w - 1;
        for (j, c) in l.chars().enumerate() {
            match c {
                'O' => {
                    let mem = &mut mem[j];
                    sum += *mem;
                    // println!("{i} {j} v{} {sum}", *mem);
                    *mem -= 1;
                }
                '#' => {
                    mem[j] = w;
                }
                '.' => (),
                o => panic!("unknown {o:?}"),
            }
        }
        (omem, sum, w)
    });

    a
}

#[test]
fn part_1() {
    let lines = INPUT.lines().count();
    let ans = load(lines, INPUT.lines());

    println!("day 14 part 1 = {ans}");
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
struct Pos(isize, isize);

impl Pos {
    fn add(&self, Pos(ox, oy): Pos) -> Pos {
        let Pos(x, y) = self;
        Pos(*x + ox, *y + oy)
    }

    fn dot(&self, Pos(dx, dy): Pos) -> isize {
        let Pos(x, y) = self;
        *x * dx + *y * dy
    }

    fn flip(&self) -> Pos {
        let Pos(x, y) = self;
        Pos(*y, *x)
    }
}

struct Map {
    values: Vec<u8>,
    cols: usize,
    rows: usize,
}

impl Map {
    fn new(lines: &str) -> Map {
        let cols = lines.lines().next().unwrap().len();
        let values: Vec<_> = lines
            .lines()
            .map(|l| l.as_bytes().iter().copied())
            .flatten()
            .collect();
        let rows = values.len() / cols;
        Self { values, cols, rows }
    }

    fn shift_params(&self, d: Direction) -> (impl Iterator<Item = Pos>, Pos) {
        let rows = self.rows;
        let cols = self.cols;
        match d {
            Direction::North => {
                let iter = (0..rows)
                    .into_iter()
                    .map(move |y| {
                        (0..cols)
                            .into_iter()
                            .map(move |x| Pos(x as isize, y as isize))
                    })
                    .flatten();
                (
                    itertools::Either::Left(itertools::Either::Left(iter)),
                    Pos(0, 1),
                )
            }
            Direction::West => {
                let iter = (0..cols)
                    .into_iter()
                    .map(move |x| {
                        (0..rows)
                            .into_iter()
                            .map(move |y| Pos(x as isize, y as isize))
                    })
                    .flatten();
                (
                    itertools::Either::Right(itertools::Either::Left(iter)),
                    Pos(1, 0),
                )
            }
            Direction::South => {
                let iter = (0..rows)
                    .rev()
                    .into_iter()
                    .map(move |y| {
                        (0..cols)
                            .into_iter()
                            .map(move |x| Pos(x as isize, y as isize))
                    })
                    .flatten();
                (
                    itertools::Either::Left(itertools::Either::Right(iter)),
                    Pos(0, -1),
                )
            }
            Direction::East => {
                let iter = (0..cols)
                    .rev()
                    .into_iter()
                    .map(move |x| {
                        (0..rows)
                            .into_iter()
                            .map(move |y| Pos(x as isize, y as isize))
                    })
                    .flatten();
                (
                    itertools::Either::Right(itertools::Either::Right(iter)),
                    Pos(-1, 0),
                )
            }
        }
    }

    fn at(&mut self, Pos(x, y): Pos) -> &mut u8 {
        let map_idx = (y as usize) * self.cols + (x as usize);
        &mut self.values[map_idx]
    }

    fn get(&self, Pos(x, y): Pos) -> char {
        let map_idx = (y as usize) * self.cols + (x as usize);
        self.values[map_idx] as char
    }

    fn shift(&mut self, d: Direction) {
        let (iter, delta) = self.shift_params(d);
        let mut mem = Vec::new();
        mem.resize(cmp::max(self.cols, self.rows), None);
        for pos in iter {
            let c = self.at(pos);
            let mem_idx = pos.dot(delta.flip()).abs() as usize;

            match *c as char {
                '.' => {
                    let mem = &mut mem[mem_idx];
                    if mem.is_none() {
                        *mem = Some(pos);
                    }
                }
                '#' => {
                    mem[mem_idx] = None;
                }
                'O' => {
                    let mem = &mut mem[mem_idx];
                    if let Some(npos) = mem.take() {
                        *c = '.' as u8;
                        *self.at(npos) = 'O' as u8;
                        *mem = Some(npos.add(delta));
                    }
                }
                c => panic!("unknown {c:?}"),
            }
        }
    }

    fn score(&self) -> usize {
        self.shift_params(Direction::North)
            .0
            .map(|pos| {
                if self.get(pos) != 'O' {
                    return 0usize;
                }
                self.rows - (pos.1 as usize)
            })
            .sum()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            let beg = i * self.cols;
            let end = beg + self.cols;
            write!(f, "{}\n", String::from_utf8_lossy(&self.values[beg..end]))?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[test]
fn part_2() {
    let mut map = Map::new(INPUT);
    let mut mem = HashMap::<Vec<u8>, Vec<u8>>::new();
    let rounds = 1000000000;
    let mut f = None;
    for j in 0..rounds {
        match mem.entry(map.values.clone()) {
            std::collections::hash_map::Entry::Occupied(_) => {
                f = Some(j);
                break;
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                for d in [
                    Direction::North,
                    Direction::West,
                    Direction::South,
                    Direction::East,
                ] {
                    map.shift(d);
                }
                v.insert(map.values.clone());
            }
        }
    }
    // Find the loop length from where I am;
    let mut i = 0;
    let mut v = &map.values;
    let loop_len = loop {
        let nv = mem.get(v).unwrap();
        i += 1;
        if nv == &map.values {
            break i;
        }
        v = nv;
    };
    let rem = (rounds - f.unwrap()) % loop_len;
    v = &map.values;
    for _ in 0..rem {
        v = mem.get(v).unwrap();
    }
    map.values = v.clone();

    let ans = map.score();
    println!("Day 14 part 2 = {ans}");
}
