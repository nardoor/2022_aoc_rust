#![feature(iter_array_chunks)]
// Above feature flag requires nightly

use std::{collections::HashSet, str::Chars, iter::Chain};

// ascii value of 'a' - 1
const OFFSET_LOWERCASE: u8 = b'a' - 1;
// ascii value of 'A' - 27
const OFFSET_UPERCASE: u8 = b'A' - 27;

type Item = char;
type Items<'a> = Chars<'a>;

trait Priority {
    fn priority(&self) -> u8;
}

impl Priority for Item {
    fn priority(&self) -> u8 {
        // ascii => 1 lenght buffer is enough
        if !self.is_ascii() {
            panic!("Non ascii Item is not supported.");
        }
        let mut buf = [0; 1];
        self.encode_utf8(&mut buf);
        if self.is_lowercase() {
            buf[0] - OFFSET_LOWERCASE
        } else {
            buf[0] - OFFSET_UPERCASE
        }
    }
}

struct Rucksack {
    comp1: String,
    comp2: String,
}

impl Rucksack {
    // part_one => 10.64ms
    // fn get_common(&self) -> HashSet<char> {
    //     self.comp1
    //         .chars()
    //         .filter(|c| self.comp2.contains([*c; 1]))
    //         .collect::<HashSet<char>>()
    // }

    // part_one => 4.15ms
    fn get_common(&self) -> HashSet<Item> {
        let seen: HashSet<Item> = self.comp1.chars().collect();
        let mut common: HashSet<Item> = HashSet::new();

        for c in self.comp2.chars() {
            if seen.contains(&c) {
                common.insert(c);
            }
        }
        common
    }

    fn get_all_items_iter(&self) -> Chain<Items, Items> {
        self.comp1.chars().chain(self.comp2.chars())
    }
}

impl From<&str> for Rucksack {
    fn from(input: &str) -> Self {
        assert!(input.len() % 2 == 0);
        let comp1 = &input[..input.len() / 2];
        let comp2 = &input[input.len() / 2..];
        Rucksack {
            comp1: String::from(comp1),
            comp2: String::from(comp2),
        }
    }
}

struct ElvesGroup {
    rucksacks: [Rucksack; 3],
}

impl From<[&str; 3]> for ElvesGroup {
    fn from(inputs: [&str; 3]) -> Self {
        ElvesGroup {
            rucksacks: inputs.map(Rucksack::from),
        }
    }
}

impl ElvesGroup {
    // total part_two ~3.30ms 
    fn get_badge(&self) -> Option<Item> {
        let r1 = &self.rucksacks[0];
        let r2 = &self.rucksacks[1];
        let r3 = &self.rucksacks[2];

        let seen: HashSet<Item> = r1.get_all_items_iter().collect();
        let mut common: HashSet<Item> = HashSet::new();

        for item in r2.get_all_items_iter()
        {
            if seen.contains(&item)
            {
                common.insert(item);
            }
        }
        return r3.get_all_items_iter().find(|item| common.contains(item));
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| {
                Rucksack::from(line)
                    .get_common()
                    .iter()
                    .map(|item| item.priority() as u32)
                    .collect::<Vec<u32>>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .array_chunks::<3>() // requires rust nightly
            .map(|group| ElvesGroup::from(group)
                .get_badge()
                .expect("Failed to get_badge() out of ElvesGroup;")
                .priority() as u32
            ).sum()
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    // from https://riptutorial.com/rust/example/4149/create-a-hashset-macro
    macro_rules! set {
        ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
            {
                let mut temp_set = HashSet::new();  // Create a mutable HashSet
                $(
                    temp_set.insert($x); // Insert each item matched into the HashSet
                )*
                temp_set // Return the populated HashSet
            }
        };
    }

    #[test]
    fn test_rucksack_get_common() {
        assert_eq!(
            Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp").get_common(),
            set!['p']
        );
        assert_eq!(
            Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").get_common(),
            set!['L']
        );
        assert_eq!(Rucksack::from("PmmdzqPrVvPwwTWBwg").get_common(), set!['P']);
        assert_eq!(
            Rucksack::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").get_common(),
            set!['v']
        );
        assert_eq!(Rucksack::from("ttgJtRGJQctTZtZT").get_common(), set!['t']);
        assert_eq!(
            Rucksack::from("CrZsJsPPZsGzwwsLwLmpwMDw").get_common(),
            set!['s']
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
