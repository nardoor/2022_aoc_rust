/*

-> Release:
🎄 Part 1 🎄
1659 (elapsed: 52.94ms)
🎄 Part 2 🎄
2382 (elapsed: 51.61s) :/

*/
use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, preceded, tuple},
    IResult,
};

type Label = [char; 2];

#[derive(Debug)]
struct Cave {
    label: Label,
    valve_flow: u8,
    con_cave: Vec<Label>,
}

impl Cave {
    fn parse_cave(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                preceded(tag("Valve "), pair(anychar, anychar)),
                preceded(tag(" has flow rate="), digit1),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list1(tag(", "), pair(anychar, anychar)),
                ),
            )),
            |((l1, l2), flow, oth_caves): (_, &str, _)| Cave {
                label: [l1, l2],
                valve_flow: flow.parse::<u8>().unwrap(),
                con_cave: oth_caves.into_iter().map(|(l1, l2)| [l1, l2]).collect(),
            },
        )(input)
    }
}

impl From<&str> for Cave {
    fn from(line: &str) -> Self {
        Self::parse_cave(line).unwrap().1
    }
}

#[derive(Debug)]
struct CaveSystem {
    caves: HashMap<Label, Cave>,
    all_dists: HashMap<Label, HashMap<Label, usize>>,
}

impl CaveSystem {
    fn dists_bfs(&self, label: Label) -> HashMap<Label, usize> {
        let mut dists = HashMap::new();
        dists.insert(label, 0);
        let mut to_visit = VecDeque::new();
        to_visit.push_front((&label, 0));
        while let Some((l, d)) = to_visit.pop_front() {
            for neigh in &self.caves.get(l).unwrap().con_cave {
                if dists.contains_key(neigh) {
                    continue;
                }
                to_visit.push_back((neigh, d + 1));
                dists.insert(*neigh, d + 1);
            }
        }
        dists
            .into_iter()
            .filter(|(l, _c)| self.caves.get(l).unwrap().valve_flow != 0)
            .collect()
    }
    fn new(caves: impl Iterator<Item = Cave>) -> Self {
        let mut caves_map = HashMap::new();
        caves.for_each(|c| {
            caves_map.insert(c.label, c);
        });
        let mut cs = CaveSystem {
            caves: caves_map,
            all_dists: HashMap::new(),
        };

        for (&l1, _c1) in cs.caves.iter() {
            cs.all_dists.insert(l1, cs.dists_bfs(l1));
        }
        cs
    }
}

fn get_pressure(
    label: &Label,
    time: usize,
    op_valves: Vec<Label>,
    cave_system: &CaveSystem,
    elephant: Option<()>,
) -> usize {
    let mut pressure = 0;
    let mut other_caves: Vec<(&Label, &usize)> = cave_system
        .all_dists
        .get(label)
        .unwrap()
        .iter()
        .filter(|(&othr_label, &d)| {
            !op_valves.contains(&othr_label)
                && time > d + 1
        })
        .collect();
    other_caves.sort_by(|(&l1, &d1), (&l2, &d2)| {
        (time.saturating_sub(d1) * cave_system.caves.get(&l1).unwrap().valve_flow as usize).cmp(
            &(time.saturating_sub(d2) * cave_system.caves.get(&l2).unwrap().valve_flow as usize),
        )
    });
    for (other_label, dist_from_label) in other_caves {
        let new_time = time - (dist_from_label + 1);
            let mut new_op_valves = op_valves.clone();
            new_op_valves.push(*other_label);
            pressure = pressure.max(
                new_time * cave_system.caves.get(other_label).unwrap().valve_flow as usize
                    + get_pressure(other_label, new_time, new_op_valves, cave_system, elephant),
            );
    }
    if elephant.is_some() {
        pressure = pressure.max(get_pressure(&['A', 'A'], 26, op_valves, cave_system, None));
    }
    pressure
}

pub fn part_one(input: &str) -> Option<usize> {
    let cave_system = CaveSystem::new(input.lines().map(Cave::from));
    let result = get_pressure(&['A', 'A'], 30, Vec::new(), &cave_system, None);
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let cave_system = CaveSystem::new(input.lines().map(Cave::from));
    let result = get_pressure(&['A', 'A'], 26, Vec::new(), &cave_system, Some(()));
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
