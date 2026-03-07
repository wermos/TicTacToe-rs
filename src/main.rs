use std::io;
use std::process;
use tictactoe_rs::automated_player::AutomatedPlayer;
use tictactoe_rs::definitions::{Player, PlayerType, opposite};
use tictactoe_rs::game::Game;

fn get_user_choice() -> Player {
    println!(
        "Please enter X if you want to be X (and go first) or O if you would like to go second (X/O): "
    );

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let player = match choice.trim().to_ascii_uppercase().as_str() {
        "X" => Player::X,
        "O" => Player::O,
        _ => {
            eprintln!("Invalid choice. Please enter X or O.");
            process::exit(1);
        }
    };

    player
}

fn ask_player_move() -> (usize, usize) {
    todo!()
}

fn main() {
    let mut game = Game::new();

    let human = ask_player_choice();dPlayer::new(opposite(human));
ch human {
        Player::X => [(Player::X, PlayerType::Human),
                      (Player::O, PlayerType::AI)],
        Player::O => [(Player::X, PlayerType::AI),
                      (Player::O, PlayerType::Human)],
    };

    while game.result().is_none() {
        let player = game.current_player();

        let mv = match players[player as usize].1 {
            PlayerType::Human => get_user_move(),
            PlayerType::AI => ai.choose_move(game.board()),
        };

        game.make_move(player, mv);
    }

    let result = game.result();

    match result {
        Some(player) => println!("The winner was {player}."),
        Draw => println!("The game was a draw."),
    }
}

// struct Players {
//     x: PlayerType,
//     o: PlayerType,
// }

// match players.get(player)
