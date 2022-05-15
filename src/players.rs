use crate::deck::{Card, Deck};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Dealer {
    pub(crate) hand: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Self {
        Self { hand: Vec::new() }
    }
    pub fn show(&self) {
        println!("Dealers Card: <Face Down>");
        for card in &self.hand[1..] {
            println!("Dealers Card:{}", card);
        }
    }
    pub fn hand_total(&self) -> u8 {
        let total: u8 = self.hand.iter().map(|card| card.face.1).sum();
        total
    }
}

#[derive(Debug)]
pub struct Player {
    pub(crate) hand: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        Self { hand: Vec::new() }
    }
    pub fn show(&self) {
        for card in &self.hand {
            println!("Your Card: {}", card);
        }
    }
    pub fn hand_total(&self) -> u8 {
        let total: u8 = self.hand.iter().map(|card| card.face.1).sum();
        total
    }
}
