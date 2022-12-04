use std::array::IntoIter;

#[derive(Clone, Debug)]
struct Backpack {
    fruits: Vec<usize>,
}
// hypothesis is the array is always sorted -> ascending order
struct SortedArray<T, const N: usize>
// Ideally Copy trait wouldn't be necessary
where
    T: Ord + Copy,
{
    v: [T; N],
}

impl<T: Ord + Copy, const N: usize> SortedArray<T, N> {
    /// Insert `new_el` in the SortedArray if it fits there.  

    /// If the `new_el` is lower than all the SortedArray,
    /// the SortedArray will remain unchanged.
    /// ```
    /// let mut sa = SortedArray::from([2, 4, 7, 7]);
    /// sa.insert(8);
    /// assert_eq!(sa.v, [4, 7, 7, 8]);
    ///     
    /// sa.insert(5);
    /// assert_eq!(sa.v, [5, 7, 7, 8]);
    /// ```
    pub fn insert(&mut self, new_el: T) {
        let mut iter = self.v.iter_mut().peekable();
        while let Some(cur_el) = iter.next() {
            if new_el < *cur_el {
                return;
            }

            if new_el > *cur_el {
                match iter.peek_mut() {
                    None => {
                        // cur_el == last_el; cur_el < new_el
                        *cur_el = new_el;
                        return;
                    }
                    Some(next_el) => {
                        // cur_el < new_el < next_el
                        if new_el < **next_el {
                            *cur_el = new_el;
                            return;
                        } else {
                            // This is where copy trait is necessary
                            *cur_el = **next_el;
                        }
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> IntoIter<T, N> {
        self.v.into_iter()
    }
}

impl<T: Ord + Copy, const N: usize> From<[T; N]> for SortedArray<T, N> {
    fn from(mut v: [T; N]) -> Self {
        v.sort();
        SortedArray { v }
    }
}

impl Backpack {
    fn total(&self) -> usize {
        return self.fruits.iter().sum();
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
        if !fruits.is_empty() {
            elves.push(Backpack { fruits });
        }
        Crew { elves }
    }
}

impl Crew {
    // part_one => ~ 1ms
    // fn best_backpack(&self) -> usize {
    //     self.elves
    //         .iter()
    //         .max_by(|b1, b2| b1.total().cmp(&b2.total()))
    //         .expect("Could not find max element in backpacks")
    //         .total()
    // }

    // Suboptimal -> sorting whole array unecessary
    // part_two => ~2ms
    // fn best_three_backpack(&self) -> usize {
    //     let len = self.elves.len();
    //     if len < 3 {
    //         panic!("Can't find best three backpacks without at least three backpacks");
    //     }
    //     let mut elves_cpy = self.elves.clone();
    //     elves_cpy.sort_by_key(|b| b.total());
    //     elves_cpy[(len - 3)..]
    //         .iter()
    //         .fold(0, |acc, e| acc + e.total())
    // }

    // part_one => ~900µs
    // part_two => ~900µs
    fn best_n_backpacks<const N: usize>(&self) -> usize {
        let mut sorted_array = SortedArray::from([0; N]);
        self.elves
            .iter()
            .for_each(|el| sorted_array.insert(el.total()));
        sorted_array.iter().sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(Crew::from(input).best_n_backpacks::<1>() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(Crew::from(input).best_n_backpacks::<3>() as u32)
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

    #[test]
    fn test_sorted_array_same_elems() {
        let mut sa = SortedArray::from([3; 5]);
        sa.insert(4);
        assert_eq!(sa.v, [3, 3, 3, 3, 4]);
    }

    #[test]
    fn test_sorted_array_replace_first() {
        let mut sa = SortedArray::from([2, 4, 5]);
        sa.insert(3);
        assert_eq!(sa.v, [3, 4, 5]);
    }

    #[test]
    fn test_sorted_array_replace_middle() {
        let mut sa = SortedArray::from([2, 3, 5]);
        sa.insert(4);
        assert_eq!(sa.v, [3, 4, 5]);
    }

    #[test]
    fn test_sorted_array_replace_last() {
        let mut sa = SortedArray::from([2, 3, 5, 5]);
        sa.insert(6);
        assert_eq!(sa.v, [3, 5, 5, 6]);
    }

    #[test]
    fn test_sorted_array_doc_test() {
        let mut sa = SortedArray::from([2, 4, 7, 7]);
        sa.insert(8);
        assert_eq!(sa.v, [4, 7, 7, 8]);

        sa.insert(5);
        assert_eq!(sa.v, [5, 7, 7, 8]);
    }
}
