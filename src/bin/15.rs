/*
-> Debug:
🎄 Part 1 🎄
4961647 (elapsed: 14.93s)
🎄 Part 2 🎄
12274327017867 (elapsed: 11.91s)

-> Release:
🎄 Part 1 🎄
4961647 (elapsed: 1.22s)
🎄 Part 2 🎄
12274327017867 (elapsed: 2.02s)
*/

use advent_of_code::helpers::{Point, Within};
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};
use std::{collections::BTreeSet, iter::once};

trait TuningFrequency {
    fn tuning_frequency(&self) -> u128;
}

impl TuningFrequency for Point {
    fn tuning_frequency(&self) -> u128 {
        assert!(self.x() > 0);
        assert!(self.y() > 0);
        let x: u128 = self.x().try_into().unwrap();
        let y: u128 = self.y().try_into().unwrap();

        x * 4_000_000 + y
    }
}

trait ManhattanDistance {
    fn mdist(&self, other: &Self) -> i32;
}

impl ManhattanDistance for Point {
    fn mdist(&self, other: &Self) -> i32 {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs()
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn range(&self) -> i32 {
        self.pos.mdist(&self.closest_beacon)
    }
    fn covers_point(&self, p: &Point) -> bool {
        self.pos.mdist(p) <= self.range()
    }
    fn covers_at(&self, y: i32) -> Option<BTreeSet<Point>> {
        let range = self.range();
        if !y.within(&(self.pos.y() - range), &(self.pos.y() + range)) {
            return None;
        }

        let rab = if y > self.pos.y() {
            self.pos.y() + range - y
        } else {
            y - (self.pos.y() - range)
        };

        assert!(rab >= 0);

        let base_x = self.pos.x();

        let mut cover: BTreeSet<Point> = (1..=rab)
            .flat_map(|dx| [Point(base_x + dx, y), Point(base_x - dx, y)])
            .chain(once(Point(base_x, y)))
            .collect();

        cover.remove(&self.closest_beacon);

        Some(cover)
    }

    // fn get_border<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
    fn get_border(&self) -> impl Iterator<Item = Point> + '_ {
        let out_of_range = self.range() + 1;
        (0..out_of_range).flat_map(move |dx| {
            let dy = out_of_range - dx;
            [
                Point(self.pos.x() + dx, self.pos.y() + dy),
                Point(self.pos.x() - dx, self.pos.y() + dy),
                Point(self.pos.x() + dx, self.pos.y() - dy),
                Point(self.pos.x() - dx, self.pos.y() - dy),
            ]
        })
    }

    fn parse_digit((sign, v): (Option<&str>, &str)) -> i32 {
        let mul = if sign.is_some() { -1 } else { 1 };
        v.parse::<i32>().unwrap() * mul
    }
    fn parse_line(value: &str) -> IResult<&str, Self> {
        type SignedDigits<'a> = (Option<&'a str>, &'a str);
        map(
            tuple((
                preceded(tag("Sensor at x="), tuple((opt(tag("-")), digit1))),
                preceded(tag(", y="), tuple((opt(tag("-")), digit1))),
                preceded(
                    tag(": closest beacon is at x="),
                    tuple((opt(tag("-")), digit1)),
                ),
                preceded(tag(", y="), tuple((opt(tag("-")), digit1))),
            )),
            |(x1, y1, x2, y2): (SignedDigits, SignedDigits, SignedDigits, SignedDigits)| Sensor {
                pos: Point(Self::parse_digit(x1), Self::parse_digit(y1)),
                closest_beacon: Point(Self::parse_digit(x2), Self::parse_digit(y2)),
            },
        )(value)
    }
}

impl From<&str> for Sensor {
    fn from(value: &str) -> Self {
        Self::parse_line(value).expect("Failed to parse Sensor").1
    }
}

pub fn inner_part_one(input: &str, line: usize) -> usize {
    let cover = input
        .lines()
        .map(Sensor::from)
        .filter_map(|s| s.covers_at(line as i32))
        .fold(BTreeSet::new(), |mut acc, mut bt| {
            acc.append(&mut bt);
            acc
        });
    cover.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(inner_part_one(input, 2_000_000))
}

pub fn inner_part_two(input: &str, limit: i32) -> u128 {
    let sensors: Vec<Sensor> = input.lines().map(Sensor::from).collect();
    for sensor in &sensors {
        let p = sensor.get_border().find(|p| {
            if !(p.x().within(&0, &limit) && p.y().within(&0, &limit)) {
                return false;
            }
            for sensor in &sensors {
                if sensor.covers_point(p) {
                    return false;
                }
            }
            true
        });
        if let Some(p) = p {
            return p.tuning_frequency();
        }
    }
    0
}

pub fn part_two(input: &str) -> Option<u128> {
    Some(inner_part_two(input, 4_000_000))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_1() {
        let s = Sensor {
            closest_beacon: Point(10, 5),
            pos: Point(10, 0),
        };
        assert_eq!(
            s.covers_at(4),
            Some(BTreeSet::from([Point(10, 4), Point(11, 4), Point(9, 4)]))
        )
    }

    #[test]
    fn test_sensor_2() {
        let s = Sensor {
            closest_beacon: Point(10, 5),
            pos: Point(10, 10),
        };
        assert_eq!(
            s.covers_at(6),
            Some(BTreeSet::from([Point(10, 6), Point(11, 6), Point(9, 6)]))
        )
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(inner_part_one(&input, 10), 26);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(inner_part_two(&input, 20), 56000011);
    }
}
