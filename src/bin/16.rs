use std::ops::RangeInclusive;

#[derive(Debug)]
struct Rule {
    field: String,
    range_1: RangeInclusive<usize>,
    range_2: RangeInclusive<usize>,
}

impl Rule {
    fn parse(input: &str) -> Self {
        let (field, ranges) = input
            .split_once(": ")
            .expect("Failed to find `: ` field delimiter");

        let (range_1, range_2) = ranges
            .split_once(" or ")
            .expect("Failed to find the or separating the ranges");

        Self {
            field: field.to_string(),
            range_1: Rule::split_range(range_1),
            range_2: Rule::split_range(range_2),
        }
    }

    fn split_range(range: &str) -> RangeInclusive<usize> {
        let (min, max) = range
            .split_once('-')
            .expect("Failed to find dash separating range");

        min.parse().unwrap()..=max.parse().unwrap()
    }
}

type Ticket = Vec<usize>;

#[derive(Debug)]
struct Information {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Information {
    fn parse(input: &str) -> Self {
        let parts = input.split("\n\n").collect::<Vec<_>>();

        let rules = parts[0].lines().map(Rule::parse).collect::<Vec<_>>();
        let my_ticket = parts[1]
            .lines()
            .skip(1)
            .collect::<String>()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let nearby_tickets = parts[2]
            .lines()
            .skip(1)
            .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
            .collect();

        Self {
            rules,
            my_ticket,
            nearby_tickets,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let info = Information::parse(input);
    let mut invalid_values = Vec::new();

    for ticket in info.nearby_tickets {
        for value in ticket {
            let mut valid = false;
            for rule in info.rules.iter() {
                if rule.range_1.contains(&value) || rule.range_2.contains(&value) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                invalid_values.push(value);
            }
        }
    }

    Some(invalid_values.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(71));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
