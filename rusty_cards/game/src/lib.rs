use serde::{Deserialize, Serialize};

pub mod board;
pub mod card;
pub mod deck;
pub mod display;
pub mod hand;
pub mod player;

use board::*;
use player::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    opponent: Player,
    me: Player,
    board: Board,
    is_my_turn: bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            opponent: Player::opponent(),
            me: Player::me(),
            board: Board::new(),
            is_my_turn: false,
        }
    }

    pub fn change_player(&mut self, side: Side, player: &Player) {
        *self.player_by_side(&side) = player.clone();
    }

    pub fn begin(&mut self) {
        self.me.begin();
        self.opponent.begin();
    }

    pub fn set_turn(&mut self, is_my_turn: bool) {
        self.is_my_turn = is_my_turn;
    }

    fn player_by_side(&mut self, side: &Side) -> &mut Player {
        match side {
            Side::Me => &mut self.me,
            Side::Opponent => &mut self.opponent,
        }
    }

    pub fn get_current_player_mana(&self) -> i32 {
        if self.is_my_turn {
            return self.me.get_mana();
        }
        self.opponent.get_mana()
    }

    pub fn get_current_player_hand_size(&self) -> usize {
        if self.is_my_turn {
            return self.me.hand_size();
        }
        self.opponent.hand_size()
    }

    pub fn get_current_player_nth_card_mana_cost(&self, n: usize) -> i32 {
        if self.is_my_turn {
            return self.me.get_card(n).get_mana();
        }
        self.opponent.get_card(n).get_mana()
    }

    pub fn is_field_empty(&self, idx: usize) -> bool {
        if self.is_my_turn {
            return self.board.is_nth_field_empty(idx, Side::Me);
        }
        self.board.is_nth_field_empty(idx, Side::Opponent)
    }

    pub fn play_from_hand(&mut self, hand_idx: usize, board_idx: usize, side: Side) {
        let card = self.player_by_side(&side).throw_card(hand_idx);
        self.board.play_card(card, board_idx, &side);
    }

    pub fn is_my_turn(&self) -> bool {
        self.is_my_turn
    }

    pub fn end_turn(&mut self) -> (bool, Side) {
        let side = match self.is_my_turn {
            true => Side::Me,
            false => Side::Opponent,
        };
        for i in 1..=7 {
            match self.board.attack_on_file(i, &side) {
                card::Attack::Face => {
                    match &side {
                        Side::Me => {
                            if self.opponent.receive_dmg(self.board.get_attack_of_minion(i, &side)) {
                                return (true, Side::Me);
                            }
                        }
                        Side::Opponent => {
                            if self.me.receive_dmg(self.board.get_attack_of_minion(i, &side)) {
                                return (true, Side::Opponent);
                            }
                        }
                    }
                }
                _ => (),
            };
        }

        self.player_by_side(&side).reset_mana();
        self.player_by_side(&side).draw_card();
        self.board.reset_turn();
        self.is_my_turn = !self.is_my_turn;
        
        (false, Side::Me)
    }

    pub fn display(&self) {
        println!();
        if self.is_my_turn {
            println!("It's your turn. Choose an action n_n");
        } else {
            println!("It's your opponent's turn. You must wait for they to finish their actions d-_-b");
        }
        println!();

        self.opponent.display();
        self.board.display();
        self.me.display();
    }
}
