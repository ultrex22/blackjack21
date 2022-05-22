use std::io::stdout;

use crossterm::style::Stylize;
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::{execute, style::Print};

use crate::deck::Card;

#[derive(Debug)]
pub struct Dealer {
    pub(crate) hand: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Self {
        Self { hand: Vec::new() }
    }
    pub fn show(&self) {
        println!("{}", "Dealers Card: <Face Down>".yellow());
        for card in &self.hand[1..] {
            let _res = execute!(
                stdout(),
                SetForegroundColor(Color::Yellow),
                Print("Dealers Card: ".to_string()),
                Print(card),
                Print("\n"),
                ResetColor
            );
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
            let _res = execute!(
                stdout(),
                SetForegroundColor(Color::Blue),
                Print("Your Card: ".to_string()),
                Print(card),
                Print("\n"),
                ResetColor
            );
        }
    }
    pub fn hand_total(&self) -> u8 {
        let total: u8 = self.hand.iter().map(|card| card.face.1).sum();
        total
    }
}
