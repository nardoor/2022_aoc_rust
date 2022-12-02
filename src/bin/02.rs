use std::cmp::Eq;

trait Scorable {
    fn to_score(&self) -> u32;
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum GameRes {
    Win,
    Loss,
    Draw,
}

impl From<&str> for GameRes {
    fn from(r: &str) -> Self {
        match r {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Failed to GameRes::from(&str) !"),
        }
    }
}

impl Scorable for GameRes {
    fn to_score(&self) -> u32 {
        match self {
            GameRes::Draw => 3,
            GameRes::Loss => 0,
            GameRes::Win => 6,
        }
    }
}

impl Move {
    fn play(&self, m: &Move) -> GameRes {
        if *self == *m {
            return GameRes::Draw;
        }
        match (self, m) {
            (&Move::Rock, &Move::Paper)
            | (&Move::Scissors, &Move::Rock)
            | (&Move::Paper, &Move::Scissors) => GameRes::Loss,
            _ => GameRes::Win,
        }
    }

    fn get_matching_move(&self, r: &GameRes) -> Self {
        match r {
            GameRes::Draw => self.clone(),
            GameRes::Loss => match self {
                Move::Paper => Move::Rock,
                Move::Rock => Move::Scissors,
                Move::Scissors => Move::Paper,
            },
            GameRes::Win => match self {
                Move::Paper => Move::Scissors,
                Move::Rock => Move::Paper,
                Move::Scissors => Move::Rock,
            },
        }
    }
}

impl From<&str> for Move {
    fn from(m: &str) -> Self {
        match m {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Failed to Move::from(&str) !"),
        }
    }
}

impl Scorable for Move {
    fn to_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.trim().split_once(' ').expect("Could not parse input"))
            .fold(0, |acc, e| {
                let my_move = Move::from(e.1);
                acc + my_move.to_score() + my_move.play(&Move::from(e.0)).to_score()
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.trim().split_once(' ').expect("Could not parse input"))
            .fold(0, |acc, e| {
                let my_res = GameRes::from(e.1);
                acc + my_res.to_score() + Move::from(e.0).get_matching_move(&my_res).to_score()
            }),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }

    #[test]
    fn test_part_two_exh() {
        // A X => ROCK SCI - LOSS => 3 + 0 = 3
        // A Y => ROCK ROCK - DRAW => 1 + 3 = 4
        // A Z => ROCK PAPER - WIN => 2 + 6 =
        // B X => PAPER ROCK - LOSS => 1 + 0
        // B Y => PAPER PAPER - DRAW => 2 + 3
        // B Z => PAPER SCI - WIN => 3 + 6
        // C X => SCI PAPER - LOSS => 2 + 0
        // C Y => SCI SCI - DRAW => 3 + 3
        // C Z => SCI ROCK - WIN => 1 + 6

        let cases = [
            ("A X", 3),
            ("A Y", 4),
            ("A Z", 8),
            ("B X", 1),
            ("B Y", 5),
            ("B Z", 9),
            ("C X", 2),
            ("C Y", 6),
            ("C Z", 7),
        ];
        for (input, res) in cases {
            println!("Test {input}->{res}; got {:?}", part_two(input));
            assert_eq!(part_two(input), Some(res));
        }
    }
}
