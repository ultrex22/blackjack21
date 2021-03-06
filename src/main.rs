// The goal of blackjack is to beat the dealer's hand without going over 21.
// Face cards are worth 10. Aces are worth 1 or 11, whichever makes a better hand.
// Each player starts with two cards, one of the dealer's cards is hidden until the end.
// To 'Hit' is to ask for another card. To 'Stand' is to hold your total and end your turn.
// If you go over 21 you bust, and the dealer wins regardless of the dealer's hand.
// If you are dealt 21 from the start (Ace & 10), you got a blackjack.
// Dealer will hit until his/her cards total 17 or higher.

//  todo: add betting???

use std::io::stdout;

use crossterm::execute;
use crossterm::style::Print;
use crossterm::style::Stylize;
use crossterm::style::{Color, ResetColor, SetForegroundColor};

use crate::deck::Deck;
use crate::players::*;

mod deck;
mod players;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle_cards();
    let mut dealer = Dealer::new();
    let mut player = Player::new();
    deck.start_game(&mut dealer, &mut player);

    if player.hand_total() == 21 {
        println!("{}", "Player was dealt BlackJack!".magenta());
    } else {
        let players_out = players_turn(&mut player, &mut dealer, &mut deck);

        if !players_out {
            dealer_turn(dealer, player, deck);
        } else {
            //place holder
        }
    }
    println!("\n{}", "GAME OVER".green());
}

fn players_turn(player: &mut Player, dealer: &mut Dealer, deck: &mut Deck) -> bool {
    let mut player_over = false;

    loop {
        println!(
            "{}",
            "\nDo you want Another card or Hold? (A or H): ".green()
        );

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let another = buffer.trim().to_lowercase();

        if &another[..] == "a" {
            player.hand.push(deck.next_card());
            player.show();
            println!();
            match player.hand_total() {
                over if over > 21 => {
                    println!("{}", "Player has gone over, Dealer wins!".yellow());
                    game_totals(player, dealer);
                    player_over = true;
                    break;
                }
                _ => {}
            }
        } else {
            break;
        }
    }
    player_over
}

fn game_totals(player: &mut Player, dealer: &mut Dealer) {
    let _res = execute!(
        stdout(),
        SetForegroundColor(Color::Magenta),
        Print("\n"),
        Print("Dealer Total: ".to_string()),
        Print(dealer.hand_total()),
        Print(", "),
        Print("Player Total: ".to_string()),
        Print(player.hand_total()),
        Print("\n"),
        ResetColor
    );
}

fn dealer_turn(mut dealer: Dealer, mut player: Player, mut deck: Deck) {
    while dealer.hand_total() <= 17 {
        println!("{}", "Dealer takes a card...".yellow());
        dealer.hand.push(deck.next_card())
    }
    println!("{}", "Dealer stays...".yellow());
    let dealer_total = dealer.hand_total();

    match player.hand_total() {
        _x if dealer.hand_total() > 21 => {
            println!("{}", "Dealer has gone over, Player wins!".blue())
        }
        total if total == 21 => {
            println!("{}", "Player wins, BlackJack!".blue())
        }
        total if total > dealer_total => {
            println!("{}", "Player wins!".blue());
        }
        total if total < dealer_total => {
            println!("{}", "Dealer wins!".yellow());
        }
        _ => {
            println!("{}", "Tie!".green())
        }
    }

    game_totals(&mut player, &mut dealer);
}
