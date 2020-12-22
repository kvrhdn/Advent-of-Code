use aoc_runner_derive::aoc;
use std::collections::{HashSet, VecDeque};

type Deck = VecDeque<u32>;

enum Player {
    Player1,
    Player2,
}

fn play_game(mut player_1: Deck, mut player_2: Deck, with_recursion: bool) -> (Player, u32) {
    let mut previous_rounds = HashSet::new();

    let winner = loop {
        if with_recursion {
            // check if this configuration has been seen before
            if previous_rounds.contains(&player_1) || previous_rounds.contains(&player_2) {
                break Player::Player1;
            }
            previous_rounds.insert(player_1.clone());
            previous_rounds.insert(player_2.clone());
        }

        let p1 = player_1.pop_front().unwrap();
        let p2 = player_2.pop_front().unwrap();

        let winner = if with_recursion && p1 <= player_1.len() as u32 && p2 <= player_2.len() as u32
        {
            let mut new_player_1 = player_1.clone();
            new_player_1.truncate(p1 as usize);
            let mut new_player_2 = player_2.clone();
            new_player_2.truncate(p2 as usize);

            play_game(new_player_1, new_player_2, true).0
        } else {
            if p1 > p2 {
                Player::Player1
            } else {
                Player::Player2
            }
        };

        match winner {
            Player::Player1 => {
                player_1.push_back(p1);
                player_1.push_back(p2);
            }
            Player::Player2 => {
                player_2.push_back(p2);
                player_2.push_back(p1);
            }
        }

        if player_1.is_empty() {
            break Player::Player2;
        }
        if player_2.is_empty() {
            break Player::Player1;
        }
    };

    let winning_deck = match winner {
        Player::Player1 => player_1,
        Player::Player2 => player_2,
    };

    (winner, score_of_deck(&winning_deck))
}

fn score_of_deck(deck: &Deck) -> u32 {
    deck.iter()
        .enumerate()
        .map(|(i, &value)| value * (deck.len() - i) as u32)
        .sum()
}

fn parse_input(input: &str) -> (Deck, Deck) {
    let (player_1, player_2) = input.split_once("\n\n").unwrap();

    let parse_deck = |input: &str| -> Deck {
        input
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect()
    };

    (parse_deck(player_1), parse_deck(player_2))
}

#[aoc(day22, part1)]
fn solve_part1(input: &str) -> u32 {
    let (player_1, player_2) = parse_input(input);

    play_game(player_1, player_2, false).1
}

#[aoc(day22, part2)]
fn solve_part2(input: &str) -> u32 {
    let (player_1, player_2) = parse_input(input);

    play_game(player_1, player_2, true).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

        assert_eq!(solve_part1(input), 306);
        assert_eq!(solve_part2(input), 291);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day22.txt");

        assert_eq!(solve_part1(input), 34664);
        assert_eq!(solve_part2(input), 32018);
    }
}
