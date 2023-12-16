use std::fmt::Debug;

const INPUT: &str = std::include_str!("input/day16.txt");

type Vector = euclid::Vector2D<isize, ()>;

enum NodeType {
    Ground,
    Mirror(isize),
    Splitter(Vector),
}

struct Node {
    ty: NodeType,
    energized: u8,
}

impl Node {
    fn energize_with(&mut self, speed: Vector) -> (bool, u8) {
        let idx = speed.x + speed.y * 2 + 2;
        let msk = 1 << idx;
        let bef = self.energized;
        self.energized = bef | msk;
        (bef & msk != 0, bef)
    }

    fn energized(&self) -> bool {
        self.energized != 0
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.energized() {
            return write!(f, "#");
        }
        let c = match self.ty {
            NodeType::Ground => ".",
            NodeType::Mirror(i) => {
                if i > 0 {
                    "\\"
                } else {
                    "/"
                }
            }
            NodeType::Splitter(d) => {
                if d.x != 0 {
                    "-"
                } else {
                    "|"
                }
            }
        };
        write!(f, "{c}")
    }
}

impl From<NodeType> for Node {
    fn from(value: NodeType) -> Self {
        Self {
            ty: value,
            energized: Default::default(),
        }
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
                let m = map.get_or_insert_with(|| Self {
                    nodes: Vec::new(),
                    cols: l.len(),
                });

                m.nodes.extend(l.chars().map(|c| {
                    match c {
                        '.' => NodeType::Ground,
                        '/' => NodeType::Mirror(-1),
                        '\\' => NodeType::Mirror(1),
                        '-' => NodeType::Splitter(Vector::new(1, 0)),
                        '|' => NodeType::Splitter(Vector::new(0, 1)),
                        o => panic!("unknown node {o:?}"),
                    }
                    .into()
                }));

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

    fn visit(&mut self, mut pos: Vector, mut speed: Vector) -> usize {
        let mut energized = 0;
        loop {
            let node = if let Some(n) = self.at(pos) {
                n
            } else {
                return energized;
            };
            let (stop, bef) = node.energize_with(speed);
            if stop {
                break energized;
            }
            if bef == 0 {
                energized += 1;
            }

            match node.ty {
                NodeType::Ground => {}
                NodeType::Mirror(mul) => {
                    speed = speed.yx() * mul;
                }
                NodeType::Splitter(dir) => {
                    if speed.dot(dir) == 0 {
                        speed = speed.yx();
                        energized += self.visit(pos + speed, speed);
                        speed = speed * -1;
                    }
                }
            }
            pos += speed
        }
    }

    fn reset(&mut self) {
        for n in self.nodes.iter_mut() {
            n.energized = 0;
        }
    }

    fn lines(&self) -> usize {
        self.nodes.len() / self.cols
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, n) in self.nodes.iter().enumerate() {
            write!(f, "{n:?}")?;
            if (i + 1) % self.cols == 0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[test]
fn part_1() {
    let mut map = Map::new(INPUT);
    let ans = map.visit(Vector::zero(), Vector::new(1, 0));
    println!("day 16 part 1 = {ans}");
}

#[test]
fn part_2() {
    let mut map = Map::new(INPUT);

    let lines = map.lines() as isize;
    let cols = map.cols as isize;

    let lines_iter = (0..lines)
        .into_iter()
        .map(|l| {
            [
                (Vector::new(0, l), Vector::new(1, 0)),
                (Vector::new(lines - 1, l), Vector::new(-1, 0)),
            ]
            .into_iter()
        })
        .flatten();
    let cols_iter = (0..cols)
        .into_iter()
        .map(|c| {
            [
                (Vector::new(c, 0), Vector::new(0, 1)),
                (Vector::new(c, cols - 1), Vector::new(0, -1)),
            ]
            .into_iter()
        })
        .flatten();

    let ans = lines_iter
        .chain(cols_iter)
        .map(|(pos, speed)| {
            let e = map.visit(pos, speed);
            map.reset();
            e
        })
        .max()
        .unwrap();

    println!("day 16 part 2 = {ans}");
}
