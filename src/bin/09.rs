use std::collections::{HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<usize> {
    // NOTE: I edited by input files so that the first line is the preamble length
    let preamble_length: usize = input.lines().next().unwrap().parse().unwrap();
    let mut queue: VecDeque<usize> = VecDeque::with_capacity(preamble_length);

    for (i, line) in input.lines().enumerate() {
        let current_number: usize = line.parse().unwrap();

        if i <= preamble_length {
            queue.push_back(current_number);
            continue;
        }

        let mut found = false;
        let mut complements: HashSet<usize> = Default::default();

        for number in &queue {
            if number > &current_number {
                continue;
            }

            if complements.contains(&(current_number - number)) {
                found = true;
                break;
            }
            complements.insert(*number);
        }

        if found {
            queue.pop_front();
            queue.push_back(current_number);
        } else {
            return Some(current_number);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(127));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
