use rand::Rng;
use std::sync::{
    Arc
};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use crate::bot::base::{
    Bot,
    bots_fight_rand
};

fn expected_score(r1: f32, r2: f32) -> f32 {
    1.0 / (1.0 + 10f32.powf((r2 - r1) / 400.0))
}

fn matchable_elos(r1: f32, r2: f32) -> bool {
    let diff = (r1 - r2).abs();
    diff < 800.0
}

pub fn run_tournament<'a>(
    bots: Vec<Box<dyn Bot>>,
    num_matchups: usize,
    k: Option<f32>,
    num_threads: usize,
    prog_func: &Option<Box<dyn Fn(usize) + 'a>>
) -> Vec<f32> {
    let k = k.unwrap_or(32.0);
    let bots = Arc::new(bots);
    let mut elos = vec![0.0f32; bots.len()];
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();
    let mut remaining = num_matchups;

    let matchup_func = |elos: &Vec<f32>| {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..bots.len());
        let mut chosen_bots = (0..bots.len())
            .filter(
                |j| {
                    let elo_i = (elos)[i];
                    let elo_j = (elos)[*j];
                    *j != i && matchable_elos(elo_i, elo_j)
                }
            )
            .collect::<Vec<_>>();
        if chosen_bots.len() == 0 {
            chosen_bots = (0..bots.len())
                .filter(|j| *j != i)
                .collect::<Vec<_>>();
        }
        let j_chosen = rng.gen_range(0..chosen_bots.len());
        let j = chosen_bots[j_chosen];
        (i, j)
    };

    while remaining > 0 {
        let mut inp = vec![(0usize, 0usize); 0];
        let mut remaining_batch = num_threads * 4;
        while remaining_batch > 0 && remaining > 0 {
            inp.push(matchup_func(&elos));
            remaining_batch -= 1;
            remaining -= 1;
        };
        let out = pool.install(|| {
            inp.into_par_iter()
                .map(|(i, j)| {
                    let b1 = &bots[i];
                    let b2 = &bots[j];
                    (i, j, bots_fight_rand(b1.as_ref(), b2.as_ref()))
                })
                .collect::<Vec<_>>()
        });
        for (i, j, does_b1_win) in out {
            let b1_elo = elos[i];
            let b2_elo = elos[j];

            let b1_win_prob = expected_score(b1_elo, b2_elo);
            let b2_win_prob = expected_score(b2_elo, b1_elo);

            let b1_score = if does_b1_win { 1.0 } else { 0.0 };
            let b2_score = if does_b1_win { 0.0 } else { 1.0 };

            let b1_new = b1_elo + k * (b1_score - b1_win_prob);
            let b2_new = b2_elo + k * (b2_score - b2_win_prob);

            elos[i] = b1_new;
            elos[j] = b2_new;
        }
        if prog_func.is_some() {
            prog_func.as_ref().unwrap()(num_threads * 4);
        }
    }

    elos
}