use rand::Rng;
use crate::bot::base::{
    Bot,
    bots_fight
};

fn expected_score(r1: f32, r2: f32) -> f32 {
    1.0 / (1.0 + 10f32.powf((r2 - r1) / 400.0))
}

pub fn run_tournament<'a>(
    bots: Vec<Box<dyn Bot>>,
    num_matchups: usize,
    k: Option<f32>,
    prog_func: &Option<Box<dyn Fn(usize) + 'a>>,
) -> Vec<f32> {
    let k = k.unwrap_or(32.0);
    let mut rng = rand::thread_rng();
    let mut elos = vec![0.0; bots.len()];

    for p in 0..num_matchups {
        if let Some(func) = prog_func {
            func(p);
        }

        let (i, j) = loop {
            let pair = (rng.gen_range(0..bots.len()), rng.gen_range(0..bots.len()));
            if pair.0 != pair.1 {
                break pair;
            }
        };

        let b1 = &bots[i];
        let b2 = &bots[j];

        let b1_elo = elos[i];
        let b2_elo = elos[j];

        let b1_win_prob = expected_score(b1_elo, b2_elo);
        let b2_win_prob = expected_score(b2_elo, b1_elo);

        let does_b1_win = bots_fight(b1.as_ref(), b2.as_ref());

        let b1_score = if does_b1_win { 1.0 } else { 0.0 };
        let b2_score = if does_b1_win { 0.0 } else { 1.0 };

        let b1_new = b1_elo + k * (b1_score - b1_win_prob);
        let b2_new = b2_elo + k * (b2_score - b2_win_prob);

        elos[i] = b1_new;
        elos[j] = b2_new;
    }

    elos
}