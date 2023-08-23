use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    let sum = input
        .split("\n\n")
        .map(|group| {
            let mut map: HashMap<u8, usize> = HashMap::new();

            for line in group.lines() {
                for byte in line.as_bytes() {
                    *map.entry(*byte).or_default() += 1;
                }
            }

            map.values().len()
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let sum = input
        .split("\n\n")
        .map(|group| {
            let group_lines = group.lines().collect::<Vec<_>>();
            let mut map: HashMap<u8, usize> = HashMap::new();

            for line in &group_lines {
                for byte in line.as_bytes() {
                    *map.entry(*byte).or_default() += 1;
                }
            }

            map.values().filter(|v| *v == &group_lines.len()).count()
        })
        .sum();

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(6));
    }
}
