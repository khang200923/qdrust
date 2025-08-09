use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use crate::bot::base::Bot;
use crate::bot::collections::map_bot_string;
use crate::bot::elo::run_tournament;


pub fn battle(
    bot_strings: Vec<String>, 
    num_matchups: usize, 
    num_threads: usize, 
    k: f32
) {
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
}