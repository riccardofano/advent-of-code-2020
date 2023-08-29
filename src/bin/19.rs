use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
enum Rule {
    Literal(String),
    And(Vec<usize>),
    Or(Box<Rule>, Box<Rule>),
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
        // NOTE: This is always a single character so it's safe to use an index to get it
        return Rule::Literal(instructions.chars().nth(1).unwrap().to_string());
    };

    // Now it's either an OR or a single AND
    match instructions.split_once('|') {
        None => Rule::And(parse_and(instructions)),
        Some((left, right)) => {
            let left_rule = Rule::And(parse_and(left));
            let right_rule = Rule::And(parse_and(right));
            Rule::Or(Box::new(left_rule), Box::new(right_rule))
        }
    }
}

fn parse_and(indices: &str) -> Vec<usize> {
    indices
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn build_regex_string(rule: &Rule, all_rules: &HashMap<usize, Rule>) -> String {
    match rule {
        Rule::Literal(s) => s.clone(),
        Rule::And(rules) => rules
            .iter()
            .map(|r| build_regex_string(all_rules.get(r).unwrap(), all_rules))
            .collect::<String>(),
        Rule::Or(left, right) => {
            let left = build_regex_string(left, all_rules);
            let right = build_regex_string(right, all_rules);
            format!("({left}|{right})")
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, codes) = input.split_once("\n\n").unwrap();
    let strings_to_match = codes.lines().collect::<Vec<_>>();

    let all_rules = parse_rules(rules);
    let zero_rule = all_rules.get(&0).unwrap();
    let regex = format!("^{}$", build_regex_string(zero_rule, &all_rules));

    let re = Regex::new(&regex).unwrap();
    let matched = strings_to_match
        .iter()
        .filter_map(|l| re.find(l))
        .collect::<Vec<_>>();

    Some(matched.len())
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
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
