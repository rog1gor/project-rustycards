use rand;
use serde::{Deserialize, Serialize};

use crate::card::*;
use crate::display;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deck {
    // The top card is considered to be at the end of the vector
    cards: Vec<Minion>,
}

impl Deck {
    pub fn default() -> Deck {
        Deck {
            cards: vec![
                Minion::solider(),
                Minion::solider(),
                Minion::swordsman(),
                Minion::swordsman(),
                Minion::shieldmaster(),
                Minion::shieldmaster(),
                Minion::archer(),
                Minion::archer(),
                Minion::pikeman(),
                Minion::pikeman(),
                Minion::assasin(),
                Minion::assasin(),
                Minion::beast(),
                Minion::beast(),
                Minion::fortress(),
                Minion::fortress(),
                Minion::knight(),
                Minion::knight(),
                Minion::wizard(),
                Minion::wizard(),
            ],
        }
    }

    pub fn get_size(&self) -> usize {
        self.cards.len()
    }

    pub fn get_top_card(&mut self) -> Option<Minion> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        let mut shuffler = rand::thread_rng();
        for card in &mut self.cards {
            card.randomize_order(&mut shuffler);
        }

        self.cards.sort();

        for card in &mut self.cards {
            card.reset_rand_int();
        }
    }

    pub fn display(&self) {
        println!("{}", display::display_edge(display::DECK_DISPLAY_WIDTH));
        println!(
            "{}",
            display::display_bound_center(display::DECK_DISPLAY_WIDTH, "Cards left:")
        );
        println!(
            "{}",
            display::display_bound_center(display::DECK_DISPLAY_WIDTH, "")
        );
        println!(
            "{}",
            display::display_bound_center(
                display::DECK_DISPLAY_WIDTH,
                &self.get_size().to_string()
            )
        );
        println!("{}", display::display_edge(display::DECK_DISPLAY_WIDTH));
    }
}
