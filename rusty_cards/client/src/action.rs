use serde::{Deserialize, Serialize};

use crate::game::{player::Side, GameState};

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Start(String),          // information that the game starts with the greetings message
    PlayCard(usize, usize), // card number, field number
    EndTurn,                // performed at the end of the turn
    Help,                   // lists all the possible actions
}

// Checks if the provided card can be played to the provided field.
// Prints the reason if the action is illegal.
// To play a card there must be certain conditions met:
// 1. You must have enaugh mana to play the card
// 2. You must provide an index of the card in hand that exists
// 3. You must provide an index of field that exists and is empty
fn is_legal_to_play(card_num: usize, field_num: usize, game_state: &GameState) -> bool {
    // VALID INDEX IN HAND
    if card_num > game_state.get_current_player_hand_size() {
        println!("To play a card you must provide a correct card number");
        println!(
            "Provided number: {}, Hand size: {}",
            card_num,
            game_state.get_current_player_hand_size()
        );
        return false;
    }

    // ENAUGH MANA
    let players_mana = game_state.get_current_player_mana();
    let card_cost = game_state.get_current_player_nth_card_mana_cost(card_num);
    if players_mana < card_cost {
        println!("You don't have enaugh mana to play the card that you chose");
        println!("Card cost: {}, Your mana: {}", card_cost, players_mana);
        return false;
    }

    // CORRECT FIELD NUMBER
    if 7 < field_num {
        println!("You must provide a field number that is on your side of the board (from 1 to 7)");
        println!("Provided field number: {}", field_num);
    }

    // IS FIELD EMPTY
    if !game_state.is_field_empty(field_num) {
        println!("You must choose an empty field to play a card");
        println!("Provided field number: {}", field_num);
        return false;
    }

    true
}

// Checks whether an action is legal. Prints reason if it's illegal.
fn is_legal(action: Action, game_state: &GameState) -> bool {
    match action {
        Action::PlayCard(n1, n2) => is_legal_to_play(n1, n2, game_state),
        Action::EndTurn => true,
        Action::Help => true,
        _ => panic!("Unknown or not ingame action"), // this should never be reached
    }
}

// Returns action that has been performed
pub fn perform_action(
    game_ends: &mut bool,
    winner: &mut Side,
    action: Action,
    game_state: &mut GameState,
) -> Action {
    match action {
        Action::PlayCard(n1, n2) => {
            if !is_legal(Action::PlayCard(n1, n2), game_state) {
                return Action::Help;
            }

            if game_state.is_my_turn() {
                game_state.play_from_hand(n1, n2, Side::Me);
            } else {
                game_state.play_from_hand(n1, n2, Side::Opponent);
            }
            Action::PlayCard(n1, n2)
        }
        Action::EndTurn => {
            (*game_ends, *winner) = game_state.end_turn();
            Action::EndTurn
        }
        Action::Help => {
            println!("Here is a list of possible actions:");
            println!("1. Play Card arg1 arg2 - to play card number arg1 to the arg2 field");
            println!("2. End Turn - to end the turn and proceed attacks");
            println!("3. Help - to list all the available actions");
            Action::Help
        }
        _ => panic!("Unknown or not ingame action"), // this should never be reached
    }
}
