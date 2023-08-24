use std::collections::{HashMap, HashSet};

fn get_list_of_adapters(input: &str) -> Vec<isize> {
    let mut adapters = input
        .lines()
        .map(|l| l.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Expected all the lines to be numbers");
    adapters.sort();

    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);

    adapters
}

pub fn part_one(input: &str) -> Option<u32> {
    let adapters = get_list_of_adapters(input);

    let mut differences = [0; 3];
    for i in 0..adapters.len() - 1 {
        let small_adapter = adapters[i];
        let big_adapter = adapters[i + 1];
        differences[(big_adapter - small_adapter - 1) as usize] += 1;
    }

    Some(differences[0] * differences[2])
}

fn count_ways(
    adapters: &HashSet<isize>,
    adapter: isize,
    cache: &mut HashMap<isize, isize>,
) -> isize {
    if let Some(&value) = cache.get(&adapter) {
        return value;
    }

    let Some(current_adapter) = adapters.get(&adapter) else {
        return 0;
    };

    if *current_adapter == 0 {
        return 1;
    }

    let mut sum = 0;
    for i in 1..=3 {
        let next_to_check = current_adapter - i;
        if next_to_check >= 0 {
            let ways = count_ways(adapters, next_to_check, cache);
            cache.insert(next_to_check, ways);
            sum += ways;
        }
    }
    sum
}

pub fn part_two(input: &str) -> Option<isize> {
    let adapters = get_list_of_adapters(input);
    let goal = adapters.last().cloned().unwrap();

    let mut cache = HashMap::with_capacity(adapters.len());
    let set = adapters.into_iter().collect::<HashSet<isize>>();

    let ways = count_ways(&set, goal, &mut cache);

    Some(ways)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(35));

        let input = include_str!("../examples/10-2.txt");
        assert_eq!(part_one(input), Some(220))
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(8));

        let input = include_str!("../examples/10-2.txt");
        assert_eq!(part_two(input), Some(19208))
    }
}
