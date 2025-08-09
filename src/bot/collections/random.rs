use rand::Rng;
use crate::bot::base::Bot;
use crate::qd::state::GameState;
use crate::qd::legalcomp::{get_possible_legal_moves};

#[derive(Clone)]
pub struct RandomBot {}

fn random_set_bit_index(bitboard: u64) -> Option<u32> {
    let popcnt = bitboard.count_ones();
    if popcnt == 0 {
        return None;
    }

    let k = rand::thread_rng().gen_range(0..popcnt);

    let mut n = bitboard;
    for _ in 0..k {
        assert_ne!(n, 0);
        n &= n - 1;
    }

    Some(n.trailing_zeros())
}

impl RandomBot {
    pub fn new() -> Self {
        Self {}
    }
}

impl Bot for RandomBot {
    fn decide(&self, state: GameState) -> u8 {
        let legal_moves = get_possible_legal_moves(&state);

        assert_ne!(legal_moves, 0);

        let move_index = random_set_bit_index(legal_moves);
        move_index.expect("No legal moves available");
        let move_index = move_index.unwrap();

        let move_to = move_index as u8;

        move_to
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_legal() {
        for _ in 0..1000 {
            let mut state = GameState::def();
            let bot = RandomBot::new();

            while let None = state.result() {
                let move_to = bot.decide(state);
                state.make_move(move_to);
            }

            assert!(state.result().is_some());
        }
    }
}