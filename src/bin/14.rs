/*
Release -> 
🎄 Part 1 🎄
625 (elapsed: 267.99ms)
🎄 Part 2 🎄
25193 (elapsed: 29.43s) // SHAME!
*/

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, multi::separated_list1,
    sequence::tuple, IResult,
};

const SAND_ORIGIN: (usize, usize) = (500, 0);

#[derive(Debug)]
struct RockLine(Vec<(usize, usize)>);

impl RockLine {
    fn covers(&self, p: (usize, usize)) -> bool {
        for points in self.0.windows(2) {
            let p1 = points.get(0).unwrap();
            let p2 = points.get(1).unwrap();
            if p.1 == p1.1 && p1.1 == p2.1 {
                let mi = p1.0.min(p2.0);
                let ma = p1.0.max(p2.0);
                if mi <= p.0 && p.0 <= ma {
                    return true;
                };
            } else if p.0 == p1.0 && p1.0 == p2.0 {
                let mi = p1.1.min(p2.1);
                let ma = p1.1.max(p2.1);
                if mi <= p.1 && p.1 <= ma {
                    return true;
                };
            }
        }
        false
    }
    fn lowest(&self) -> usize {
        self.0
            .windows(2)
            .fold(0, |acc, p| acc.max(p[0].1).max(p[1].1))
    }
}
#[derive(Debug)]
struct Cave {
    rock_lines: Vec<RockLine>,
    lowest_rock: usize,
    floor: Option<usize>,
    sand: Vec<(usize, usize)>,
}

impl Cave {
    fn set_floor(&mut self) {
        self.floor = Some(self.lowest_rock + 2);
    }
    fn parse_rock(line: &str) -> IResult<&str, RockLine> {
        map(
            separated_list1(
                tag(" -> "),
                map(
                    tuple((digit1, tag(","), digit1)),
                    |(d1, _, d2): (&str, _, &str)| (d1.parse().unwrap(), d2.parse().unwrap()),
                ),
            ),
            |v| RockLine(v),
        )(line)
    }
    fn is_free(&self, p: (usize, usize)) -> bool {
        for rl in &self.rock_lines {
            if rl.covers(p) {
                return false;
            }
        }

        for s in &self.sand {
            if *s == p {
                return false;
            }
        }

        if let Some(floor) = self.floor {
            if p.1 == floor {
                return false;
            }
        }

        return true;
    }
    fn sand_fall(&mut self) -> Option<(usize, usize)> {
        if self.sand.contains(&SAND_ORIGIN) {
            return None;
        }
        let mut new_sand = SAND_ORIGIN;
        loop {
            // fell too low
            if self.floor.is_none() && new_sand.1 > self.lowest_rock {
                return None;
            }
            // down
            else if self.is_free((new_sand.0, new_sand.1 + 1)) {
                new_sand.1 += 1;
            }
            // down left
            else if self.is_free((new_sand.0 - 1, new_sand.1 + 1)) {
                new_sand.0 -= 1;
                new_sand.1 += 1;
            }
            // down right
            else if self.is_free((new_sand.0 + 1, new_sand.1 + 1)) {
                new_sand.0 += 1;
                new_sand.1 += 1;
            }
            // stopped mooving
            else {
                break;
            }
        }
        self.sand.push(new_sand);
        Some(new_sand)
    }
}

impl From<&str> for Cave {
    fn from(value: &str) -> Self {
        let rock_lines: Vec<RockLine> = value
            .lines()
            .map(|l| Self::parse_rock(l).unwrap().1)
            .collect();
        let lowest_floor = rock_lines.iter().map(RockLine::lowest).max().unwrap();
        Cave {
            rock_lines,
            sand: Vec::new(),
            lowest_rock: lowest_floor,
            floor: None,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cave = Cave::from(input);
    while cave.sand_fall() != None {}

    Some(cave.sand.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cave = Cave::from(input);
    cave.set_floor();

    while cave.sand_fall() != None {}
    Some(cave.sand.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
