use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

const INPUT: &str = std::include_str!("input/day25.txt");

fn input(input: &str) -> (HashMap<usize, Vec<usize>>, HashMap<String, usize>) {
    input.lines().fold(
        (
            HashMap::<usize, Vec<usize>>::new(),
            HashMap::<String, usize>::new(),
        ),
        |(mut map, mut names), l| {
            let (k, r) = l.split_once(":").unwrap();
            let namelen = names.len();
            let ki = *names.entry(k.to_string()).or_insert(namelen);
            for i in r.trim().split(" ") {
                assert_ne!(i, "");
                let i = i.trim();
                let namelen = names.len();
                let ii = *names.entry(i.to_string()).or_insert(namelen);
                map.entry(ki).or_default().push(ii);
                map.entry(ii).or_default().push(ki);
            }
            (map, names)
        },
    )
}

struct Graph {
    nodes: HashMap<usize, Vec<usize>>,
    edges: HashSet<(usize, usize)>,
}

impl Graph {
    fn new(i: &str) -> Self {
        let (nodes, _) = input(i);

        let edges = nodes
            .iter()
            .map(|(a, v)| v.iter().map(|b| super::minmax(*a, *b)))
            .flatten()
            .collect::<HashSet<_>>();
        Self { nodes, edges }
    }

    fn reach_node(
        &self,
        start: usize,
        end: usize,
        used_edges: &mut HashSet<(usize, usize)>,
        visited: &mut HashSet<usize>,
    ) -> bool {
        if !visited.insert(start) {
            return false;
        }
        let next = self.nodes.get(&start).unwrap();
        for e in next {
            if visited.contains(e) {
                continue;
            }
            let edge = super::minmax(start, *e);
            if !used_edges.insert(edge) {
                continue;
            }
            if *e == end {
                return true;
            }
            if self.reach_node(*e, end, used_edges, visited) {
                return true;
            }

            used_edges.remove(&edge);
        }

        return false;
    }

    fn fill_group(&self, rem: &mut HashSet<usize>, avoid: &HashSet<(usize, usize)>) -> usize {
        let mut count = 0;
        let mut queue = VecDeque::new();
        let item = *rem.iter().next().unwrap();
        queue.push_back(item);
        while let Some(i) = queue.pop_front() {
            if !rem.remove(&i) {
                continue;
            }
            count += 1;
            for e in self.nodes.get(&i).unwrap().iter() {
                let edge = super::minmax(i, *e);
                if avoid.contains(&edge) {
                    continue;
                }
                queue.push_back(*e);
            }
        }

        count
    }

    fn try_bisect(&self, avoid: &HashSet<(usize, usize)>) -> Option<usize> {
        let mut v = Vec::new();
        let mut rem = self.nodes.keys().copied().collect();
        loop {
            v.push(self.fill_group(&mut rem, avoid));
            if rem.is_empty() {
                if v.len() > 1 {
                    break Some(v.iter().copied().product());
                } else {
                    break None;
                }
            }
            if v.len() >= 2 {
                break None;
            }
        }
    }
}

/// This is horribly slow, but we basically try to reduce the number of edge combinations before we
/// check for the bisection. Takes about 2min to run.
#[test]
fn part_1() {
    let g = Graph::new(INPUT);

    let mut candidates = HashMap::new();
    for (start, end) in g.edges.iter() {
        let mut used = HashSet::new();
        used.insert((*start, *end));
        let mut i = 0;
        let e = loop {
            if g.reach_node(*start, *end, &mut used, &mut HashSet::new()) {
                if i >= 3 {
                    break None;
                }
                i += 1;
            } else {
                break (i > 0 && i < 3).then_some((*start, *end));
            }
        };
        if let Some(e) = e {
            candidates.insert(e, used);
        }
    }
    let ans = candidates
        .keys()
        .copied()
        .tuple_combinations()
        .filter_map(|(a, b, c)| {
            let ks = [a, b, c];
            ks.iter()
                .all(|i| {
                    let cs = candidates.get(i).unwrap();
                    ks.iter().all(|j| cs.contains(j))
                })
                .then_some(ks)
        })
        .filter_map(|ks| g.try_bisect(&ks.into_iter().collect()))
        .collect::<Vec<_>>();

    println!("day 25 part 1 = {ans:?} {}", ans.len());
}
