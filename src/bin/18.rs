#![feature(iter_array_chunks)]
/*
-> Release:
🎄 Part 1 🎄
4322 (elapsed: 806.98µs)
🎄 Part 2 🎄
2516 (elapsed: 8.64ms)
*/

use std::collections::{HashSet, VecDeque};

use advent_of_code::helpers::Within;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn get_adjacent(&self) -> [Cube; 6] {
        ([(self.x - 1), (self.x + 1)])
            .into_iter()
            .map(|x| Cube {
                x,
                y: self.y,
                z: self.z,
            })
            .chain(
                [self.y - 1, self.y + 1]
                    .into_iter()
                    .map(|y| Cube {
                        x: self.x,
                        y,
                        z: self.z,
                    })
                    .chain([self.z - 1, self.z + 1].into_iter().map(|z| Cube {
                        x: self.x,
                        y: self.y,
                        z,
                    })),
            )
            .array_chunks::<6>()
            .next()
            .unwrap()
    }
}

struct LavaDrop {
    droplets: HashSet<Cube>,
}

type Bounds = ((i32, i32), (i32, i32), (i32, i32));
impl LavaDrop {
    fn get_bounds(&self) -> Bounds {
        let drplt = self.droplets.iter().next().unwrap();
        let mut res = ((drplt.x, drplt.x), (drplt.y, drplt.y), (drplt.z, drplt.z));

        self.droplets.iter().for_each(|d| {
            res.0 .0 = res.0 .0.min(d.x);
            res.0 .1 = res.0 .1.max(d.x);
            res.1 .0 = res.1 .0.min(d.y);
            res.1 .1 = res.1 .1.max(d.y);
            res.2 .0 = res.2 .0.min(d.z);
            res.2 .1 = res.2 .1.max(d.z);
        });

        res
    }

    fn is_air_stuck(&self, c: &Cube, air_stuck_cache: &mut HashSet<Cube>, bounds: &Bounds) -> bool {
        if self.droplets.contains(c) {
            return false;
        }
        if air_stuck_cache.contains(c) {
            return true;
        };
        let ((xmin, xmax), (ymin, ymax), (zmin, zmax)) = bounds;

        let mut current_cloud = HashSet::new();
        current_cloud.insert(*c);
        let mut to_visit = VecDeque::from([*c]);
        while let Some(cube) = to_visit.pop_front() {
            for n_cube in cube.get_adjacent() {
                // already visited
                if current_cloud.contains(&n_cube) {
                    continue;
                }
                // Reached out ! Air is not stuck
                if !n_cube.x.within(&xmin, &xmax)
                    || !n_cube.y.within(&ymin, &ymax)
                    || !n_cube.z.within(&zmin, &zmax)
                {
                    return false;
                }
                // Reached lava ! could still be either a stuck cloud or an external cloud
                if self.droplets.contains(&n_cube) {
                    continue;
                }
                to_visit.push_front(n_cube);
                current_cloud.insert(n_cube);
            }
        }
        // couldn't find out an exit for this cloud
        air_stuck_cache.extend(current_cloud.into_iter());
        return true;
    }

    fn count_air_facing_face(&mut self) -> usize {
        let mut air_stuck_cache = HashSet::new();
        let mut count = 0;
        let bounds: Bounds = self.get_bounds();
        for c in &self.droplets {
            for nc in c.get_adjacent() {
                if self.droplets.contains(&nc) {
                    continue;
                }
                if self.is_air_stuck(&nc, &mut air_stuck_cache, &bounds) {
                   continue; 
                }
                count += 1;
            }
    
        }   
        count
    }
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        fn parse_line(value: &str) -> IResult<&str, Cube> {
            map(separated_list1(tag(","), digit1), |coords: Vec<&str>| {
                assert_eq!(coords.len(), 3);
                Cube {
                    x: coords[0].parse().unwrap(),
                    y: coords[1].parse().unwrap(),
                    z: coords[2].parse().unwrap(),
                }
            })(value)
        }
        parse_line(value).unwrap().1
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let cubes: HashSet<Cube> = input.lines().map(Cube::from).collect();
    Some(cubes.iter().fold(0, |acc, c| {
        let mut acc = acc;
        for c in c.get_adjacent() {
            if !cubes.contains(&c) {
                acc += 1;
            }
        }
        acc
    }))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lavadrop = LavaDrop{
        droplets: input.lines().map(Cube::from).collect(),
    };
    Some(lavadrop.count_air_facing_face())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
