use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
    Literal(char),
    Single(usize),
    And2(usize, usize),
    And3(usize, usize, usize),
    Or(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn matches(&self, rules: &HashMap<usize, Rule>, left_to_parse: &str) -> Vec<String> {
        if left_to_parse.is_empty() {
            return Vec::new();
        }

        match self {
            Rule::Literal(char) => {
                if left_to_parse.chars().nth(0) == Some(*char) {
                    vec![left_to_parse[1..].to_string()]
                } else {
                    Vec::new()
                }
            }
            Rule::Single(a) => rules.get(a).unwrap().matches(rules, left_to_parse),
            Rule::And2(a, b) => {
                let mut result = Vec::new();
                for left in rules.get(a).unwrap().matches(rules, left_to_parse) {
                    for left2 in rules.get(b).unwrap().matches(rules, &left) {
                        result.push(left2);
                    }
                }
                result
            }
            Rule::And3(a, b, c) => {
                let mut result = Vec::new();
                for left in rules.get(a).unwrap().matches(rules, left_to_parse) {
                    for left2 in rules.get(b).unwrap().matches(rules, &left) {
                        for left3 in rules.get(c).unwrap().matches(rules, &left2) {
                            result.push(left3);
                        }
                    }
                }
                result
            }
            Rule::Or(a, b) => {
                let mut result = Vec::new();
                for left_a in a.matches(rules, left_to_parse) {
                    result.push(left_a);
                }
                for left_b in b.matches(rules, left_to_parse) {
                    result.push(left_b);
                }
                result
            }
        }
    }
}

fn parse_rules(input: &str) -> HashMap<usize, Rule> {
    input
        .lines()
        .map(|l| {
            let (index, instructions) = l.split_once(": ").unwrap();
            (index.parse().unwrap(), parse_rule(instructions))
        })
        .collect()
}

fn parse_rule(instructions: &str) -> Rule {
    if instructions.contains('\"') {
        return Rule::Literal(instructions.chars().nth(1).unwrap());
    };

    match instructions.split_once('|') {
        None => parse_and(instructions),
        Some((left, right)) => {
            let left_rule = parse_and(left);
            let right_rule = parse_and(right);
            Rule::Or(Box::new(left_rule), Box::new(right_rule))
        }
    }
}

fn parse_and(indices: &str) -> Rule {
    let indices = indices
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    match indices.len() {
        1 => Rule::Single(indices[0]),
        2 => Rule::And2(indices[0], indices[1]),
        3 => Rule::And3(indices[0], indices[1], indices[2]),
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, codes) = input.split_once("\n\n").unwrap();
    let strings_to_match = codes.lines().collect::<Vec<_>>();

    let rules = parse_rules(rules);
    let zero_rule = rules.get(&0).unwrap();

    let mut count = 0;
    for line in strings_to_match {
        for matched in zero_rule.matches(&rules, line) {
            if matched.is_empty() {
                count += 1;
                break;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(2));

        let input = include_str!("../examples/19-2.txt");
        assert_eq!(part_one(input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
