use std::iter::Peekable;

fn parse_number(iterator: &mut Peekable<impl Iterator<Item = u8>>) -> Option<usize> {
    let mut bytes = Vec::new();
    bytes.push(iterator.next()?);

    while iterator.peek()?.is_ascii_digit() {
        bytes.push(iterator.next()?);
    }

    std::str::from_utf8(&bytes).ok()?.parse().ok()
}

fn evaluate_operation(left: usize, operator: u8, right: usize) -> Option<usize> {
    match operator {
        b'*' => Some(left * right),
        b'+' => Some(left + right),
        _ => panic!("Unknown operator"),
    }
}

fn evaluate_literal(
    iterator: &mut Peekable<impl Iterator<Item = u8>>,
    is_advanced: bool,
) -> Option<usize> {
    match iterator.peek()? {
        b'(' => {
            iterator.next()?;
            let result = evaluate_expression(iterator, is_advanced);
            match iterator.next()? {
                b')' => result,
                _ => panic!("End wasn't a )"),
            }
        }
        _ => parse_number(iterator),
    }
}

fn evaluate_expression(
    iterator: &mut Peekable<impl Iterator<Item = u8>>,
    is_advanced: bool,
) -> Option<usize> {
    let mut value = evaluate_literal(iterator, is_advanced)?;
    while iterator.peek()? != &b')' {
        let operator = iterator.next()?;
        let right_value = if is_advanced && operator == b'*' {
            evaluate_expression(iterator, is_advanced)?
        } else {
            evaluate_literal(iterator, is_advanced)?
        };
        value = evaluate_operation(value, operator, right_value)?;
    }

    Some(value)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    for line in input.lines() {
        let mut iterator = line
            .as_bytes()
            .iter()
            .chain(&[b')'])
            .filter(|&&b| b != b' ')
            .copied()
            .peekable();

        sum += evaluate_expression(&mut iterator, false)?
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;
    for line in input.lines() {
        let mut iterator = line
            .as_bytes()
            .iter()
            .chain(&[b')'])
            .filter(|&&b| b != b' ')
            .copied()
            .peekable();

        sum += evaluate_expression(&mut iterator, true)?
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let expected_results = [71, 26, 437, 12240, 13632];
        let mut input = advent_of_code::read_file("examples", 18).to_string();

        input.push_str("\n2 * 3 + (4 * 5)");
        input.push_str("\n5 + (8 * 3 + 9 + 3 * 4 * 3)");
        input.push_str("\n5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        input.push_str("\n((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");

        assert_eq!(part_one(&input), Some(expected_results.iter().sum()))
    }

    #[test]
    fn test_part_two() {
        let expected_results = [231, 51, 46, 1445, 669060, 23340];
        let mut input = advent_of_code::read_file("examples", 18);

        input.push_str("\n1 + (2 * 3) + (4 * (5 + 6))");
        input.push_str("\n2 * 3 + (4 * 5)");
        input.push_str("\n5 + (8 * 3 + 9 + 3 * 4 * 3)");
        input.push_str("\n5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        input.push_str("\n((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(part_two(&input), Some(expected_results.iter().sum()));
    }
}
