use regex::Regex;
use std::str::FromStr;

const INPUT: &str = std::include_str!("input/day6.txt");

#[test]
fn part_1() {
    let regex = Regex::new("[0-9]+").unwrap();
    let mut lines = INPUT.lines();
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();
    let time = regex
        .find_iter(time)
        .map(|m| i32::from_str(m.as_str()).unwrap());
    let distance = regex
        .find_iter(distance)
        .map(|m| i32::from_str(m.as_str()).unwrap());
    let ans: i32 = std::iter::zip(time, distance)
        .map(|(time, distance)| {
            // D < (T-H)* H
            // H^2 -T*H + D > 0
            // (-b +- sqrt(b^2 - 4 * a * c))/(2*a)
            // (T +- sqrt(T^2 - 4*D)/2)
            let t = f64::from(time);
            let d = f64::from(distance);
            let delta = t * t - 4f64 * d;
            assert!(delta > 0f64);
            let delta = f64::sqrt(delta);
            let left = f64::ceil((t - delta) / 2f64) as i32;
            let right = f64::floor((t + delta) / 2f64) as i32;

            let travel = |x: i32| (time - x) * x;
            let left = if travel(left) == distance {
                left + 1
            } else {
                left
            };
            let right = if travel(right) == distance {
                right - 1
            } else {
                right
            };
            let wins = right - left + 1;

            println!(
                "{time} {distance} [{left} {right} {wins}] {} {}",
                travel(left),
                travel(right)
            );
            wins
        })
        .product();
    println!("day 6 part 1 = {ans}");
}

#[test]
fn part_2() {
    let regex = Regex::new("[0-9]+").unwrap();
    let mut lines = INPUT.lines();
    let time = lines.next().unwrap().replace(" ", "");
    let distance = lines.next().unwrap().replace(" ", "");

    let time = regex
        .find(time.as_str())
        .map(|m| i64::from_str(m.as_str()).unwrap())
        .unwrap();
    let distance = regex
        .find(distance.as_str())
        .map(|m| i64::from_str(m.as_str()).unwrap())
        .unwrap();

    // D < (T-H)* H
    // H^2 -T*H + D > 0
    // (-b +- sqrt(b^2 - 4 * a * c))/(2*a)
    // (T +- sqrt(T^2 - 4*D)/2)
    let t = time as f64;
    let d = distance as f64;
    let delta = t * t - 4f64 * d;
    assert!(delta > 0f64);
    let delta = f64::sqrt(delta);
    let left = f64::ceil((t - delta) / 2f64) as i64;
    let right = f64::floor((t + delta) / 2f64) as i64;

    let travel = |x: i64| (time - x) * x;
    let left = if travel(left) == distance {
        left + 1
    } else {
        left
    };
    let right = if travel(right) == distance {
        right - 1
    } else {
        right
    };
    let wins = right - left + 1;

    println!(
        "{time} {distance} [{left} {right} {wins}] {} {}",
        travel(left),
        travel(right)
    );
    let ans = wins;

    println!("day 6 part 2 = {ans}");
}
