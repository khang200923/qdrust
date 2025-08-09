mod qd;
mod bot;
mod app;

use clap::{Parser, Subcommand};
use crate::app::battle::battle;
use crate::app::playbot::play_bot;

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
            battle(bot_strings, num_matchups, num_threads, k);
        },
        Commands::PlayBot { bot_string, color } => {
            play_bot(bot_string, color);
        }
    }
}