use serde::{Deserialize, Serialize};

use crate::utils;
use crate::game::{GameState, player::Side};

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Start(String),          // Information that the game starts with the greetings message
    PlayCard(usize, usize), // card number, field number
    EndTurn,                // Performed at the end of the turn
    Help,                   // lists all the possible actions
}

fn is_legal() {
    todo!();
}

pub fn perform_action(game_state: &mut GameState) {
    // 1. Zczytać z stdin akcje
    // 2. zparsowac akcje
    // TODO 3. spradzic czy jest legalna
    // 4. jesli jest legalna to wykonac
    // TODO 5. wyslac akcje do przeciwnika
    let action = utils::provide_action();

    match action {
        Action::PlayCard(n1, n2) => {
            if game_state.is_my_turn() {
                game_state.play_from_hand(n1, n2, Side::Me);
            } else {
                game_state.play_from_hand(n1, n2, Side::Opponent);
            }
        }
        Action::EndTurn => game_state.end_turn(),
        Action::Help => {
            println!("Here is a list of possible actions:");
            println!("1. Play Card arg1 arg2 - to play card number arg1 to the arg2 field");
            println!("2. End Turn - to end the turn and proceed attacks");
            println!("3. Help - to list all the available actions");
        }
        _ => panic!("Error - extraoridanry action"),
    };

    game_state.display();
}
