use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

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

    fn filter_invalid_tickets(&mut self) -> Vec<usize> {
        let mut invalid_values = Vec::new();
        let mut valid_tickets = Vec::with_capacity(self.nearby_tickets.len());

        'ticket: for ticket in self.nearby_tickets.clone() {
            for value in &ticket {
                let mut valid = false;
                for rule in self.rules.iter() {
                    if rule.range_1.contains(value) || rule.range_2.contains(value) {
                        valid = true;
                        break;
                    }
                }
                if !valid {
                    invalid_values.push(*value);
                    continue 'ticket;
                }
            }
            valid_tickets.push(ticket)
        }
        self.nearby_tickets = valid_tickets;

        invalid_values
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut info = Information::parse(input);

    let invalid_values = info.filter_invalid_tickets();

    Some(invalid_values.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut info = Information::parse(input);
    let total_fields = info.my_ticket.len();

    info.filter_invalid_tickets();
    info.nearby_tickets.push(info.my_ticket.clone());

    let mut possible_fields_per_col: Vec<Vec<&str>> = vec![Vec::new(); total_fields];

    for position in 0..total_fields {
        'rule: for rule in info.rules.iter() {
            for ticket in info.nearby_tickets.iter() {
                let ticket_value = ticket[position];
                if !rule.range_1.contains(&ticket_value) && !rule.range_2.contains(&ticket_value) {
                    continue 'rule;
                }
            }
            possible_fields_per_col[position].push(&rule.field);
        }
    }

    let mut departure_positions: HashMap<&str, usize> = HashMap::new();
    let mut taken_fields: HashSet<&str> = HashSet::with_capacity(total_fields);
    while taken_fields.len() != total_fields {
        for (position, possible_fields) in possible_fields_per_col.iter_mut().enumerate() {
            possible_fields.retain(|f| !taken_fields.contains(f));

            if possible_fields.len() == 1 {
                if possible_fields[0].starts_with("departure") {
                    departure_positions.insert(possible_fields[0], position);
                }
                taken_fields.insert(possible_fields[0]);
                possible_fields.clear()
            }
        }
    }

    Some(
        departure_positions
            .values()
            .map(|i| info.my_ticket[*i])
            .product(),
    )
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
        // This day doesn't provide the right value for the end of the puzzle
        // just the fields that I should have found
        // let input = include_str!("../examples/16-2.txt");
        // assert_eq!(part_two(input), None);
    }
}
