use itertools::Itertools;

const INPUT: &str = std::include_str!("input/day11.txt");

fn input(empty_multipler: usize) -> Vec<(usize, usize)> {
    let cols = INPUT.lines().next().unwrap().len();
    // First collect all galaxies.
    let mut galaxies = INPUT
        .lines()
        .enumerate()
        .map(|(j, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(i, c)| (c == '#').then_some((i, j)))
        })
        .flatten()
        .collect::<Vec<_>>();

    // Find all rows that are empty.
    INPUT.lines().fold(0usize, |j, l| {
        if l.contains("#") {
            return j + 1;
        }
        for (_, y) in galaxies.iter_mut() {
            if *y > j {
                *y = *y + empty_multipler - 1;
            }
        }
        j + empty_multipler
    });

    // Find all columns that are empty.
    (0..cols).into_iter().fold(0usize, |i, col| {
        if INPUT
            .lines()
            .map(|l| l.chars().skip(col).next().unwrap())
            .any(|c| c == '#')
        {
            return i + 1;
        };
        for (x, _) in galaxies.iter_mut() {
            if *x > i {
                *x = *x + empty_multipler - 1;
            }
        }
        i + empty_multipler
    });

    galaxies
}

#[test]
fn part_1() {
    let galaxies = input(2);
    let ans: usize = galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((xa, ya), (xb, yb))| ya.abs_diff(yb) + xa.abs_diff(xb))
        .sum();

    println!("day 11 part 1 = {ans}");
}

#[test]
fn part_2() {
    let galaxies = input(1000000);
    let ans: usize = galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((xa, ya), (xb, yb))| ya.abs_diff(yb) + xa.abs_diff(xb))
        .sum();

    println!("day 11 part 2 = {ans}");
}
