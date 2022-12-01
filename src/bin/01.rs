#[derive(Clone, Debug)]
struct Backpack {
    fruits: Vec<usize>,
}

impl Backpack {
    fn total(&self) -> usize {
        return self.fruits.iter().fold(0, |acc, c| acc + c);
    }
}

struct Crew {
    elves: Vec<Backpack>,
}

impl From<&str> for Crew {
    fn from(input: &str) -> Self {
        let mut elves = vec![];
        let mut fruits: Vec<usize> = vec![];
        for line in input.lines() {
            match line.trim() {
                "" => {
                    elves.push(Backpack {
                        fruits: fruits.clone(),
                    });
                    fruits.clear();
                }
                clr => {
                    fruits.push(clr.parse::<usize>().expect("Failed to parse input line"));
                }
            }
        }
        if fruits.len() > 0 {
            elves.push(Backpack { fruits });
        }
        Crew { elves }
    }
}

impl Crew {
    fn best_backpack(&self) -> usize {
        self.elves
            .iter()
            .max_by(|b1, b2| b1.total().cmp(&b2.total()))
            .expect("Could not find max element in backpacks")
            .total()
    }

    fn best_three_backpack(&self) -> usize {
        let len = self.elves.len();
        if len < 3 {
            panic!("Can't find best three backpacks without at least three backpacks");
        }
        let mut elves_cpy = self.elves.clone();
        elves_cpy.sort_by(|b1, b2| b1.total().cmp(&b2.total()));
        elves_cpy[(len - 3)..]
            .iter()
            .fold(0, |acc, e| acc + e.total())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(Crew::from(input).best_backpack() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(Crew::from(input).best_three_backpack() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
