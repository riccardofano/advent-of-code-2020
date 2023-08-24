pub fn part_one(input: &str) -> Option<u32> {
    let mut adapters = input
        .lines()
        .map(|l| l.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Expected all the lines to be numbers");
    adapters.sort();

    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);

    let mut differences = [0; 3];
    for i in 0..adapters.len() - 1 {
        let small_adapter = adapters[i];
        let big_adapter = adapters[i + 1];
        differences[big_adapter - small_adapter - 1] += 1;
    }

    Some(differences[0] * differences[2])
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
