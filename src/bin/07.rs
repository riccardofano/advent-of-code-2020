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

pub fn part_one(input: &str) -> Option<u32> {
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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
