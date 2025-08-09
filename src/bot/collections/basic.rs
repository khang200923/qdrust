use crate::bot::base::Bot;
use crate::qd::state::{GameState};
use crate::qd::legalcomp::{get_possible_attack_mask, get_possible_legal_moves};

const INFINITY: f32 = 1e6;

#[derive(Clone)]
pub struct BasicBot {
    depth: u32,
}

fn queens_in_reach(state: &GameState) -> bool {
    if (get_possible_attack_mask(state.wqueen) | get_possible_attack_mask(state.bqueen))
    & (1u64 << state.wqueen | 1u64 << state.bqueen) != 0 {
        if get_possible_legal_moves(state) & (1u64 << state.wqueen | 1u64 << state.bqueen) != 0 { return true; }
    }
    false
}

fn use_heuristic(state: &GameState) -> bool {
    if state.result().is_some() { return true; }
    if queens_in_reach(state) { return true; }
    false
}

fn heuristic(state: &GameState) -> f32 {
    if state.result() == Some(true) {
        return INFINITY;
    }
    if state.result() == Some(false) {
        return -INFINITY;
    }
    if queens_in_reach(state) {
        if state.is_white_turn { return INFINITY; } else { return -INFINITY; }
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
    depth: u32, 
    alpha: f32, beta: f32,
    ab_pruning: bool,
    top_level: bool
) -> (f32, Option<u8>, bool) {
    if (depth == 0 || use_heuristic(state)) && !top_level {
        // println!("{}: dbgg {}", depth, heuristic(state));
        return (heuristic(state), None, false);
    }

    assert!((!top_level) || depth > 0);

    let mut alpha = alpha;
    let mut beta = beta;

    let mut best_value = if state.is_white_turn { -INFINITY } else { INFINITY };
    let mut best_move = None;
    let mut best_move_unpruned = None;
    let mut pruned = false;

    for (child, move_made) in get_children(state) {
        let (mut value, _, eval_pruned) 
            = minimax_local(&child, depth - 1, alpha, beta, ab_pruning, false);
        if value > 0. { value -= 0.01 } else { value += 0.01 }
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
        if beta <= alpha && ab_pruning {
            pruned = true;
            break;
        }
    }

    if best_move.is_none() {
        best_move = best_move_unpruned;
    }

    (best_value, best_move, pruned)
}

fn minimax(state: &GameState, depth: u32) -> (f32, Option<u8>) {
    let (best_value, best_move, _) = 
        minimax_local(state, depth, -INFINITY, INFINITY, true, true);
    (best_value, best_move)
}

impl BasicBot {
    pub fn new(depth: u32) -> Self {
        Self { depth: depth }
    }
}

impl Bot for BasicBot {
    fn decide(&self, state: GameState) -> u8 {
        assert!(state.result().is_none());
        assert!(self.depth > 0);
        let (_, best_move) = minimax(&state, self.depth);
        best_move.unwrap()
    }
}