pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<_> = input.lines().collect();

    let mut trees_encountered = 0;
    let mut x = 0;

    for y in 1..lines.len() {
        x += 3;

        let columns = lines[y].as_bytes();
        let index = x % columns.len();

        if let b'#' = columns[index] {
            trees_encountered += 1
        }
    }

    Some(trees_encountered)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
