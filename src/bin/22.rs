use std::collections::{HashSet, VecDeque};

enum Winner {
    Player1,
    Player2,
}

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

    fn play_turn_recursive(
        game: usize,
        round: &mut usize,
        player_1: &mut VecDeque<usize>,
        player_2: &mut VecDeque<usize>,
        seen: &mut HashSet<(VecDeque<usize>, VecDeque<usize>)>,
    ) -> Option<Winner> {
        // println!("--- Round {round} (Game: {game}) ---");
        if seen.contains(&(player_1.clone(), player_2.clone())) {
            // If we've seen this round before player 1 is the winner
            // println!("Game already seen, winner is player 1");
            return Some(Winner::Player1);
        }
        seen.insert((player_1.clone(), player_2.clone()));
        // println!("Player 1 deck: {:?}", player_1);
        // println!("Player 2 deck: {:?}", player_2);

        let card_one = player_1.pop_front().unwrap();
        let card_two = player_2.pop_front().unwrap();
        // println!("Player 1 draws: {card_one}");
        // println!("Player 1 draws: {card_two}");

        // If the value of the card a player picked is less than the number of
        // cards in their deck start a recursive round
        if player_1.len() >= card_one && player_2.len() >= card_two {
            // println!("Going to a subgame to decide our fate...");
            let winner = Game::play_recursive(
                game + 1,
                &mut player_1.clone().iter().take(card_one).copied().collect(),
                &mut player_2.clone().iter().take(card_two).copied().collect(),
            );

            // println!("Anyways returning to game {game}...");
            match winner {
                Winner::Player1 => {
                    player_1.push_back(card_one);
                    player_1.push_back(card_two);
                    // println!("Player 1 wins round {round}");
                }
                Winner::Player2 => {
                    player_2.push_back(card_two);
                    player_2.push_back(card_one);
                    // println!("Player 2 wins round {round}");
                }
            }
            return None;
        }

        // Otherwise procede as normal
        match card_one.cmp(&card_two) {
            std::cmp::Ordering::Less => {
                player_2.push_back(card_two);
                player_2.push_back(card_one);
                // println!("Player 2 wins round {round}");
            }
            std::cmp::Ordering::Greater => {
                player_1.push_back(card_one);
                player_1.push_back(card_two);
                // println!("Player 1 wins round {round}");
            }
            std::cmp::Ordering::Equal => panic!("There shouldn't be any draws"),
        }

        *round += 1;
        None
    }

    fn play_recursive(
        game: usize,
        player_1: &mut VecDeque<usize>,
        player_2: &mut VecDeque<usize>,
    ) -> Winner {
        let mut round = 1;
        let mut seen = HashSet::new();
        while !player_1.is_empty() && !player_2.is_empty() {
            if let Some(winner) =
                Game::play_turn_recursive(game, &mut round, player_1, player_2, &mut seen)
            {
                return winner;
            }
        }

        if player_1.is_empty() {
            Winner::Player2
        } else {
            Winner::Player1
        }
    }

    fn play_to_end(&mut self) -> Winner {
        while !self.player_1.is_empty() && !self.player_2.is_empty() {
            self.play_turn();
        }

        if self.player_1.is_empty() {
            Winner::Player2
        } else {
            Winner::Player1
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
    let winner = game.play_to_end();

    match winner {
        Winner::Player1 => Some(Game::count_score(&game.player_1)),
        Winner::Player2 => Some(Game::count_score(&game.player_2)),
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut game = Game::parse(input);
    let winner = Game::play_recursive(1, &mut game.player_1, &mut game.player_2);

    match winner {
        Winner::Player1 => Some(Game::count_score(&game.player_1)),
        Winner::Player2 => Some(Game::count_score(&game.player_2)),
    }
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
        assert_eq!(part_two(&input), Some(291));
    }
}
