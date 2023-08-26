use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    let starting_numbers = input
        .trim()
        .split(',')
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut current_turn = 1;
    let mut spoken_numbers = HashMap::<usize, usize>::new();

    for number in starting_numbers {
        spoken_numbers.insert(number, current_turn);
        current_turn += 1;
    }

    // NOTE: assumes all the starting numbers are different
    let mut last_spoken = 0;
    while current_turn != 2020 {
        let next_spoken = match spoken_numbers.get(&last_spoken) {
            Some(last_turn_spoken) => current_turn - last_turn_spoken,
            None => 0,
        };
        spoken_numbers.insert(last_spoken, current_turn);
        last_spoken = next_spoken;
        current_turn += 1;
    }

    Some(last_spoken)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(436));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
