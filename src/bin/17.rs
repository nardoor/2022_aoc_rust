/*
-> Release:
🎄 Part 1 🎄
3085 (elapsed: 8.14ms)
🎄 Part 2 🎄
1535483870924 (elapsed: 16.85ms)
*/
use advent_of_code::btree_set;
use std::{
    collections::{hash_map::Entry, BTreeSet, HashMap},
    fmt::{Debug, Write},
};

#[derive(Debug)]
enum RockShape {
    Hori,
    Cross,
    Angle,
    Verti,
    Square,
}

impl RockShape {
    fn get_points(&self) -> Vec<(usize, usize)> {
        match self {
            RockShape::Hori => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            RockShape::Cross => vec![(1, 0), (1, 1), (1, 2), (0, 1), (2, 1)],
            RockShape::Angle => vec![(0, 2), (1, 2), (2, 2), (2, 1), (2, 0)],
            RockShape::Verti => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            RockShape::Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }

    fn get_width_height(&self) -> (usize, usize) {
        match self {
            RockShape::Hori => (4, 1),
            RockShape::Cross => (3, 3),
            RockShape::Angle => (3, 3),
            RockShape::Verti => (1, 4),
            RockShape::Square => (2, 2),
        }
    }
}

#[derive(Debug)]
enum HotJet {
    R,
    L,
}

struct NotTetris {
    grid: BTreeSet<(usize, usize)>,
    rocks: [RockShape; 5],
    hot_jets: Vec<HotJet>,
    highest_rock: (usize, usize),
}

impl NotTetris {
    fn new(hot_jet_input: &str) -> Self {
        let rocks = [
            RockShape::Hori,
            RockShape::Cross,
            RockShape::Angle,
            RockShape::Verti,
            RockShape::Square,
        ];

        let tetris = NotTetris {
            grid: btree_set![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)],
            rocks,
            hot_jets: hot_jet_input
                .trim()
                .chars()
                .map(|c| match c {
                    '<' => HotJet::L,
                    '>' => HotJet::R,
                    _ => unreachable!(),
                })
                .collect(),
            highest_rock: (0, 0),
        };

        tetris
    }
    fn get_top_row_relative_to_highest(&self) -> [usize; 7] {
        let mut res = [0; 7];
        for (x, e_res) in res.iter_mut().enumerate() {
            let base_y = self.highest_rock.1;
            let mut dy = 0;
            while !self.grid.contains(&(x, base_y - dy)) {
                dy += 1
            }
            *e_res = dy;
        }
        res
    }
    fn try_move_hori(
        &self,
        ori_rock: (usize, usize),
        (width, _height): (usize, usize),
        rock_points: &Vec<(usize, usize)>,
        next_jet: &HotJet,
    ) -> Option<(usize, usize)> {
        match next_jet {
            HotJet::L => {
                if let Some(new_x) = ori_rock.0.checked_sub(1) {
                    for p in rock_points {
                        if self.grid.get(&(new_x + p.0, ori_rock.1 - p.1)).is_some() {
                            return None;
                        }
                    }
                    return Some((new_x, ori_rock.1));
                }
            }
            HotJet::R => {
                if ori_rock.0 + 1 + (width - 1) < 7 {
                    for p in rock_points {
                        if self
                            .grid
                            .get(&(ori_rock.0 + 1 + p.0, ori_rock.1 - p.1))
                            .is_some()
                        {
                            return None;
                        }
                    }
                    return Some((ori_rock.0 + 1, ori_rock.1));
                }
            }
        };

        None
    }
    fn try_move_down(
        &self,
        ori_rock: (usize, usize),
        rock_points: &Vec<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        let new_ori = (ori_rock.0, ori_rock.1 - 1);
        for p in rock_points {
            if self.grid.get(&(new_ori.0 + p.0, new_ori.1 - p.1)).is_some() {
                return None;
            }
        }
        Some(new_ori)
    }
    fn fall_rocks(&mut self, n: usize) {
        let mut rocks_iter = self.rocks.iter().enumerate().cycle();
        let mut hot_jets_iter = self.hot_jets.iter().enumerate().cycle();
        let mut precedent = HashMap::new();
        let mut rock_just_poped;
        let mut _i = 0;
        while _i < n {
            let (rock_index, nrock) = rocks_iter.next().unwrap();
            let (width, height) = nrock.get_width_height();
            let nrock_points = nrock.get_points();
            let mut origin_nrock = (2, self.highest_rock.1 + 3 + height);
            rock_just_poped = true;
            loop {
                let (jet_index, next_jet) = hot_jets_iter.next().expect("No hot jets parsed...");
                if rock_just_poped {
                    if let Entry::Vacant(e) = precedent.entry((
                        rock_index,
                        jet_index,
                        self.get_top_row_relative_to_highest(),
                    )) {
                        e.insert((_i, self.highest_rock.1));
                    } else {
                        // period found !
                        let (oldi, old_highest) = precedent
                            .get(&(
                                rock_index,
                                jet_index,
                                self.get_top_row_relative_to_highest(),
                            ))
                            .unwrap();
                        // compute the number of period we can foresee
                        let n_periods = (n - _i) / (_i - oldi);

                        _i += n_periods * (_i - oldi);
                        let new_highest =
                            self.highest_rock.1 + (self.highest_rock.1 - old_highest) * n_periods;

                        for (x, dy) in self
                            .get_top_row_relative_to_highest()
                            .into_iter()
                            .enumerate()
                        {
                            self.grid.insert((x, new_highest - dy));
                            if dy == 0 {
                                self.highest_rock = (x, new_highest);
                            }
                        }
                    }
                    // origin might have changed
                    origin_nrock = (2, self.highest_rock.1 + 3 + height);
                    rock_just_poped = false;
                }
                if let Some(new_ori) =
                    self.try_move_hori(origin_nrock, (width, height), &nrock_points, next_jet)
                {
                    origin_nrock = new_ori;
                }
                if let Some(new_ori) = self.try_move_down(origin_nrock, &nrock_points) {
                    origin_nrock = new_ori;
                } else {
                    break;
                }
            }
            let old_highest = self.highest_rock.1;
            // landing
            for p in nrock_points {
                let grid_p = (origin_nrock.0 + p.0, origin_nrock.1 - p.1);
                if self.highest_rock.1 < grid_p.1 {
                    self.highest_rock = grid_p;
                }
                assert!(self.grid.insert(grid_p));
            }
            for y in
                (old_highest.saturating_sub(1_000))..(self.highest_rock.1.saturating_sub(1_000))
            {
                for x in 0..7 {
                    self.grid.remove(&(x, y));
                }
            }
            _i += 1;
        }
    }
}

impl Debug for NotTetris {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.highest_rock.1;
        for y in (1..=height).rev() {
            for x in 0..7 {
                f.write_char(if self.grid.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                })
                .unwrap();
            }
            f.write_char('\n').unwrap();
        }
        for _ in 0..7 {
            f.write_char('-').unwrap();
        }
        f.write_char('\n').unwrap();
        f.write_char('\n').unwrap();

        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut tetris = NotTetris::new(input);
    tetris.fall_rocks(2022);
    Some(tetris.highest_rock.1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut tetris = NotTetris::new(input);
    tetris.fall_rocks(1_000_000_000_000);
    Some(tetris.highest_rock.1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
