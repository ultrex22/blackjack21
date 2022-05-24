use crossterm::cursor::MoveTo;
use std::fmt::{Display, Formatter};
use std::io::stdout;

use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{Dealer, Player};

const SUITS: [char; 4] = ['\u{2660}', '\u{2663}', '\u{2665}', '\u{2666}'];
const FACE: [(&str, u8); 13] = [
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("10", 10),
    ("Jack", 10),
    ("Queen", 10),
    ("King", 10),
    ("Ace", 11),
];

#[derive(Debug)]
pub struct Card {
    pub(crate) face: (String, u8),
    pub(crate) suit: char,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {} worth {}.", self.face.0, self.suit, self.face.1)
    }
}

#[derive(Debug)]
pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut full_deck = Vec::new();
        for s in SUITS {
            for f in FACE {
                full_deck.push(Card {
                    face: (f.0.to_owned(), f.1),
                    suit: s,
                });
            }
        }
        Self { deck: full_deck }
    }
    pub fn start_game(&mut self, dealer: &mut Dealer, player: &mut Player) {
        dealer.hand.push(self.next_card());
        dealer.hand.push(self.next_card());
        dealer.show();
        println!();

        player.hand.push(self.next_card());
        player.hand.push(self.next_card());
        player.show();
    }
    pub fn next_card(&mut self) -> Card {
        self.deck.pop().unwrap()
    }
    pub fn shuffle_cards(&mut self) {
        let _res = execute!(stdout(), Clear(ClearType::All), MoveTo(0, 10));
        println!("{}", "shuffling...".green());
        println!();
        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
        self.deck.shuffle(&mut rng);
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for card in &self.deck {
            writeln!(f, "{} of {}.", card.face.0, card.suit)?;
        }
        Ok(())
    }
}
