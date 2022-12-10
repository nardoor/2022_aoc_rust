use std::slice::IterMut;

/*
-> debug:
🎄 Part 1 🎄
1805 (elapsed: 2.71ms)
🎄 Part 2 🎄
385728 (elapsed: 6.67ms)

-> release + ignoring not visible trees:
🎄 Part 1 🎄
1805 (elapsed: 62.53µs)
🎄 Part 2 🎄
385728 (elapsed: 137.97µs)

-> release + not ignoring visible trees:
🎄 Part 1 🎄
1805 (elapsed: 62.29µs)
🎄 Part 2 🎄
444528 (elapsed: 302.20µs)
*/
#[derive(Debug)]
struct Tree {
    height: u8,
    juged_visible: bool,
}

#[derive(Debug)]
struct TreeLine {
    trees: Vec<Tree>,
}

#[derive(Debug)]
struct Forest {
    tree_lines: Vec<TreeLine>,
    judged_visible: Option<u32>,
}

impl From<&str> for TreeLine {
    fn from(input: &str) -> Self {
        let trees: Vec<Tree> = input
            .chars()
            .map(|c| {
                let height = c.to_digit(10).expect("Not a digit..") as u8;
                Tree {
                    height,
                    juged_visible: false,
                }
            })
            .collect();

        TreeLine { trees }
    }
}

impl From<&str> for Forest {
    fn from(input: &str) -> Self {
        let tree_lines = input.lines().map(TreeLine::from).collect();

        Forest {
            tree_lines,
            judged_visible: None,
        }
    }
}

impl Forest {
    fn get_tree(&self, x: usize, y: usize) -> &Tree {
        self.tree_lines
            .get(y)
            .expect("Failed indexing Forest")
            .trees
            .get(x)
            .expect("Failed indexing TreeLine")
    }
    fn get_row_len(&self) -> usize {
        self.tree_lines.first().expect("Empty forest").trees.len()
    }
    fn get_col_len(&self) -> usize {
        self.tree_lines.len()
    }
    fn iter_mut_line(&mut self, y: usize) -> IterMut<'_, Tree> {
        self.tree_lines
            .get_mut(y)
            .expect("Failed indexing Forest")
            .trees
            .iter_mut()
    }

    fn iter_mut_col(&mut self, x: usize) -> impl Iterator<Item = &mut Tree> + DoubleEndedIterator {
        self.tree_lines.iter_mut().map(move |tree_line| {
            tree_line
                .trees
                .get_mut(x)
                .expect("Failed indexing TreeLine")
        })
    }
    fn visible_trees(&mut self) -> u32 {
        if self.judged_visible.is_some() {
            return self.judged_visible.unwrap();
        }

        let mut count = 0;
        // by line
        for y in 0..self.get_col_len() {
            // ->
            count += count_visible_one_way(self.iter_mut_line(y));
            // <-
            count += count_visible_one_way(self.iter_mut_line(y).rev());
        }

        // by col
        for x in 0..self.get_row_len() {
            // ->
            count += count_visible_one_way(self.iter_mut_col(x));

            // <-
            count += count_visible_one_way(self.iter_mut_col(x).rev());
        }
        self.judged_visible = Some(count);
        count
    }

    fn get_best_scenery(&mut self, ignore_not_visible: bool) -> u32 {
        let mut best_scenery = 0;
        for x in 0..self.get_row_len() {
            for y in 0..self.get_col_len() {
                best_scenery = best_scenery.max(self.evaluate_scenery(x, y, ignore_not_visible));
            }
        }
        best_scenery
    }

    fn evaluate_scenery(&mut self, x: usize, y: usize, ignore_not_visible: bool) -> u32 {
        // ignoring not visible might false the result!!!
        // what if a forst is surrounded with very high trees
        // then trees in the middle might have a good scenery,
        // but they won't be considered visible by the exterior !
        // example:
        /*

        99999
        91119
        91219
        91119
        99999


        */
        if ignore_not_visible && !self.get_tree(x, y).juged_visible {
            return 0;
        }
        let height = self.get_tree(x, y).height;

        let row_len = self.get_row_len();
        let col_len = self.get_col_len();

        let mut count_left = 0;
        let mut count_top = 0;
        let mut count_right = 0;
        let mut count_down = 0;

        if x < row_len {
            for rx in x + 1..row_len {
                count_right += 1;
                if self.get_tree(rx, y).height >= height {
                    break;
                }
            }
        }

        if x > 0 {
            for lx in (0..=x - 1).rev() {
                count_left += 1;
                if self.get_tree(lx, y).height >= height {
                    break;
                }
            }
        }

        if y < col_len {
            for dy in y + 1..col_len {
                count_down += 1;
                if self.get_tree(x, dy).height >= height {
                    break;
                }
            }
        }

        if y > 0 {
            for ty in (0..=y - 1).rev() {
                count_top += 1;
                if self.get_tree(x, ty).height >= height {
                    break;
                }
            }
        }

        count_down * count_left * count_right * count_top
    }
}

fn count_visible_one_way<'a>(mut iter: impl Iterator<Item = &'a mut Tree>) -> u32 {
    // fn count_visible_one_way(&mut self) -> u32 {
    let first_tree = iter
        .next()
        .expect("ERROR: expected at least 1 elem in iter.");
    let mut max_height = first_tree.height;
    let mut count = {
        if !first_tree.juged_visible {
            first_tree.juged_visible = true;
            1
        } else {
            0
        }
    };
    for tree in iter {
        if tree.height > max_height {
            max_height = tree.height;
            if !tree.juged_visible {
                tree.juged_visible = true;
                count += 1;
            }
        }
    }

    count
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(Forest::from(input).visible_trees())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut forest = Forest::from(input);
    // forest.visible_trees();
    Some(forest.get_best_scenery(false))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
    #[test]
    fn test_best_scenry() {
        let lines = ["99999", "91119", "91219", "91119", "99999"];
        let inp = lines.join("\n");
        let mut forest = Forest::from(inp.as_str());
        forest.visible_trees();
        // 2 * 2 * 2 * 2
        /*
        ["99999"
         "91119"
         "91219"
         "91119"
         "99999"];
        */
        assert_eq!(forest.get_best_scenery(false), 16);
        // This test passes but for the same forest we get different result, the optimisation might false the result !
        assert_eq!(forest.get_best_scenery(true), 0);
    }
}
