use serde::{Deserialize, Serialize};

use crate::card::Minion;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hand {
    // Most left card in hand is on index 0
    // Most right card is at the end of vec
    cards: Vec<Minion>,
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: Vec::<Minion>::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        self.cards.len()
    }

    pub fn add_card(&mut self, card: Minion) {
        self.cards.push(card);
    }

    pub fn get_cards(&self) -> &Vec<Minion> {
        &self.cards
    }

    pub fn get_card(&mut self, card_number: usize) -> Minion {
        assert!(card_number < self.cards.len());
        self.cards.remove(card_number)
    }

    pub fn get_cards_cost(&self, card_number: usize) -> i32 {
        assert!(card_number < self.cards.len());
        self.cards[card_number].get_mana()
    }
}
