use std::str::FromStr;

const INPUT: &str = std::include_str!("input/day18.txt");

type Vector = euclid::Vector2D<isize, ()>;

fn area(iter: impl Iterator<Item = Vector>) -> isize {
    let (_, inner, perimeter) = iter.fold((Vector::zero(), 0, 0), |(p0, inner, perimeter), v| {
        // Direction is never diagonal just add the components for perimeter.
        let perimeter = perimeter + isize::abs(v.x + v.y);
        let p1 = p0 + v;
        let inner = inner + p0.x * p1.y - p1.x * p0.y;
        (p1, inner, perimeter)
    });
    let inner = inner / 2;
    isize::abs(inner + perimeter / 2 + 1)
}

#[test]
fn part_1() {
    let iter = INPUT.lines().map(|l| {
        let mut l = l.split(" ");
        let dir = match l.next().unwrap() {
            "R" => Vector::new(1, 0),
            "L" => Vector::new(-1, 0),
            "U" => Vector::new(0, -1),
            "D" => Vector::new(0, 1),
            o => panic!("unknown dir {o}"),
        };
        let sz = isize::from_str(l.next().unwrap()).expect("bad number");
        dir * sz
    });
    let ans = area(iter);
    println!("day 18 part 1 = {ans}");
}

#[test]
fn part_2() {
    let iter = INPUT.lines().map(|l| {
        let l = l.split(" ").last().unwrap().as_bytes();
        assert_eq!(l.len(), 9);
        let hex = &l[2..7];
        let dir = match l[7] as char {
            '0' => Vector::new(1, 0),
            '2' => Vector::new(-1, 0),
            '3' => Vector::new(0, -1),
            '1' => Vector::new(0, 1),
            o => panic!("unknown dir {o}"),
        };
        let sz = usize::from_str_radix(String::from_utf8_lossy(hex).as_ref(), 16).unwrap();
        dir * (sz as isize)
    });
    let ans = area(iter);
    println!("day 18 part 2 = {ans}");
}
