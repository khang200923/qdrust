mod qd;
mod bot;

use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
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
    #[command(about = "Let bots battle and get their eloes", long_about = None)]
    Battle {
        #[arg(name = "bots")]
        bot_strings: Vec<String>,
        #[arg(long, default_value_t = 100)]
        num_matchups: usize,
        #[arg(short, default_value_t = 32.)]
        k: f32,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Battle { bot_strings, num_matchups, k } => {
            let bot_zip: Vec<(Option<Box<dyn Bot>>, String)> = 
            bot_strings.clone().into_iter().map(
                    |x| (map_bot_string(&x), x.clone())
                ).collect();
            for (bot, string) in &bot_zip {
                if bot.is_none() {
                    println!("\"{}\" does not exist", string);
                    ()
                }
            }
            let bots: Vec<Box<dyn Bot>> = 
                bot_zip.into_iter().map(
                    |(x, _)| x.expect("")
                ).collect();
            if bots.len() <= 1 {
                println!("You need at least 2 bots to battle");
                return;
            }
                
            let bar = ProgressBar::new(num_matchups as u64);
            bar.set_style(
                ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
                    .unwrap()
                    .progress_chars("##-"),
            );

            fn prog_func_base(_progress: usize, _num_matchups: usize, bar: &ProgressBar) {
                bar.set_message("");
                bar.inc(1);
            }

            let prog_func = Box::new(
                |progress| prog_func_base(progress, num_matchups, &bar)
            );
            let elo_scores = run_tournament(
                bots, 
                num_matchups, 
                Some(k), 
                &Some(prog_func));
            bar.finish();

            let elo_zip = bot_strings.clone().into_iter().zip(elo_scores.into_iter());
            for (bot, elo) in elo_zip {
                println!("{}: {:.0}", bot, elo);
            }
        }
    }
}