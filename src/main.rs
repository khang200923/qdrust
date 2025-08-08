mod qd;
mod bot;

use std::io;
use std::io::{Write};
use std::{time::Instant};
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use rand::Rng;
use regex::Regex;
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use crate::qd::state::GameState;
use crate::qd::legalcomp::get_possible_legal_moves;
use crate::qd::utils::{gsvd};
use crate::bot::base::Bot;
use crate::bot::collections::map_bot_string;
use crate::bot::elo::run_tournament;

#[derive(Parser, Debug)]
#[command(name = "qdrust")]
#[command(about = "Queen Duel chess variant in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Play against a bot")]
    PlayBot {
        #[arg(name = "bot", default_value = "random")]
        bot_string: String,
        #[arg(long, default_value = "random")]
        color: String,
    },
    #[command(about = "Let bots battle and get their eloes", long_about = None)]
    Battle {
        #[arg(name = "bots")]
        bot_strings: Vec<String>,
        #[arg(long, default_value_t = 100)]
        num_matchups: usize,
        #[arg(long, default_value_t = 1)]
        num_threads: usize,
        #[arg(short, default_value_t = 32.)]
        k: f32,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Battle { 
            bot_strings, 
            num_matchups, 
            num_threads, 
            k 
        } => {
            let bot_zip: Vec<(Option<Box<dyn Bot>>, String)> = 
            bot_strings.clone().into_iter().map(
                    |x| (map_bot_string(&x), x.clone())
                ).collect();
            for (bot, string) in &bot_zip {
                if bot.is_none() {
                    eprintln!("\"{}\" does not exist", string);
                    return ();
                } 
            }
            let bots: Vec<Box<dyn Bot>> = 
                bot_zip.into_iter().map(
                    |(x, _)| x.expect("")
                ).collect();
            if bots.len() <= 1 {
                eprintln!("You need at least 2 bots to battle");
                return;
            }
                
            let bar = Arc::new(ProgressBar::new(num_matchups as u64));
            bar.set_style(
                ProgressStyle::with_template("[{msg}] [{bar:40.cyan/blue}] {pos:>7}/{len:7}")
                    .unwrap()
                    .progress_chars("##-"),
            );

            fn prog_func_base(inc: usize, _num_matchups: usize, bar: &ProgressBar, start: &Instant) {
                let elapsed = start.elapsed();
                bar.set_message(format!("{:.3}s", elapsed.as_secs_f64()));
                bar.inc(inc as u64);
            }

            let start = Arc::new(Instant::now());
            let prog_func = Box::new(
                |inc| prog_func_base(inc, num_matchups, &bar, &start)
            );
            let tournament_running = Arc::new(AtomicBool::new(true));
            let start_clone = start.clone();
            let bar_clone = bar.clone();
            let tournament_running_clone = tournament_running.clone();
            thread::spawn(move || {
                while tournament_running_clone.load(Ordering::Relaxed) {
                    thread::sleep(std::time::Duration::from_millis(10));
                    let elapsed = start_clone.elapsed();
                    bar_clone.set_message(format!("{:.3}s", elapsed.as_secs_f64()));
                }
            });
            let elo_scores = run_tournament(
                bots, 
                num_matchups, 
                Some(k), 
                num_threads,
                &Some(prog_func));
            bar.finish();

            let min_elo = elo_scores.iter().cloned().fold(f32::INFINITY, f32::min);
            let elo_scores: Vec<f32> = elo_scores.into_iter().map(|elo| elo - min_elo).collect();

            let elo_zip = bot_strings.clone().into_iter().zip(elo_scores.into_iter());
            for (bot, elo) in elo_zip {
                println!("{}: {:.0}", bot, elo);
            }
        },
        Commands::PlayBot { bot_string, color } => {
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
    }
}