use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::card::*;
use crate::deck::*;
use crate::display;
use crate::hand::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Side {
    Opponent,
    Me,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    deck: Deck,
    hand: Hand,
    health: i32,
    mana: i32,
    side: Side,
}

impl Player {
    pub fn opponent() -> Player {
        Player {
            deck: Deck::default(),
            hand: Hand::new(),
            health: 20,
            mana: 5,
            side: Side::Opponent,
        }
    }

    pub fn me() -> Player {
        Player {
            deck: Deck::default(),
            hand: Hand::new(),
            health: 20,
            mana: 5,
            side: Side::Me,
        }
    }

    pub fn draw_card(&mut self) {
        match self.deck.get_top_card() {
            Some(minion) => self.hand.add_card(minion),
            None => println!("No more cards in deck -_-;"),
        }
    }

    pub fn begin(&mut self) {
        for _ in 1..4 {
            self.draw_card();
        }
    }

    pub fn get_card(&self, idx: usize) -> &Minion {
        assert!(idx > 0);
        let i = idx - 1;
        assert!(i < self.hand.get_size());
        &self.hand.get_cards()[i]
    }

    pub fn throw_card(&mut self, idx: usize) -> Minion {
        assert!(idx > 0);
        let i = idx - 1;
        assert!(i < self.hand.get_size());
        assert!(self.mana >= self.hand.get_cards_cost(i));
        self.spend_mana(self.hand.get_cards_cost(i));
        self.hand.get_card(i)
    }

    // Returns true if a player dies
    pub fn receive_dmg(&mut self, dmg: i32) -> bool {
        assert!(dmg > 0);
        self.health -= dmg;

        if self.health <= 0 {
            return true;
        }

        false
    }

    pub fn get_mana(&self) -> i32 {
        self.mana
    }

    pub fn spend_mana(&mut self, mana_points: i32) {
        assert!(mana_points <= self.mana);
        self.mana -= mana_points;
    }

    pub fn reset_mana(&mut self) {
        self.mana = 5;
    }

    pub fn set_side(&mut self, side: Side) {
        self.side = side;
    }

    pub fn get_side(&self) -> &Side {
        &self.side
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }

    pub fn hand_size(&self) -> usize {
        self.hand.get_size()
    }

    fn display_as_opponent(&self) {
        println!("Opponent's {}: {},", "HEALTH".red(), self.health);
        println!("Opponent's {}: {}\n", "MANA POINTS".cyan(), self.mana);
        println!(
            " {}      {} ",
            "Opponent's HAND".bright_green(),
            "Opponent's DECK".bright_green()
        );
        println!("+---------------+    +---------------+");
        println!("|     Cards:    |    |  Cards Left:  |");
        println!("|               |    |               |");
        println!(
            "{}    {}",
            display::display_bound_center(
                display::DECK_DISPLAY_WIDTH,
                &self.hand.get_size().to_string()
            ),
            display::display_bound_center(
                display::DECK_DISPLAY_WIDTH,
                &self.deck.get_size().to_string()
            ),
        );
        println!("+---------------+    +---------------+");
    }

    fn display_as_me(&self) {
        let mut minions = Vec::new();
        for minion in self.hand.get_cards() {
            minions.push(Some(minion));
        }

        println!("\nMy HAND:");
        display::display_card_row(&minions, true, display::EdgeNum::Low);

        println!("My DECK:");
        self.deck.display();
        println!("My {}: {},", "HEALTH".red(), self.health);
        println!("My {}: {}\n", "MANA POINTS".cyan(), self.mana);
    }

    pub fn display(&self) {
        match self.side {
            Side::Opponent => self.display_as_opponent(),
            Side::Me => self.display_as_me(),
        }
    }
}
