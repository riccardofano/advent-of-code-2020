use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
};

pub fn part_one(input: &str) -> Option<isize> {
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
            return Some(current_number as isize);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<isize> {
    let Some(solution) = part_one(input) else {
        panic!("Part 1 didn't have the answer");
    };

    let lines: Vec<_> = input
        .lines()
        .skip(1)
        .map(|l| l.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut left = 0;
    let mut right = 0;
    let mut current_sum: isize = lines[left];

    let mut max_range: RangeInclusive<usize> = left..=right;

    while right < lines.len() - 1 {
        match current_sum.cmp(&solution) {
            std::cmp::Ordering::Less => {
                right += 1;
                current_sum += lines[right];
            }
            std::cmp::Ordering::Greater => {
                current_sum -= lines[left];
                left += 1;
            }
            std::cmp::Ordering::Equal => {
                if right - left > max_range.end() - max_range.start() {
                    max_range = left..=right
                }
                right += 1;
                current_sum += lines[right];
            }
        }
    }

    let mut min_value = isize::MAX;
    let mut max_value = 0;

    for i in max_range {
        min_value = min_value.min(lines[i]);
        max_value = max_value.max(lines[i]);
    }

    Some(min_value + max_value)
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
        assert_eq!(part_two(&input), Some(62));
    }
}
