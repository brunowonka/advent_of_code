use std::collections::HashMap;
use std::str::FromStr as _;

const INPUT: &str = std::include_str!("input/day12.txt");

fn check_game(
    b: &[u8],
    damaged: &[usize],
    memory: &mut HashMap<(String, Vec<usize>), Option<usize>>,
) -> Option<usize> {
    if b.is_empty() {
        if damaged.is_empty() {
            return Some(1);
        }
        return None;
    }
    let owned_b = String::from_utf8(b.to_vec()).unwrap();
    let owned_damaged = damaged.to_vec();
    let key = (owned_b, owned_damaged);
    if let Some(v) = memory.get(&key) {
        return v.clone();
    }

    let mut eval = |head_damaged: bool| -> Option<usize> {
        if head_damaged {
            let want = *damaged.first()?;
            if b.len() < want {
                return None;
            }
            if b[..want].iter().any(|b| *b == '.' as u8) {
                return None;
            }
            if b.len() == want {
                return check_game(&[], &damaged[1..], memory);
            }
            if b[want] == '#' as u8 {
                return None;
            }
            check_game(&b[want + 1..], &damaged[1..], memory)
        } else {
            check_game(&b[1..], damaged, memory)
        }
    };

    let result = match b[0] as char {
        '.' => eval(false),
        '#' => eval(true),
        '?' => {
            let a = eval(false);
            let b = eval(true);
            match (a, b) {
                (Some(a), Some(b)) => Some(a + b),
                (None, Some(x)) | (Some(x), None) => Some(x),
                (None, None) => None,
            }
        }
        o => panic!("unexpected char {o}"),
    };
    memory.insert(key, result.clone());
    result
}

#[test]
fn part_1() {
    let ans: usize = INPUT
        .lines()
        .map(|l| {
            let (map, damaged) = l.split_once(" ").unwrap();
            let damaged = damaged
                .split(",")
                .map(|v| usize::from_str(v).unwrap())
                .collect::<Vec<_>>();
            let result = check_game(map.as_bytes(), &damaged[..], &mut HashMap::new());
            result.unwrap_or(0)
        })
        .sum();
    println!("day 12 part 1 = {ans}");
}

#[test]
fn part_2() {
    let ans: usize = INPUT
        .lines()
        .map(|l| {
            let (map, damaged) = l.split_once(" ").unwrap();
            let damaged = damaged
                .split(",")
                .map(|v| usize::from_str(v).unwrap())
                .collect::<Vec<_>>();

            let damaged = std::iter::repeat(damaged.iter().copied())
                .take(5)
                .flatten()
                .collect::<Vec<_>>();

            let map =
                std::iter::repeat(itertools::Either::Left(map.as_bytes().iter().copied())).take(5);
            let map =
                itertools::intersperse(map, itertools::Either::Right(std::iter::once('?' as u8)))
                    .flatten()
                    .collect::<Vec<_>>();

            let result = check_game(&map[..], &damaged[..], &mut HashMap::new());
            result.unwrap_or(0)
        })
        .sum();
    println!("day 12 part 2 = {ans}");
}
