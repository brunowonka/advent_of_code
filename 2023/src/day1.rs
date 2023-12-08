const INPUT: &str = std::include_str!("input/day1.txt");

#[test]
fn part_1() {
    let lines = INPUT.lines();

    let ans: u32 = lines
        .map(|l| {
            let (first, last) = l.chars().fold((None, None), |state, c| {
                let c = if let Some(d) = c.to_digit(10) {
                    d
                } else {
                    return state;
                };
                match state {
                    (None, _) => (Some(c), None),
                    (prev, _) => (prev, Some(c)),
                }
            });
            let first = first.unwrap();
            let last = last.unwrap_or(first);
            first * 10 + last
        })
        .sum();
    println!("day 1 part 1 = {ans}");
}

#[test]
fn part_2() {
    let search = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    let lines = INPUT.lines();

    let ans: usize = lines
        .map(|l| {
            let (first, last) =
                search
                    .iter()
                    .enumerate()
                    .fold((None, None), |(first, last), (val, pat)| {
                        let val = val % 10;
                        let l_find = l.find(&*pat);
                        let first = match l_find {
                            Some(idx) => Some(
                                first
                                    .map(|(f_val, f_idx)| {
                                        if idx < f_idx {
                                            (val, idx)
                                        } else {
                                            (f_val, f_idx)
                                        }
                                    })
                                    .unwrap_or((val, idx)),
                            ),
                            None => first,
                        };
                        let r_find = l_find.and_then(|_| l.rfind(&*pat));
                        let last = match r_find {
                            Some(idx) => Some(
                                last.map(|(l_val, l_idx)| {
                                    if idx > l_idx {
                                        (val, idx)
                                    } else {
                                        (l_val, l_idx)
                                    }
                                })
                                .unwrap_or((val, idx)),
                            ),
                            None => last,
                        };
                        (first, last)
                    });
            let (first, _) = first.unwrap();
            let (last, _) = last.unwrap();
            let value = first * 10 + last;
            value
        })
        .sum();
    println!("day 1 part 2 = {ans}");
}
