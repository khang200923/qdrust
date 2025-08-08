use crate::bot::base::Bot;
use crate::qd::state::{GameState};
use crate::qd::legalcomp::{get_possible_legal_moves};

const INFINITY: f32 = 1e6;

#[derive(Clone)]
pub struct AdaptiveBot {
    max_compute: u64
}

fn heuristic(state: &GameState) -> f32 {
    if state.result() == Some(true) {
        return INFINITY;
    }
    if state.result() == Some(false) {
        return -INFINITY;
    }
    let white_state = GameState::new(
        Some(state.wqueen),
        Some(state.bqueen),
        Some(state.blocks),
        Some(true),
    );
    let black_state = GameState::new(
        Some(state.wqueen),
        Some(state.bqueen),
        Some(state.blocks),
        Some(false),
    );
    let white_score = get_possible_legal_moves(&white_state).count_ones();
    let black_score = get_possible_legal_moves(&black_state).count_ones();
    white_score as f32 - black_score as f32
}

fn get_children(state: &GameState) -> Vec<(GameState, u8)> {
    let legal_moves = get_possible_legal_moves(state);
    let mut children = Vec::new();
    for i in 0..64 {
        if (legal_moves >> i) & 1 == 1 {
            let mut child_state = state.clone();
            child_state.make_move(i as u8);
            children.push((child_state, i as u8));
        }
    }
    children
}

fn minimax_local(
    state: &GameState, 
    max_compute: u64, 
    alpha: f32, beta: f32
) -> (f32, Option<u8>, u64, bool) {
    if max_compute <= 1 || state.result().is_some() {
        return (heuristic(state), None, max_compute, false)
    }

    let mut alpha = alpha;
    let mut beta = beta;

    let mut best_value = if state.is_white_turn { -INFINITY } else { INFINITY };
    let mut best_move = None;
    let mut best_move_unpruned = None;
    let mut pruned = false;

    let mut remaining = max_compute;

    let mut children = get_children(state);

    while let Some((child, move_made)) = children.pop() {
        let max_cost = remaining / (children.len() as u64 + 1);
        let (mut value, _, cost, eval_pruned) 
            = minimax_local(&child, max_cost, alpha, beta);
        remaining -= cost;
        if state.is_white_turn {
            if value >= best_value {
                best_value = value;
                if !eval_pruned { best_move = Some(move_made) };
                best_move_unpruned = Some(move_made);
            }
            alpha = alpha.max(value);
        } else {
            if value <= best_value {
                best_value = value;
                if !eval_pruned { best_move = Some(move_made) };
                best_move_unpruned = Some(move_made);
            }
            beta = beta.min(value);
        }
        if beta <= alpha {
            pruned = true;
            break;
        }
    }

    if best_move.is_none() {
        best_move = best_move_unpruned;
    }

    (best_value, best_move, max_compute - remaining, pruned)
}

fn minimax(state: &GameState, max_compute: u64) -> (f32, Option<u8>, u64) {
    let (best_value, best_move, remaining, _) = minimax_local(state, max_compute, -INFINITY, INFINITY);
    (best_value, best_move, remaining)
}

impl AdaptiveBot {
    pub fn new(max_compute: u64) -> Self {
        Self { max_compute }
    }
}

impl Bot for AdaptiveBot {
    fn decide(&self, state: GameState) -> u8 {
        let (_, best_move, _) = minimax(&state, self.max_compute);
        best_move.unwrap()
    }
}