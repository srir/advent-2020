use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Deck = Vec<usize>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Player {
    Player1,
    Player2
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Status {
    Won(Player),
    InProgress
}


#[derive(Debug, Clone)]
struct Combat {
    player1: Deck,
    player2: Deck
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> (Deck, Deck) {
    let (p1, p2) = input.split_at(input.find("\n\n").expect("malformed"));

    (
        p1.trim().lines().skip(1).map(|c| c.parse().expect("malformed")).collect(),
        p2.trim().lines().skip(1).map(|c| c.parse().expect("malformed")).collect(),
    )
}

trait Game {
    fn game_status(&self) -> Status;
    fn play_round(&mut self) -> Status;
    fn play_game(&mut self) -> Player;
    fn high_score(&self) -> usize;
}

impl Combat {
    fn new(player1: Deck, player2: Deck) ->  Combat {
        Combat { player1, player2 }
    }
}

impl Game for Combat {
    fn game_status(&self) -> Status {
        match (self.player1.len(), self.player2.len()) {
            (_, 0) => Status::Won(Player::Player1),
            (0, _) => Status::Won(Player::Player2),
            _ => Status::InProgress
        }
    }

    fn play_round(&mut self) -> Status {
        match (self.player1.as_slice(), self.player2.as_slice()) {
            ([p1, p1_deck @ ..], [p2, p2_deck @ ..]) => {
                if p1 > p2 {
                    let mut new_p1_deck = p1_deck.to_vec();
                    new_p1_deck.extend(vec![p1, p2]);
                    self.player1 = new_p1_deck;
                    self.player2 = p2_deck.to_vec();
                } else {
                    let mut new_p2_deck = p2_deck.to_vec();
                    new_p2_deck.extend(vec![p2, p1]);
                    self.player2 = new_p2_deck;
                    self.player1 = p1_deck.to_vec();
                }
            }
            _ => ()
        }

        self.game_status()
    }

    // winner
    fn play_game(&mut self) -> Player {
        while self.game_status() == Status::InProgress {
            self.play_round();
        }

        match self.game_status() {
            Status::InProgress => unreachable!("unexpected in progress game"),
            Status::Won(player) => player
        }
    }

    fn high_score(&self) -> usize {
        vec![self.player1.as_slice(), self.player2.as_slice()].iter().map(|&deck| {
            deck.iter().rev().enumerate().map(|(i, card)| {
                card * (i+1)
            }).sum()
        }).max().unwrap()
    }
}

#[derive(Debug, Clone)]
struct RecursiveCombat {
    previous_rounds: HashSet<(Deck, Deck)>,
    player1: Deck,
    player2: Deck,
    game_depth: usize,
}

impl RecursiveCombat {
    fn new(player1: Deck, player2: Deck) -> RecursiveCombat {
        RecursiveCombat {
            previous_rounds: HashSet::new(),
            player1,
            player2,
            game_depth: 0,
        }
    }

    fn new_with_depth(player1: Deck, player2: Deck, game_depth: usize) -> RecursiveCombat {
        RecursiveCombat {
            previous_rounds: HashSet::new(),
            player1,
            player2,
            game_depth,
        }
    }

    fn subgame(&self) -> Option<RecursiveCombat> {
        match (self.player1.as_slice(), self.player2.as_slice()) {
            ([p1, p1_deck @ ..], [p2, p2_deck @ ..]) => {
                if p1_deck.len() >= *p1 && p2_deck.len() >= *p2 {
                    Some(RecursiveCombat::new_with_depth(
                        p1_deck.to_vec(),
                        p2_deck.to_vec(),
                        self.game_depth + 1
                    ))
                } else {
                    None
                }
            }
            _ => None
        }
    }
}

impl Game for RecursiveCombat {
    fn game_status(&self) -> Status {
        let this_round = (self.player1.clone(), self.player2.clone());

        if self.previous_rounds.contains(&this_round) {
            Status::Won(Player::Player1)
        } else {
            match (self.player1.len(), self.player2.len()) {
                (_, 0) => Status::Won(Player::Player1),
                (0, _) => Status::Won(Player::Player2),
                _ => Status::InProgress
            }
        }
    }

    // -> Winner
    fn play_game(&mut self) -> Player {
        while self.game_status() == Status::InProgress {
            println!("Depth {} round, {:?} vs {:?}", self.game_depth, self.player1, self.player2);

            let round_result = self.play_round();

            // println!("> played round {}, => {:?}", round_count, round_result);
        }

        println!("Depth {} game done! player1: {:?}, player2: {:?}", self.game_depth, self.player1, self.player2);

        match self.game_status() {
            Status::InProgress => unreachable!("unexpected in progress game"),
            Status::Won(player) => player
        }
    }

    fn high_score(&self) -> usize {
        vec![self.player1.as_slice(), self.player2.as_slice()].iter().map(|&deck| {
            deck.iter().rev().enumerate().map(|(i, card)| {
                card * (i+1)
            }).sum()
        }).max().unwrap()
    }

    fn play_round(&mut self) -> Status {
        let this_round = (self.player1.clone(), self.player2.clone());

        // println!("player1: {:?}, player2: {:?}", self.player1, self.player2);

        if self.previous_rounds.contains(&this_round) {
            Status::Won(Player::Player1)
        } else {
            self.previous_rounds.insert(this_round);

            let round_winner = if let Some(mut subgame) = self.subgame() {
                Some(subgame.play_game())
            } else {
                match (self.player1.as_slice(), self.player2.as_slice()) {
                    ([p1, ..], [p2, ..]) => {
                        if p1 > p2 {
                            Some(Player::Player1)
                        } else {
                            Some(Player::Player2)
                        }
                    }
                    _ => None
                }
            };

            match round_winner {
                Some(Player::Player1) => {
                    let p1 = self.player1.drain(..1).next().unwrap();
                    let p2 = self.player2.drain(..1).next().unwrap();

                    self.player1.extend(vec![p1, p2]);
                },
                Some(Player::Player2) => {
                    let p1 = self.player1.drain(..1).next().unwrap();
                    let p2 = self.player2.drain(..1).next().unwrap();

                    self.player2.extend(vec![p2, p1]);
                },
                None => ()
            }

            self.game_status()
        }
    }
}

#[aoc(day22, part1)]
fn winning_players_score((p1_deck, p2_deck): &(Deck, Deck)) -> usize {
    let mut game = Combat::new(p1_deck.to_vec(), p2_deck.to_vec());
    game.play_game();

    game.high_score()
}

#[aoc(day22, part2)]
fn recursive_combat_winning_score((p1_deck, p2_deck): &(Deck, Deck)) -> usize {
    let mut game = RecursiveCombat::new(p1_deck.to_vec(), p2_deck.to_vec());
    game.play_game();

    game.high_score()
}
