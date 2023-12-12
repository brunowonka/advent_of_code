use std::str::FromStr;

const INPUT: &str = std::include_str!("input/day9.txt");

#[test]
fn part_1() {
    fn diffs(iter: &mut dyn Iterator<Item = i64>) -> i64 {
        let mut f = if let Some(f) = iter.next() {
            f
        } else {
            return 0;
        };
        let mut niter = iter.scan(&mut f, |s, n| {
            let nn = n - **s;
            **s = n;
            Some(nn)
        });
        let up = diffs(&mut niter);
        up + f
    }
    let ans: i64 = INPUT
        .lines()
        .map(|l| {
            let mut items = l.split(" ").map(|item| i64::from_str(item).unwrap());
            diffs(&mut items)
        })
        .sum();

    println!("day 9 part 1 = {ans}");
}

#[test]
fn part_2() {
    fn diffs(iter: &mut dyn Iterator<Item = i64>) -> i64 {
        let f = if let Some(f) = iter.next() {
            f
        } else {
            return 0;
        };
        let mut niter = iter.scan(f, |s, n| {
            let nn = n - *s;
            *s = n;
            Some(nn)
        });
        let up = diffs(&mut niter);
        f - up
    }
    let ans: i64 = INPUT
        .lines()
        .map(|l| {
            let mut items = l.split(" ").map(|item| i64::from_str(item).unwrap());
            diffs(&mut items)
        })
        .sum();

    println!("day 9 part 2 = {ans}");
}
