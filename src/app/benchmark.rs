use std::io::{self, Read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};

use crate::bot::base::Bot;
use crate::bot::collections::map_bot_string;
use crate::bot::elo::run_benchmark;

enum Exception {
    InvalidBuffer
}

fn buffer_to_oppo_bots_elos(buffer: &str) -> Result<(Vec<Box<dyn Bot>>, Vec<f32>), Exception> {
    let mut oppo_bots = Vec::new();
    let mut elos = Vec::new();
    for line in buffer.lines() {
        let mut parts = line.split(':');
        let name = parts.next().ok_or(Exception::InvalidBuffer)?;
        let elo_str = parts.next().ok_or(Exception::InvalidBuffer)?;
        let elo: f32 = elo_str.trim().parse().map_err(|_| Exception::InvalidBuffer)?;

        if let Some(bot) = map_bot_string(name) {
            oppo_bots.push(bot);
            elos.push(elo);
        } else {
            return Err(Exception::InvalidBuffer);
        }
    }

    Ok((oppo_bots, elos))
}

pub fn benchmark(
    bot_string: String,
    num_matchups: usize,
    num_threads: usize,
    k_start: f32,
    k_end: f32,
) {
    let mut buffer = String::new();
    let res = io::stdin().read_to_string(&mut buffer);
    if res.is_err() {
        eprintln!("Failed to read from stdin");
        return;
    }
    let (oppo_bots, oppo_elos) = match buffer_to_oppo_bots_elos(&buffer) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Invalid input format");
            return;
        }
    };
    if oppo_bots.is_empty() {
        eprintln!("No opponent bots provided");
        return;
    }
    let bot = map_bot_string(&bot_string);
    if bot.is_none() {
        eprintln!("\"{}\" does not exist", bot_string);
        return;
    }
    let bot = bot.unwrap();

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
    let benchmark_running = Arc::new(AtomicBool::new(true));
    {
        let start = start.clone();
        let bar = bar.clone();
        let benchmark_running = benchmark_running.clone();
        thread::spawn(move || {
            while benchmark_running.load(Ordering::Relaxed) {
                thread::sleep(std::time::Duration::from_millis(10));
                let elapsed = start.elapsed();
                bar.set_message(format!("{:.3}s", elapsed.as_secs_f64()));
            }
        });
    }

    let elo = run_benchmark(
        bot,
        oppo_bots,
        num_matchups,
        oppo_elos,
        k_start,
        k_end,
        num_threads,
        &Some(prog_func)
    );
    bar.finish();
    println!("{:.0}", elo);
}