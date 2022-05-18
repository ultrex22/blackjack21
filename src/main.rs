// The goal of blackjack is to beat the dealer's hand without going over 21.
// Face cards are worth 10. Aces are worth 1 or 11, whichever makes a better hand.
// Each player starts with two cards, one of the dealer's cards is hidden until the end.
// To 'Hit' is to ask for another card. To 'Stand' is to hold your total and end your turn.
// If you go over 21 you bust, and the dealer wins regardless of the dealer's hand.
// If you are dealt 21 from the start (Ace & 10), you got a blackjack.
// Dealer will hit until his/her cards total 17 or higher.

// todo: add color to terminal or similar??
//  todo: add betting???

mod deck;
mod players;

use crate::deck::Deck;
use crate::players::*;

// test
fn main() {
    let mut deck = Deck::new();
    deck.shuffle_cards();
    let mut dealer = Dealer::new();
    let mut player = Player::new();
    deck.start_game(&mut dealer, &mut player);
    let players_out = players_turn(&mut player, &mut dealer, &mut deck);
    if !players_out {
        dealer_turn(dealer, player, deck);
    }
}

fn players_turn(player: &mut Player, dealer: &mut Dealer, deck: &mut Deck) -> bool {
    let mut player_over = false;
    loop {
        println!("\nDo you want Another card or Hold? (A or H): ");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let another = buffer.trim().to_lowercase();
        if &another[..] == "a" {
            player.hand.push(deck.next_card());
            player.show();
            println!();
            match player.hand_total() {
                over if over > 21 => {
                    println!("Player has gone over, Dealer wins!");
                    println!(
                        "Dealer Total: {:?}, Player Total: {:?}",
                        dealer.hand_total(),
                        player.hand_total()
                    );
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

fn dealer_turn(mut dealer: Dealer, player: Player, mut deck: Deck) {
    while dealer.hand_total() <= 17 {
        println!("Dealer takes a card...");
        dealer.hand.push(deck.next_card())
    }
    println!("Dealer stays...");
    let dealer_total = dealer.hand_total();

    match player.hand_total() {
        _x if dealer.hand_total() > 21 => {
            println!("Dealer has gone over, Player wins!")
        }
        total if total == 21 => {
            println!("Player wins, BlackJack!")
        }
        total if total > dealer_total => {
            println!("Dealer wins!");
        }
        total if total < dealer_total => {
            println!("Player wins!");
        }
        _ => {
            println!("Tie!")
        }
    }
    println!(
        "Dealer Total: {:?}, Player Total: {:?}",
        dealer_total,
        player.hand_total()
    );
}
