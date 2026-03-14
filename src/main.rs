use std::io;
use std::io::Write;
use std::process;
use tictactoe_rs::automated_player::AutomatedPlayer;
use tictactoe_rs::definitions::{GameResult, Player, PlayerType, opposite};
use tictactoe_rs::game::Game;

fn ask_player_choice() -> Player {
    print!(
        "Please enter X if you want to be X (and go first) or O if you would like to go second (X/O): "
    );
    io::stdout().flush().unwrap();

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim().to_ascii_uppercase().as_str() {
        "X" => Player::X,
        "O" => Player::O,
        _ => {
            eprintln!("Invalid choice. Please enter X or O.");
            process::exit(1);
        }
    }
}

fn get_player_move() -> (usize, usize) {
    println!(" 1 │ 2 │ 3");
    println!("───┼───┼───");
    println!(" 4 │ 5 │ 6");
    println!("───┼───┼───");
    println!(" 7 │ 8 │ 9");
    print!("Please enter a number between 1-9 to make your move.\nYour move: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let idx = choice.trim().parse::<usize>().unwrap() - 1;

    let row = idx / 3;
    let col = idx % 3;

    (row, col)
}

fn main() {
    let mut game = Game::new();

    let human = ask_player_choice();
    let ai = AutomatedPlayer::new(opposite(human));

    println!("You are {}\nComputer is {}", human, opposite(human));

    let players = match human {
        Player::X => [(Player::X, PlayerType::Human), (Player::O, PlayerType::AI)],
        Player::O => [(Player::X, PlayerType::AI), (Player::O, PlayerType::Human)],
    };

    while game.result().is_none() {
        game.print_board();

        let player = game.current_player();

        let (row, col) = match players[player as usize].1 {
            PlayerType::Human => get_player_move(),
            PlayerType::AI => ai.choose_move(&game.board()),
        };

        game.make_move(player, row, col);
    }

    game.print_board();

    let result = game.result().unwrap();

    match result {
        GameResult::Win(player) => println!("The winner was {player}."),
        GameResult::Draw => println!("The game was a draw."),
    }
}

// struct Players {
//     x: PlayerType,
//     o: PlayerType,
// }

// match players.get(player)
