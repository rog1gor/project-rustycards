use serde::{Deserialize, Serialize};

use crate::card::*;
use crate::display;
use crate::player::Side;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    can_attack: bool,
    minion: Option<Minion>,
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

impl Field {
    pub fn new() -> Field {
        Field {
            can_attack: false,
            minion: None,
        }
    }

    pub fn can_attack(&self) -> bool {
        self.can_attack
    }

    pub fn is_empty(&self) -> bool {
        if self.minion.is_none() {
            return true;
        }
        false
    }

    pub fn get_minion(&self) -> &Minion {
        assert!(!self.is_empty());
        match &self.minion {
            Some(m) => m,
            None => panic!("This method should not be called when the field is empty!"),
        }
    }

    pub fn place_minion(&mut self, minion: Minion) {
        self.can_attack = false;
        self.minion = Some(minion);
    }

    pub fn use_attack(&mut self) {
        self.can_attack = false
    }

    pub fn apply_dmg(&mut self, dmg: i32) {
        assert!(!self.is_empty());
        match &mut self.minion {
            Some(m) => {
                if m.apply_dmg(dmg) {
                    self.minion = None;
                }
            }
            None => panic!("This method should not be called when the field is empty!"),
        }
    }

    pub fn reset_turn(&mut self) {
        self.can_attack = true;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    fields: Vec<Field>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            fields: vec![Field::new(); 14],
        }
    }

    // Returns first index of the fields vector that belongs to the player
    fn get_fst_idx(side: &Side) -> usize {
        match side {
            Side::Opponent => 0,
            Side::Me => 7,
        }
    }

    // Translates index provided by the player to the index of the fields vector
    fn translate_idx(idx: usize, side: &Side) -> usize {
        assert!(idx < 14);
        match side {
            Side::Me => 6 + idx,
            Side::Opponent => 7 - idx,
        }
    }

    // Returns index to the field that opposes field under index provided by the player
    fn opposing_idx(idx: usize) -> usize {
        if idx < 7 {
            return idx + 7;
        }

        idx - 7
    }

    pub fn play_card(&mut self, minion: Minion, filed_num: usize, side: &Side) {
        let idx = Self::translate_idx(filed_num, side);
        assert!(self.fields[idx].is_empty());
        self.fields[idx].place_minion(minion);
    }

    pub fn reset_turn(&mut self) {
        for field in &mut self.fields {
            field.reset_turn();
        }
    }

    pub fn is_nth_field_empty(&self, n: usize, side: Side) -> bool {
        assert!(0 < n && n <= 7);
        let idx = Self::translate_idx(n, &side);
        self.fields[idx].is_empty()
    }

    pub fn attack_on_file(&mut self, field_num: usize, side: &Side) -> Attack {
        let field_num = Self::translate_idx(field_num, side);
        let opposing = Self::opposing_idx(field_num);

        if self.fields[field_num].is_empty() || !self.fields[field_num].can_attack() {
            return Attack::None;
        } else if self.fields[opposing].is_empty() {
            return Attack::Face;
        }

        let attack1 = self.fields[field_num].get_minion().get_attack();
        let attack2 = self.fields[opposing].get_minion().get_attack();

        self.fields[field_num].apply_dmg(attack2);
        self.fields[opposing].apply_dmg(attack1);

        Attack::Trade
    }

    pub fn get_attack_of_minion(&self, idx: usize, side: &Side) -> i32 {
        if self.fields[Self::translate_idx(idx, side)].is_empty() {
            return 0;
        }
        self.fields[Self::translate_idx(idx, side)]
            .get_minion()
            .get_attack()
    }

    fn get_minions(&self, side: &Side) -> Vec<Option<&Minion>> {
        let fst_idx = Self::get_fst_idx(side);
        let mut minions = Vec::new();
        for i in fst_idx..(fst_idx + 7) {
            if self.fields[i].is_empty() {
                minions.push(None);
            } else {
                minions.push(Some(self.fields[i].get_minion()));
            }
        }

        minions
    }

    fn display_side(&self, side: Side) {
        let minions = self.get_minions(&side);
        let edge_num = match &side {
            Side::Opponent => display::EdgeNum::Up,
            Side::Me => display::EdgeNum::Low,
        };
        display::display_card_row(&minions, false, edge_num);
    }

    pub fn display(&self) {
        println!("\nBOARD:");
        self.display_side(Side::Opponent);
        self.display_side(Side::Me);
    }
}
