use std::collections::VecDeque;

struct Game {
    player_1: VecDeque<usize>,
    player_2: VecDeque<usize>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let player_decks = input.trim().split("\n\n").collect::<Vec<_>>();

        Self {
            player_1: Game::parse_deck(player_decks[0]),
            player_2: Game::parse_deck(player_decks[1]),
        }
    }

    fn parse_deck(player_info: &str) -> VecDeque<usize> {
        player_info
            .lines()
            .skip(1)
            .map(str::parse)
            .collect::<Result<VecDeque<_>, _>>()
            .unwrap()
    }

    fn play_turn(&mut self) {
        let card_one = self.player_1.pop_front().unwrap();
        let card_two = self.player_2.pop_front().unwrap();

        match card_one.cmp(&card_two) {
            std::cmp::Ordering::Less => {
                self.player_2.push_back(card_two);
                self.player_2.push_back(card_one);
            }
            std::cmp::Ordering::Greater => {
                self.player_1.push_back(card_one);
                self.player_1.push_back(card_two);
            }
            std::cmp::Ordering::Equal => panic!("There shouldn't be any draws"),
        }
    }

    fn play_to_end(&mut self) {
        while !self.player_1.is_empty() && !self.player_2.is_empty() {
            self.play_turn()
        }
    }

    fn count_score(deck: &VecDeque<usize>) -> usize {
        deck.iter()
            .rev()
            .enumerate()
            .map(|(i, card)| (card * (i + 1)))
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut game = Game::parse(input);
    game.play_to_end();

    match (game.player_1.is_empty(), game.player_2.is_empty()) {
        (true, false) => Some(Game::count_score(&game.player_2)),
        (false, true) => Some(Game::count_score(&game.player_1)),
        _ => unreachable!(),
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(306));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
