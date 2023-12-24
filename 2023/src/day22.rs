use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Range,
    str::FromStr,
};

use itertools::Itertools;
use range_ext::intersect::Intersect;

const INPUT: &str = std::include_str!("input/day22.txt");

type Vector = euclid::Vector3D<isize, ()>;

#[derive(Debug)]
struct Brick {
    a: Vector,
    b: Vector,
    below: Vec<usize>,
}

impl Brick {
    fn new(input: &str) -> Self {
        fn parse_vec(input: &str) -> Vector {
            let (a, r) = input.split_once(",").unwrap();
            let (b, c) = r.split_once(",").unwrap();
            let a = isize::from_str(a).unwrap();
            let b = isize::from_str(b).unwrap();
            let c = isize::from_str(c).unwrap();
            Vector::new(a, b, c)
        }
        let (a, b) = input.split_once("~").unwrap();
        let a = parse_vec(a);
        let b = parse_vec(b);
        Self {
            a,
            b,
            below: Vec::new(),
        }
    }

    fn x_range(&self) -> Range<isize> {
        let (a, b) = super::minmax(self.a.x, self.b.x);
        a..(b + 1)
    }
    fn y_range(&self) -> Range<isize> {
        let (a, b) = super::minmax(self.a.y, self.b.y);
        a..(b + 1)
    }
    fn zs(&self) -> (isize, isize) {
        super::minmax(self.a.z, self.b.z)
    }

    fn intersect(&self, other: &Self) -> bool {
        self.x_range().does_intersect(&other.x_range())
            && self.y_range().does_intersect(&other.y_range())
    }
}

struct Game {
    bricks: Vec<Brick>,
}

impl Game {
    fn new(input: &str) -> Self {
        let bricks = input.lines().map(Brick::new).collect();
        let mut g = Self { bricks };
        g.update_below();
        g
    }

    fn update_below(&mut self) {
        for (a, b) in (0..self.bricks.len()).tuple_combinations() {
            if !self.bricks[a].intersect(&self.bricks[b]) {
                continue;
            }
            let (a0, a1) = self.bricks[a].zs();
            let (b0, b1) = self.bricks[b].zs();
            assert_ne!(a1, b0, "{:?}, {:?}", &self.bricks[a], &self.bricks[b]);
            assert_ne!(b1, a0, "{:?}, {:?}", &self.bricks[a], &self.bricks[b]);
            if a1 > b0 {
                // A is above b.
                self.bricks[a].below.push(b);
            } else {
                assert!(b1 > a0, "{:?}, {:?}", &self.bricks[a], &self.bricks[b]);
                self.bricks[b].below.push(a);
            }
        }
    }

    fn settle(&mut self) {
        loop {
            let mut changed = 0;
            for i in 0..self.bricks.len() {
                let can_z = self.bricks[i]
                    .below
                    .iter()
                    .map(|j| self.bricks[*j].zs().1 + 1)
                    .max()
                    .unwrap_or(1);
                let b = &mut self.bricks[i];
                let delta = b.zs().0 - can_z;
                assert!(delta >= 0, "{delta}");
                b.a.z -= delta;
                b.b.z -= delta;
                if delta != 0 {
                    changed += 1;
                }
                assert!(b.a.z > 0 && b.b.z > 0, "{b:?}");
            }
            if changed == 0 {
                break;
            }
        }
    }

    /// Returns map of (supporting, supported by).
    fn supporting(&self) -> HashMap<usize, (Vec<usize>, Vec<usize>)> {
        let mut map = HashMap::<usize, (Vec<usize>, Vec<usize>)>::new();
        for (b, brick) in self.bricks.iter().enumerate() {
            let bottom = brick.zs().0;
            let mut inner = Vec::new();
            for below in brick.below.iter() {
                if self.bricks[*below].zs().1 + 1 == bottom {
                    map.entry(*below).or_default().0.push(b);
                    inner.push(*below);
                }
            }
            map.entry(b).or_default().1 = inner;
        }
        map
    }
}

#[test]
fn part_1() {
    let mut game = Game::new(INPUT);
    game.settle();
    let supporting = game.supporting();
    let ans = supporting
        .values()
        .filter(|(v, _)| {
            v.iter()
                .all(|v| supporting.get(v).as_ref().unwrap().1.len() > 1)
        })
        .count();
    println!("day 22 part 1 = {ans}");
}

#[test]
fn part_2() {
    let mut game = Game::new(INPUT);
    game.settle();
    let supporting = game.supporting();

    let ans: usize = (0..game.bricks.len())
        .map(|i| {
            let mut queue = VecDeque::new();
            let mut falling = HashSet::new();
            queue.push_back(i);
            while let Some(n) = queue.pop_front() {
                let (supporting, supported_by) = supporting.get(&n).unwrap();
                if n == i || supported_by.iter().all(|s| falling.contains(s)) {
                    if falling.insert(n) {
                        queue.extend(supporting.iter().copied());
                    }
                }
            }
            falling.len() - 1
        })
        .sum();
    println!("day 22 part 1 = {ans}");
}
