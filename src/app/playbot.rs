use std::io::{self, Write};
use rand::Rng;
use regex::Regex;
use crate::bot::collections::map_bot_string;
use crate::qd::legalcomp::get_possible_legal_moves;
use crate::qd::state::GameState;
use crate::qd::utils::gsvd;

pub fn play_bot(bot_string: String, color: String) {
    let bot = map_bot_string(&bot_string);
    if bot.is_none() {
        eprintln!("\"{}\" does not exist", bot_string);
        return;
    }
    let color = color.to_lowercase();
    let color = match color.as_str() {
        "white" => true,
        "black" => false,
        "random" => {
            let mut rng = rand::thread_rng();
            rng.gen_bool(0.5)
        },
        _ => {
            eprintln!("Invalid color. Use 'white' or 'black'.");
            return;
        }
    };
    let bot = bot.unwrap();
    println!("You are playing against {} as {}", bot_string, if color { "white" } else { "black" });
    let mut game = GameState::def();
    while game.result().is_none() {
        if game.is_white_turn != color {
            game.make_move(bot.decide(game.clone()));
            continue;
        }
        println!("{}", gsvd(&game));
        let mut input = String::new();
        let re = Regex::new(r"^[a-h][1-8]$").unwrap();
        print!("Your move: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        if !re.is_match(&input) {
            println!("Invalid move format.");
            continue;
        }
        let file = input.chars().nth(0).unwrap() as u8 - b'a';
        let rank = input.chars().nth(1).unwrap() as u8 - b'1';
        let move_to = rank * 8 + file;
        if get_possible_legal_moves(&game) & 1 << move_to == 0 {
            println!("Illegal move.");
            continue;
        }
        game.make_move(move_to);
    }
    println!("{}", gsvd(&game));
    if game.result() == Some(color) {
        println!("You won!");
    } else {
        println!("You lost.");
    }
}