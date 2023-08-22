fn count_trees(x_increments: usize, y_increments: usize, lines: &[&str]) -> usize {
    let mut trees_encountered = 0;
    let mut x = x_increments;
    let mut y = y_increments;

    while y < lines.len() {
        let columns = lines[y].as_bytes();
        let index = x % columns.len();

        if let b'#' = columns[index] {
            trees_encountered += 1
        }
        x += x_increments;
        y += y_increments;
    }
    trees_encountered
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<_> = input.lines().collect();
    Some(count_trees(3, 1, &lines))
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines: Vec<_> = input.lines().collect();

    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result = slopes
        .into_iter()
        .map(|(right, down)| count_trees(right, down, &lines))
        .product();

    Some(result)
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
        assert_eq!(part_two(&input), Some(336));
    }
}
