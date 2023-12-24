#![cfg(test)]

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

type Vector = euclid::Vector2D<isize, ()>;

enum MapCoordinate {
    Vector(Vector),
    Idx(usize),
}

impl From<Vector> for MapCoordinate {
    fn from(value: Vector) -> Self {
        Self::Vector(value)
    }
}

impl From<usize> for MapCoordinate {
    fn from(value: usize) -> Self {
        Self::Idx(value)
    }
}

impl MapCoordinate {
    fn into_idx(self, conv: MapCoordinateConverter) -> Option<usize> {
        match self {
            Self::Idx(v) => Some(v),
            Self::Vector(v) => conv.to_idx(v),
        }
    }

    #[allow(dead_code)]
    fn into_vector(self, conv: MapCoordinateConverter) -> Vector {
        match self {
            MapCoordinate::Vector(v) => v,
            MapCoordinate::Idx(i) => conv.to_vector(i),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct MapCoordinateConverter {
    cols: usize,
    lines: usize,
}

impl MapCoordinateConverter {
    fn to_idx(&self, Vector { x, y, .. }: Vector) -> Option<usize> {
        let x = usize::try_from(x).ok()?;
        if x >= self.cols {
            return None;
        }
        let y = usize::try_from(y).ok()?;
        if y >= self.lines {
            return None;
        }
        Some(y * self.cols + x)
    }

    fn to_vector(&self, idx: usize) -> Vector {
        let x = isize::try_from(idx % self.cols).unwrap();
        let y = isize::try_from(idx / self.cols).unwrap();
        Vector::new(x, y)
    }

    /// Returns the wrapped vector and a vector with the number of wraps in the
    /// wrap direction.
    #[allow(dead_code)]
    fn wrap_vector(&self, v: Vector) -> (Vector, Vector) {
        fn wrap(coord: isize, max: isize) -> (isize, isize) {
            let sig = isize::signum(coord);
            let m = (sig - 1) / 2;
            let a = coord.rem_euclid(max);
            let b = coord / max + m;
            (a, b)
        }
        let (x, wx) = wrap(v.x, self.cols as isize);
        let (y, wy) = wrap(v.y, self.lines as isize);
        (Vector::new(x, y), Vector::new(wx, wy))
    }
}

#[derive(Debug)]
struct Map<T> {
    nodes: Vec<T>,
    cols: usize,
}

impl<T> Map<T> {
    fn new(nodes: Vec<T>, cols: usize) -> Self {
        Self { nodes, cols }
    }

    fn converter(&self) -> MapCoordinateConverter {
        MapCoordinateConverter {
            cols: self.cols,
            lines: self.lines(),
        }
    }

    fn at<C: Into<MapCoordinate>>(&self, t: C) -> Option<&T> {
        t.into()
            .into_idx(self.converter())
            .and_then(|i| self.nodes.get(i))
    }

    fn at_mut<C: Into<MapCoordinate>>(&mut self, t: C) -> Option<&mut T> {
        t.into()
            .into_idx(self.converter())
            .and_then(|i| self.nodes.get_mut(i))
    }

    fn lines(&self) -> usize {
        self.nodes.len() / self.cols
    }

    fn bounds(&self) -> Vector {
        Vector::new(self.cols as isize, self.lines() as isize)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> impl Iterator<Item = Direction> {
        [Self::Up, Self::Down, Self::Left, Self::Right].into_iter()
    }

    fn to_vector(&self) -> Vector {
        match self {
            Direction::Up => Vector::new(0, -1),
            Direction::Down => Vector::new(0, 1),
            Direction::Left => Vector::new(-1, 0),
            Direction::Right => Vector::new(1, 0),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn minmax<T: std::cmp::Ord>(a: T, b: T) -> (T, T) {
    match a.cmp(&b) {
        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => (a, b),
        std::cmp::Ordering::Greater => (b, a),
    }
}
