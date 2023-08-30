use std::collections::{HashMap, HashSet};

fn parse_line(line: &str) -> (&str, Option<&str>) {
    let split = line.split_once(" (contains ");

    match split {
        Some((ingredients, allergens)) => (ingredients, Some(&allergens[..allergens.len() - 1])),
        None => (line, None),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut ingredients_seen: HashMap<&str, usize> = HashMap::new();
    let mut list_with_allergens: HashMap<&str, Vec<HashSet<&str>>> = HashMap::new();

    input
        .lines()
        .map(parse_line)
        .for_each(|(ingredients, allergens)| {
            let ingredients = ingredients.split_whitespace().collect::<HashSet<_>>();
            for ingredient in &ingredients {
                *ingredients_seen.entry(ingredient).or_insert(0) += 1;
            }

            let Some(allergens) = allergens else { return };
            for allergen in allergens.split(", ") {
                list_with_allergens
                    .entry(allergen)
                    .or_insert(Vec::new())
                    .push(ingredients.clone())
            }
        });

    let mut intersections = list_with_allergens
        .into_iter()
        .map(|(name, sets)| {
            let mut sets = sets.into_iter();
            let mut set = sets.next().unwrap();
            for line in sets {
                set = set.intersection(&line).copied().collect();
            }
            (name, set)
        })
        .collect::<HashMap<&str, HashSet<&str>>>();

    let number_of_allergens = intersections.len();
    let mut confirmed_allergens: HashMap<&str, &str> = HashMap::new();
    while confirmed_allergens.len() != number_of_allergens {
        for (name, intersection) in intersections.iter_mut() {
            for ingredient in confirmed_allergens.keys() {
                intersection.remove(ingredient);
            }

            if intersection.len() == 1 {
                confirmed_allergens.insert(intersection.iter().next().unwrap(), name);
            }
        }
    }

    dbg!(&confirmed_allergens);

    let mut count = 0;
    for (name, times_seen) in ingredients_seen {
        if !confirmed_allergens.contains_key(name) {
            count += times_seen;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
