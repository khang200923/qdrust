mod qd;
mod bot;
mod app;

use tokio;
use clap::{Parser, Subcommand};
use crate::app::enums::ColorMode;
use crate::app::battle::battle;
use crate::app::playbot::play_bot;
use crate::app::playbotcli::play_bot_cli;

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
        #[arg(name = "BOT", default_value = "random")]
        bot_string: String,
        #[arg(default_value_t = 8000)]
        port: u16,
        #[arg(long, help = "Use a randomly generated token for security", default_value = "false")]
        use_token: bool,
        #[arg(long, help = "Automatically open the browser", default_value = "false")]
        open_browser: bool,
    },
    #[command(about = "Play against a bot (in CLI)")]
    PlayBotCli {
        #[arg(name = "BOT", default_value = "random")]
        bot_string: String,
        #[arg(long, default_value_t = ColorMode::Random)]
        color: ColorMode,
    },
    #[command(about = "Let bots battle and get their eloes", long_about = None)]
    Battle {
        #[arg(name = "bots")]
        bot_strings: Vec<String>,
        #[arg(long, default_value_t = 100)]
        num_matchups: usize,
        #[arg(long, default_value_t = 1)]
        num_threads: usize,
        #[arg(long, default_value_t = 32.)]
        k_start: f32,
        #[arg(long, default_value_t = 32.)]
        k_end: f32,
        #[arg(long, default_value_t = false)]
        sorted: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::PlayBot { bot_string, port, use_token, open_browser } => {
            play_bot(bot_string, port, use_token, open_browser).await
                .expect("Failed to start bot server");
        }
        Commands::PlayBotCli { bot_string, color } => {
            play_bot_cli(bot_string, color);
        }
        Commands::Battle { 
            bot_strings, 
            num_matchups, 
            num_threads, 
            k_start,
            k_end,
            sorted,
        } => {
            battle(bot_strings, num_matchups, num_threads, k_start, k_end, sorted);
        },
    }
}