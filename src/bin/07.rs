use std::collections::HashMap;

type Bag = (String, String);
type Rules = HashMap<Bag, Vec<(usize, Bag)>>;

fn parse_rules(input: &str) -> Rules {
    let mut rules = Rules::default();

    for line in input.lines() {
        let Some((container, contained)) = line.split_once(" contain ") else {
            panic!("Malformed input, couldn't split at `contain`")
        };

        let container_bag = parse_bag_into_parts(container);
        for bag in contained.split(", ") {
            let entry = rules.entry(container_bag.clone()).or_insert(Vec::new());
            let Some((amount, bag)) = bag.split_once(' ') else {
                panic!("Malformed input, couldn't separate amount from the contained bag description");
            };
            let Ok(amount) = amount.parse::<usize>() else {
                continue;
            };
            entry.push((amount, parse_bag_into_parts(bag)))
        }
    }
    rules
}

/// This expects an input formatted like `attribute<space>color<space>(bag|bags)`
/// and returns a tuple of the attribute and the color
fn parse_bag_into_parts(bag: &str) -> Bag {
    let parts = bag.split_whitespace().collect::<Vec<_>>();
    (parts[0].to_string(), parts[1].to_string())
}

fn contains_bag(rules: &Rules, node: &Bag, needle: &Bag) -> bool {
    if node == needle {
        return true;
    }

    let Some(contained_bags) = rules.get(node) else {
        return false;
    };

    for (_amount, bag) in contained_bags {
        if contains_bag(rules, bag, needle) {
            return true;
        }
    }

    false
}

fn reverse_rules(rules: &Rules) -> Rules {
    let mut reverse: Rules = Default::default();

    for (container_bag, contained_bags) in rules.iter() {
        for bag in contained_bags.iter() {
            let entry = reverse.entry(container_bag.clone()).or_insert(Vec::new());
            entry.push(bag.clone())
        }
    }

    reverse
}

fn count_bags(rules: &Rules, current: &Bag) -> usize {
    let Some(contained) = rules.get(current) else {
        return 0;
    };

    let sum = contained
        .iter()
        .map(|(amount, bag)| amount + (amount * count_bags(rules, bag)))
        .sum::<usize>();

    sum
}

pub fn part_one(input: &str) -> Option<usize> {
    let rules = parse_rules(input);
    let bag_to_find: Bag = ("shiny".into(), "gold".into());

    let mut times_found = 0;
    for rule in rules.keys() {
        if contains_bag(&rules, rule, &bag_to_find) && rule != &bag_to_find {
            times_found += 1;
        }
    }

    Some(times_found)
}

pub fn part_two(input: &str) -> Option<usize> {
    let rules = parse_rules(input);
    let our_bag: Bag = ("shiny".into(), "gold".into());
    let rules = reverse_rules(&rules);

    Some(count_bags(&rules, &our_bag))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(32));

        let input = include_str!("../examples/07-2.txt");
        assert_eq!(part_two(input), Some(126));
    }
}
