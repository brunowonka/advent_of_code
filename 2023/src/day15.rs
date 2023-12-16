use std::str::FromStr;

const INPUT: &str = std::include_str!("input/day15.txt");

fn hash(cur: usize, c: char) -> usize {
    if c == '\n' {
        return cur;
    }
    let c = c as usize;
    let cur = cur + c;
    let cur = cur * 17;
    cur % 256
}

#[test]
fn part_1() {
    let ans: usize = INPUT.split(",").map(|s| s.chars().fold(0, hash)).sum();

    println!("day 15 part 1 = {ans}");
}

#[test]
fn part_2() {
    let mut boxes = Vec::new();
    boxes.resize(256, Vec::<(String, usize)>::new());

    let boxes = INPUT.trim().split(",").fold(boxes, |mut boxes, s| {
        let cmd = s.find(&['-', '=']).unwrap();
        let label = &s[0..cmd];
        let what = &s[cmd..cmd + 1];
        let hash = label.chars().fold(0, hash);
        let bx = &mut boxes[hash];
        match what {
            "=" => {
                let v = &s[cmd + 1..];
                let v = usize::from_str(v).unwrap();
                match bx.iter_mut().find(|(l, _)| l == label) {
                    Some((_, old)) => {
                        *old = v;
                    }
                    None => {
                        bx.push((label.to_string(), v));
                    }
                }
            }
            "-" => {
                bx.retain(|(l, _)| l != label);
            }
            o => panic!("don't know {o}"),
        }
        boxes
    });

    let ans: usize = boxes
        .into_iter()
        .enumerate()
        .map(|(i, lenses)| {
            let i = i + 1;
            lenses
                .into_iter()
                .enumerate()
                .map(|(l, (_, v))| {
                    let l = l + 1;
                    i * l * v
                })
                .sum::<usize>()
        })
        .sum();

    println!("day 15 part 2  = {ans}");
}
