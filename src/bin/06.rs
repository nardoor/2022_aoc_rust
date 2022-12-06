#![feature(iter_array_chunks)]

/*
(debug binary)
🎄 Part 1 🎄
1647 (elapsed: 320.94µs)
🎄 Part 2 🎄
2447 (elapsed: 4.63ms)

(release binary)
🎄 Part 1 🎄
1647 (elapsed: 13.67µs)
🎄 Part 2 🎄
2447 (elapsed: 251.14µs)
*/
trait Unicity {
    fn all_unique(&self) -> bool;
}

impl Unicity for [char; 3] {
    fn all_unique(&self) -> bool {
        self[0] != self[1] && self[1] != self[2] && self[2] != self[0]
    }
}

#[derive(Debug)]
struct Similaritude {
    pub i1: usize,
    pub i2: usize,
}

struct UnicityTracker<T, const N: usize>
where
    T: PartialEq,
{
    sims: Option<Vec<Similaritude>>,
    data: [T; N],
}

impl<T: PartialEq, const N: usize> UnicityTracker<T, N> {
    fn new(data: [T; N]) -> Self {
        let mut sims = vec![];
        for (i1, e1) in data.iter().enumerate() {
            for (i2, e2) in data.iter().enumerate() {
                if *e1 == *e2 && i1 != i2 {
                    sims.push(Similaritude { i1, i2 })
                }
            }
        }
        UnicityTracker {
            data,
            sims: Some(sims),
        }
    }

    fn get_data(&self) -> &[T; N] {
        &self.data
    }

    fn set(&mut self, i: usize, e: T) {
        let mut next_sims = vec![];

        if e == self.data[i] {
            return;
        }

        // add new similaritudes
        for (i1, e1) in self.data.iter().enumerate() {
            if e == *e1 && i1 != i {
                next_sims.push(Similaritude { i1, i2: i });
            }
        }

        // remove sims that don't need to be there
        let sims = self.sims.take();
        self.sims = None;

        self.sims = Some(
            sims.unwrap()
                .into_iter()
                .filter(|s| {
                    if s.i1 == i && self.data[s.i2] != e {
                        return false;
                    }

                    if s.i2 == i && self.data[s.i1] != e {
                        return false;
                    }

                    true
                })
                .collect(),
        );

        self.data[i] = e;
        self.sims.as_mut().unwrap().append(&mut next_sims);
    }
}

impl<T: PartialEq, const N: usize> Unicity for UnicityTracker<T, N> {
    // Hypothesis -> all similaritudes are noted
    fn all_unique(&self) -> bool {
        self.sims.as_ref().unwrap().is_empty()
    }
}

fn start_marker_count(line: &str) -> Option<u32> {
    assert!(line.len() > 3);
    let mut old: [char; 3] = [
        line.chars().next().unwrap(),
        line.chars().nth(1).unwrap(),
        line.chars().nth(2).unwrap(),
    ];
    let mut i = 0;

    for (count, c) in line.chars().enumerate().skip(3) {
        if !old.contains(&c) && old.all_unique() {
            return Some((count + 1) as u32); // + 1 => 1-indexed
        } else {
            old[i] = c;
            i = (i + 1) % 3;
        }
    }
    None
}

fn message_marker_count(line: &str) -> Option<u32> {
    let data = line.chars().array_chunks::<13>().next().unwrap();
    let mut ut = UnicityTracker::new(data);
    let mut i = 0;
    for (count, c) in line.chars().enumerate().skip(13) {
        if !ut.get_data().contains(&c) && ut.all_unique() {
            return Some((count + 1) as u32);
        } else {
            ut.set(i, c);
            i = (i + 1) % 13;
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    start_marker_count(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    message_marker_count(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }

    #[test]
    fn test_start_marker_count() {
        for (marker, count) in [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ] {
            assert_eq!(start_marker_count(&marker), Some(count));
        }
    }

    #[test]
    fn test_message_marker_count() {
        for (marker, count) in [
            ("abcdefghijklmnopqrstuvwxyz", 14),
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ] {
            assert_eq!(message_marker_count(&marker), Some(count));
        }
    }
}
