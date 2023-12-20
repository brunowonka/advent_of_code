use std::{collections::HashMap, str::FromStr};

use itertools::{FoldWhile, Itertools};
use std::ops::Range;

const INPUT: &str = std::include_str!("input/day19.txt");

#[derive(Debug, Clone)]
enum Ruling {
    Accept,
    Reject,
    Lookup(String),
}

impl FromStr for Ruling {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            s => Self::Lookup(s.to_string()),
        })
    }
}

#[derive(Debug)]
enum Condition {
    Gt(char, usize),
    Lt(char, usize),
}

#[derive(Debug)]
struct Rule {
    cond: Option<Condition>,
    target: Ruling,
}

fn parse_rules<'a, I: Iterator<Item = &'a str>>(lines: I) -> HashMap<String, Vec<Rule>> {
    let parts_regex = regex::Regex::new(r"([a-z]+)\{(.*)\}").unwrap();
    let rule_regex = regex::Regex::new(r"(?:([a-z])([<>])([0-9]+):)?([ARa-z]+)").unwrap();
    lines
        .take_while(|l| !l.is_empty())
        .fold(HashMap::new(), |mut map, l| {
            let captures = parts_regex.captures(l).expect("no captures");
            let key = captures.get(1).unwrap().as_str().to_string();
            let rules = captures.get(2).unwrap().as_str();
            let rules = rules
                .split(",")
                .map(|r| {
                    let captures = rule_regex
                        .captures(r)
                        .unwrap_or_else(|| panic!("{r} rule doesn't match"));
                    let target =
                        Ruling::from_str(captures.get(4).expect("no target").as_str()).unwrap();
                    let cond = match (captures.get(1), captures.get(2), captures.get(3)) {
                        (Some(c), Some(s), Some(v)) => {
                            let c = c.as_str().chars().next().unwrap();
                            let v = usize::from_str(v.as_str()).unwrap();
                            Some(match s.as_str() {
                                ">" => Condition::Gt(c, v),
                                "<" => Condition::Lt(c, v),
                                o => panic!("unknown condition {o}"),
                            })
                        }
                        (None, None, None) => None,
                        o => panic!("bad captures {o:?} for {r}"),
                    };
                    Rule { cond, target }
                })
                .collect::<Vec<_>>();
            assert!(map.insert(key, rules).is_none());
            map
        })
}

#[test]
fn part_1() {
    let mut lines = INPUT.lines();
    let rules = parse_rules(lines.by_ref());
    let ans = lines.fold(0, |sum, l| {
        let l = &l[1..l.len() - 1];
        let values = l
            .split(",")
            .map(|l| {
                let (k, v) = l.split_once("=").unwrap();
                let v = usize::from_str(v).unwrap();
                (k.chars().next().unwrap(), v)
            })
            .collect::<HashMap<_, _>>();

        let mut key = "in".to_string();
        loop {
            let rules = rules.get(&key).unwrap_or_else(|| panic!("no key {key}"));
            let target = rules
                .iter()
                .find_map(|Rule { cond, target }| {
                    cond.as_ref()
                        .map(|c| match c {
                            Condition::Gt(c, v) => {
                                values.get(c).unwrap_or_else(|| panic!("no value {c}")) > v
                            }
                            Condition::Lt(c, v) => {
                                values.get(c).unwrap_or_else(|| panic!("no value {c}")) < v
                            }
                        })
                        .unwrap_or(true)
                        .then(|| target.clone())
                })
                .unwrap_or_else(|| panic!("no ruling for {rules:?} {values:?}"));
            match target {
                Ruling::Accept => {
                    break sum + values.values().sum::<usize>();
                }
                Ruling::Reject => break sum,
                Ruling::Lookup(v) => {
                    key = v;
                }
            }
        }
    });
    println!("day 19 part 1 = {ans}");
}

fn count_combinations(
    rules: &HashMap<String, Vec<Rule>>,
    key: &str,
    ranges: HashMap<char, Range<usize>>,
) -> usize {
    if ranges.values().any(|r| r.is_empty()) {
        return 0;
    }
    let eval_rules = rules
        .get(key)
        .unwrap_or_else(|| panic!("no rules for {key}"));
    let (sum, _) = eval_rules
        .iter()
        .fold_while((0, ranges), |(sum, mut state), Rule { cond, target }| {
            if let Some(cond) = cond {
                let mut inner_state = state.clone();
                match cond {
                    Condition::Gt(c, v) => {
                        let range = state.get_mut(c).unwrap();
                        range.end = *v + 1;
                        let range = inner_state.get_mut(c).unwrap();
                        range.start = *v + 1;
                    }
                    Condition::Lt(c, v) => {
                        let range = state.get_mut(c).unwrap();
                        range.start = *v;
                        let range = inner_state.get_mut(c).unwrap();
                        range.end = *v;
                    }
                }

                let inner_sum = match target {
                    Ruling::Accept => inner_state.values().map(|r| r.len()).product::<usize>(),
                    Ruling::Reject => 0,
                    Ruling::Lookup(k) => count_combinations(rules, &k, inner_state),
                };

                FoldWhile::Continue((sum + inner_sum, state))
            } else {
                FoldWhile::Done(match target {
                    Ruling::Accept => (
                        sum + state.values().map(|r| r.len()).product::<usize>(),
                        state,
                    ),
                    Ruling::Reject => (sum, state),
                    Ruling::Lookup(k) => {
                        (sum + count_combinations(rules, &k, state.clone()), state)
                    }
                })
            }
        })
        .into_inner();
    sum
}

#[test]
fn part_2() {
    let rules = parse_rules(INPUT.lines());
    let initial_ranges = ['s', 'm', 'a', 'x']
        .into_iter()
        .map(|c| (c, 1..4001))
        .collect::<HashMap<_, _>>();
    let ans = count_combinations(&rules, "in", initial_ranges);
    println!("day 19 part 2 = {ans}");
}
