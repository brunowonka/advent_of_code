use std::{fmt::Debug, num::NonZeroUsize};

const INPUT: &str = std::include_str!("input/day13.txt");

struct Map {
    values: Vec<u8>,
    cols: usize,
}

impl Map {
    fn new(l: &str) -> Self {
        Self {
            values: l.as_bytes().to_vec(),
            cols: l.len(),
        }
    }

    fn push(&mut self, l: &str) {
        assert_eq!(l.len(), self.cols);
        self.values.extend(l.as_bytes());
    }

    fn line(&self, line: usize) -> Option<&[u8]> {
        (line < self.lines()).then(|| {
            let beg = line * self.cols;
            let end = beg + self.cols;
            &self.values[beg..end]
        })
    }

    fn mirrors(
        &self,
        mirror: NonZeroUsize,
        delta: usize,
        mut budget: usize,
    ) -> Option<Option<usize>> {
        let line_a = mirror.get().checked_sub(delta + 1)?;
        let line_b = mirror.get() + delta;
        let line_a = self.line(line_a)?;
        let line_b = self.line(line_b)?;

        for (a, b) in line_a.iter().zip(line_b.iter()) {
            if *a != *b {
                if budget == 0 {
                    return Some(None);
                }
                budget = budget - 1;
            }
        }
        Some(Some(budget))
    }

    fn lines(&self) -> usize {
        self.values.len() / self.cols
    }

    fn find_mirror(&self, budget: usize) -> Option<NonZeroUsize> {
        let end = self.lines();
        (1..end).into_iter().find_map(|mirror| {
            let mirror = NonZeroUsize::new(mirror).unwrap();
            let mut delta = 0usize;
            let mut budget = budget;
            loop {
                match self.mirrors(mirror, delta, budget) {
                    Some(Some(b)) => {
                        delta = delta.saturating_add(1);
                        budget = b;
                    }
                    Some(None) => break None,
                    None => {
                        break (budget == 0).then_some(mirror);
                    }
                }
            }
        })
    }

    fn transpose(&self) -> Self {
        let mut values = Vec::new();
        let cols = self.lines();
        for i in 0..self.values.len() {
            values.push(self.values[(i * self.cols + i / cols) % self.values.len()])
        }
        Self { values, cols }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.lines() {
            let l = self.line(i).unwrap();
            let s = String::from_utf8_lossy(l);
            let i = i + 1;
            write!(f, "{i:3} {s}\n")?;
        }
        Ok(())
    }
}

fn input() -> impl Iterator<Item = Map> {
    INPUT
        .lines()
        .chain(std::iter::once(""))
        .scan(None, |map: &mut Option<Map>, l| {
            if l.is_empty() {
                return Some(Some(map.take().unwrap()));
            }
            *map = Some(match map.take() {
                Some(mut m) => {
                    m.push(l);
                    m
                }
                None => Map::new(l),
            });
            Some(None)
        })
        .filter_map(|o| o)
}

#[test]
fn part_1() {
    let ans: usize = input()
        .map(|map| {
            if let Some(m) = map.find_mirror(0) {
                return m.get() * 100;
            }
            let transposed = map.transpose();
            transposed.find_mirror(0).expect("no mirror").get()
        })
        .sum();

    println!("day 13 part 1 = {ans}");
}

#[test]
fn part_2() {
    let ans: usize = input()
        .map(|map| {
            if let Some(m) = map.find_mirror(1) {
                return m.get() * 100;
            }
            let transposed = map.transpose();
            transposed.find_mirror(1).expect("no mirror").get()
        })
        .sum();

    println!("day 13 part 2 = {ans}");
}
