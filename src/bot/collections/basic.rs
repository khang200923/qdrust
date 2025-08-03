use crate::bot::base::Bot;
use crate::qd::state::{GameState};
use crate::qd::legalcomp::{get_possible_legal_moves};

const INFINITY: f32 = 1e6;

pub struct BasicBot;

fn heuristic(state: &GameState) -> f32 {
    if state.result() == Some(true) {
        return INFINITY;
    }
    if state.result() == Some(false) {
        return -INFINITY;
    }
    let score = get_possible_legal_moves(state).count_ones();
    let oppo_state = GameState::new(
        Some(state.wqueen),
        Some(state.bqueen),
        Some(state.blocks),
        Some(!state.is_white_turn),
    );
    let oppo_score = get_possible_legal_moves(&oppo_state).count_ones();
    score as f32 - oppo_score as f32
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
    depth: u32, 
    alpha: f32, beta: f32,
    ab_pruning: bool
) -> (f32, Option<u8>) {
    if depth == 0 || state.result().is_some() {
        return (heuristic(state), None);
    }

    let mut alpha = alpha;
    let mut beta = beta;

    let mut best_value = if state.is_white_turn { -INFINITY } else { INFINITY };
    let mut best_move = None;

    for (child, move_made) in get_children(state) {
        let (value, _) = minimax_local(&child, depth - 1, alpha, beta, ab_pruning);
        let value = if value > 0. { value - 0.01 } else { value + 0.01 };
        if state.is_white_turn {
            if value >= best_value {
                best_value = value;
                best_move = Some(move_made);
            }
            alpha = alpha.max(value);
        } else {
            if value <= best_value {
                best_value = value;
                best_move = Some(move_made);
            }
            beta = beta.min(value);
        }
        if beta <= alpha && ab_pruning {
            break;
        }
    }

    (best_value, best_move)
}

fn minimax(state: &GameState, depth: u32) -> (f32, Option<u8>) {
    minimax_local(state, depth, -INFINITY, INFINITY, true)
}

impl Bot for BasicBot {
    fn decide(&self, state: GameState) -> u8 {
        let (_, best_move) = minimax(&state, 3);
        assert!(best_move.is_some());
        best_move.unwrap()
    }
}