use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(Clone, Eq, Debug, Serialize, Deserialize)]
pub struct Minion {
    attack: i32,
    health: i32,
    mana_cost: i32,
    name: String,  // this should be uniqe
    rand_int: i32, // used for deck shuffling
}

impl Ord for Minion {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rand_int.cmp(&other.rand_int) {
            Ordering::Equal => self.name.cmp(&other.name),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

impl PartialOrd for Minion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Minion {
    fn eq(&self, other: &Self) -> bool {
        self.rand_int == other.rand_int && self.name == other.name
    }
}

impl Minion {
    pub fn new(attack: i32, health: i32, mana_cost: i32, name: &str) -> Minion {
        Minion {
            attack,
            health,
            mana_cost,
            name: name.to_string(),
            rand_int: 0,
        }
    }

    // Applies dmg to the minion
    // Returns true if the minion dies
    pub fn apply_dmg(&mut self, dmg: i32) -> bool {
        self.health -= dmg;
        if self.health > 0 {
            return false;
        }
        true
    }

    pub fn get_attack(&self) -> i32 {
        self.attack
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn get_mana(&self) -> i32 {
        self.mana_cost
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Sets random int to a random number so the deck can be shuffled
    pub fn randomize_order(&mut self, rng: &mut rand::rngs::ThreadRng) {
        self.rand_int = rng.gen();
    }

    pub fn reset_rand_int(&mut self) {
        self.rand_int = 0;
    }

    // Constructors for existing cards
    pub fn solider() -> Minion {
        Minion::new(1, 1, 1, "Solider")
    }

    pub fn swordsman() -> Minion {
        Minion::new(2, 2, 2, "Swordsman")
    }

    pub fn shieldmaster() -> Minion {
        Minion::new(2, 4, 3, "Shieldmaster")
    }

    pub fn archer() -> Minion {
        Minion::new(4, 2, 3, "Archer")
    }

    pub fn pikeman() -> Minion {
        Minion::new(3, 3, 3, "Pikeman")
    }

    pub fn assasin() -> Minion {
        Minion::new(6, 3, 4, "Assasin")
    }

    pub fn beast() -> Minion {
        Minion::new(5, 4, 4, "Beast")
    }

    pub fn fortress() -> Minion {
        Minion::new(2, 7, 4, "Fortress")
    }

    pub fn knight() -> Minion {
        Minion::new(6, 6, 5, "Knight")
    }

    pub fn wizard() -> Minion {
        Minion::new(7, 5, 5, "Wizard")
    }
}

// Enum class that indicates what did the Minion attack
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Attack {
    Trade, // trade with other Minion
    Face,  // attacked opponent's health
    None,  // did not attack this turn
}
