use std::collections::{BTreeMap, HashMap};

const INPUT: &str = std::include_str!("input/day20.txt");

#[derive(Debug)]
enum Op {
    FlipFlop(bool),
    Conj(BTreeMap<String, bool>),
    None,
}

#[derive(Debug)]
struct Node {
    op: Op,
    outputs: Vec<String>,
}

#[derive(Debug)]
struct Circuit {
    nodes: BTreeMap<String, Node>,
}

impl Circuit {
    fn new(input: &str) -> Self {
        let rx = regex::Regex::new(r"([&%])?([a-z]+) -> (.*)").unwrap();
        let mut nodes = input.lines().fold(BTreeMap::new(), |mut map, l| {
            let captures = rx.captures(l).unwrap();
            let op = match captures.get(1).map(|m| m.as_str()) {
                None => Op::None,
                Some("%") => Op::FlipFlop(false),
                Some("&") => Op::Conj(BTreeMap::new()),
                o => panic!("bad capture {o:?}"),
            };
            let key = captures.get(2).unwrap().as_str().to_string();
            let outputs = captures
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.to_string())
                .collect();

            assert!(map.insert(key, Node { op, outputs }).is_none());
            map
        });
        // Update the conjunction module's inputs.
        let conjunctions = nodes
            .iter()
            .map(|(src, n)| {
                n.outputs.iter().filter_map(|dst| {
                    nodes.get(dst).and_then(|n| match n.op {
                        Op::Conj(_) => Some((src.clone(), dst.clone())),
                        _ => None,
                    })
                })
            })
            .flatten()
            .collect::<Vec<_>>();
        for (src, dst) in conjunctions {
            match &mut nodes.get_mut(&dst).as_mut().unwrap().op {
                Op::Conj(inputs) => {
                    assert!(inputs.insert(src, false).is_none());
                }
                _ => panic!("should be conj"),
            }
        }
        Self { nodes }
    }

    fn run_pulses(
        &mut self,
        pulses: impl Iterator<Item = (bool, String, String)>,
    ) -> (usize, usize, Vec<(bool, String, String)>) {
        pulses.fold(
            (0, 0, Vec::new()),
            |(mut lo, mut hi, mut nxt), (pulse, source, target)| {
                if pulse {
                    hi += 1;
                } else {
                    lo += 1;
                }

                if let Some(Node { op, outputs }) = self.nodes.get_mut(&target) {
                    let snd = match op {
                        Op::None => Some(pulse),
                        Op::FlipFlop(f) => (!pulse).then(|| {
                            *f = !*f;
                            *f
                        }),
                        Op::Conj(s) => {
                            *s.get_mut(&source).expect("no source") = pulse;
                            Some(!s.values().all(|b| *b))
                        }
                    };
                    if let Some(snd) = snd {
                        nxt.extend(outputs.iter().map(|o| (snd, target.clone(), o.clone())));
                    }
                }

                (lo, hi, nxt)
            },
        )
    }

    fn state(&self) -> Vec<bool> {
        self.nodes
            .values()
            .filter_map(|Node { op, outputs: _ }| match op {
                Op::FlipFlop(f) => Some(itertools::Either::Left(std::iter::once(*f))),
                Op::Conj(s) => Some(itertools::Either::Right(s.values().copied())),
                Op::None => None,
            })
            .flatten()
            .collect()
    }
}

#[test]
fn part_1() {
    let mut circuit = Circuit::new(INPUT);
    let mut mem = HashMap::<Vec<bool>, (usize, usize, Vec<bool>)>::new();
    let mut state = circuit.state();
    let mut lo = 0;
    let mut hi = 0;
    for _ in 0..1000 {
        let mem_entry = match mem.entry(state) {
            std::collections::hash_map::Entry::Occupied(o) => {
                let (l, h, n) = o.get();
                lo += *l;
                hi += *h;
                state = n.clone();
                continue;
            }
            std::collections::hash_map::Entry::Vacant(v) => v,
        };
        let mut pulses = vec![(false, "button".to_string(), "broadcaster".to_string())];
        let mut rl = 0;
        let mut rh = 0;
        while !pulses.is_empty() {
            let (l, h, v) = circuit.run_pulses(pulses.into_iter());
            rl += l;
            rh += h;
            pulses = v;
        }
        state = circuit.state();
        mem_entry.insert((rl, rh, state.clone()));
        lo += rl;
        hi += rh;
    }

    let ans = lo * hi;
    println!("day 20 part 1 = {lo} * {hi} = {ans}");
}

#[test]
fn part_2() {
    let mut circuit = Circuit::new(INPUT);

    // There is only one thing that outputs to rx find it.
    //
    // This would not work if there was a flip-flop in front of rx or something
    // like that because we want it to be a `Conj`. A more general solution
    // would calculate the cycles for everything.
    let look = circuit
        .nodes
        .iter()
        .filter_map(|(k, n)| {
            n.outputs
                .iter()
                .find_map(|o| (o == "rx").then_some(k.to_string()))
        })
        .collect::<Vec<_>>();
    assert_eq!(look.len(), 1);
    let look = look.into_iter().next().unwrap();
    let want_sources = match &circuit.nodes.get(&look).unwrap().op {
        Op::Conj(inputs) => inputs.len(),
        _ => panic!("must be conj"),
    };
    let mut sources = HashMap::new();
    let mut presses = 0usize;
    loop {
        presses += 1;
        let mut pulses = vec![(false, "button".to_string(), "broadcaster".to_string())];
        while !pulses.is_empty() {
            let (_, _, v) = circuit.run_pulses(pulses.into_iter());
            for (pulse, src, target) in v.iter() {
                if *pulse && target == &look {
                    sources
                        .entry(src.to_string())
                        .or_insert(Vec::new())
                        .push(presses);
                }
            }
            pulses = v;
        }
        if sources.len() == want_sources {
            break;
        }
    }

    let ans = sources
        .values()
        .map(|v| v.iter().copied())
        .flatten()
        .fold(1, num::integer::lcm);

    println!("day 20 part 2 = {ans}");
}
