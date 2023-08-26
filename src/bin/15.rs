use std::collections::{hash_map::Entry, HashMap};

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

const LAST_TURN_PART_TWO: usize = 30_000_000;
const LOW_NUMBER_CACHE_BOUNDARY: usize = LAST_TURN_PART_TWO / 1000;

pub fn part_two(input: &str) -> Option<usize> {
    let starting_numbers = input
        .trim()
        .split(',')
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut current_turn = 1;
    let mut spoken_numbers = HashMap::<usize, usize>::new();
    let mut low_number_cache = [0; LOW_NUMBER_CACHE_BOUNDARY];

    for number in starting_numbers {
        low_number_cache[number] = current_turn;
        current_turn += 1;
    }

    let mut last_spoken = 0;
    while current_turn != LAST_TURN_PART_TWO {
        if last_spoken < LOW_NUMBER_CACHE_BOUNDARY {
            let last_turn_spoken = &mut low_number_cache[last_spoken];
            last_spoken = if *last_turn_spoken == 0 {
                0
            } else {
                current_turn - *last_turn_spoken
            };
            *last_turn_spoken = current_turn;
        } else {
            match spoken_numbers.entry(last_spoken) {
                Entry::Occupied(mut occupied) => {
                    last_spoken = current_turn - occupied.insert(current_turn);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(current_turn);
                    last_spoken = 0;
                }
            };
        };

        current_turn += 1;
    }

    Some(last_spoken)
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
        assert_eq!(part_two(&input), Some(175594));
    }
}
