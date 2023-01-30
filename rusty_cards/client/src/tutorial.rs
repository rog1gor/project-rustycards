use crate::game;
use crate::utils;

fn cont(s: &str) {
    while s != utils::provide_input(format!("Type \'{}\' to continue", s).as_str()) {}
}

pub fn run_tutorial() {
    let mut game_state = game::GameState::new();
    game_state.set_turn(true);
    game_state.display();
    println!("That's how the game will look like");
    cont("c");

    game_state.begin();
    game_state.display();
    println!("At the very beggining of the game, each player draws 3 cards");
    cont("c");

    game_state.display();
    println!("During your turn you can play cards from hand (indexes at the bottom of the cards) to the battlefield (indexes from 1 to 7)");
    println!("After the end of the turn, the cards that started the turn on the board will attack forward.");
    println!("It means, that if there is a card at the opposing field, your unit will attack the opposing unit.");
    println!("If there is no unit in front of the attacking unit, then the attack will go directly at the opponent, applying damage to it's health points.");
    println!("Your goal is to drop your opponent's health to 0 or less.");
    cont("c");

    game_state.display();
    println!("Okay! Now that you know the rules it's time to play!");
    println!("Try playing card from the left of your hand to the 3rd field. To do so, type \'play card 1 3\'");
    cont("play card 1 3");

    game_state.play_from_hand(1, 3, game::player::Side::Me);
    game_state.display();

    println!("Great! You succesfully played a card from hand. In order to do this, you had to spend 5 mana, so you can't play any card this turn.");
    println!("Since you can't play any other card, end the turn. To do so, type \'end turn\'");
    cont("end turn");

    game_state.end_turn();
    game_state.display();

    println!("As you can see, a new card appeared in your hand and your mana got refilled. These two things will always happen at the end of your turn.");
    println!("Also, you can recognize that opponent's health didn't drop. That's because you've just played your card - it didn't start the turn on the board, so it didn't attack.");
    println!("Now you must wait for the opponent to finish all of it's actions and end it's turn");
    cont("c");

    game_state.play_from_hand(3, 1, game::player::Side::Opponent);
    game_state.end_turn();
    game_state.display();

    println!("You're opponent, also played a card and ended it's turn afterwards.");
    println!("Try to block opponent's unit with your card. To do so, type \'play card 2 7\'");
    cont("play card 2 7");

    game_state.play_from_hand(2, 7, game::player::Side::Me);
    game_state.display();

    println!("That's nice. Your Wizard will attack opponent's health this turn and enemy Knight will trade with your Knight.");
    println!("Since you've used all your mana, end the turn");
    cont("end turn");

    game_state.end_turn();
    game_state.display();

    println!(
        "As you can see, your opponent's health dropped by 7, because your wizard attacked it."
    );
    println!("Now you have to wait for your opponent to end his turn.");
    println!("This time your opponent won't play any cards and will just end it's turn");

    cont("c");

    game_state.end_turn();
    game_state.display();

    println!("The Knights on the 7th file traded and since they both had 6 health and 6 attack - they died");
    println!("If the card doesn't die after the trade, the damage is still applied");

    cont("c");

    print!("\x1B[2J\x1B[1;1H");
    println!("Okay, you know all the basics so you're ready to play!");

    cont("c");
}
