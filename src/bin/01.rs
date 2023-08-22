use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<i32> {
    let mut set: HashSet<i32> = HashSet::new();

    let numbers = input
        .lines()
        .map(str::parse::<i32>)
        .collect::<Result<Vec<i32>, _>>()
        .ok()?;

    for number in numbers {
        if let Some(complement) = set.get(&(2020 - number)) {
            return Some(number * *complement);
        } else {
            set.insert(number);
        };
    }
    None
}

pub fn part_two(input: &str) -> Option<i32> {
    let numbers = input
        .lines()
        .map(str::parse::<i32>)
        .collect::<Result<Vec<i32>, _>>()
        .ok()?;

    let mut set: HashSet<i32> = HashSet::new();
    for first_number in &numbers {
        for second_number in &numbers {
            match set.get(&(2020 - first_number - second_number)) {
                Some(complement) => return Some(first_number * second_number * complement),
                None => set.insert(*second_number),
            };
        }
    }

    None
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
        assert_eq!(part_one(&input), Some(514579));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(241861950));
    }
}
