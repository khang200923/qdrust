use rand::Rng;
use crate::qd::legalcomp::{
    get_possible_legal_moves
};

#[derive(PartialEq)]
#[derive(Debug, Eq)]
#[derive(Copy, Clone)]
pub struct GameState {
    pub wqueen: u8,
    pub bqueen: u8,
    pub blocks: u64,
    pub is_white_turn: bool,
}

impl GameState {
    pub fn new(
        wqueen: Option<u8>,
        bqueen: Option<u8>,
        blocks: Option<u64>,
        is_white_turn: Option<bool>,
    ) -> Self {
        Self {
            wqueen: wqueen.unwrap_or(4),
            bqueen: bqueen.unwrap_or(59),
            blocks: blocks.unwrap_or(0),
            is_white_turn: is_white_turn.unwrap_or(true),
        }
    }

    pub fn def() -> Self {
        Self::new(None, None, None, None)
    }

    pub fn def_rand() -> Self {
        let mut blocks: u64 = 0;
        let mut rng = rand::thread_rng();
        for _ in 0..7 {
            blocks |= 1 << rng.gen_range(0..64);
        }
        blocks &= !(1u64 << 4) & !(1u64 << 59);
        Self::new(Some(4), Some(59), Some(blocks), Some(true))
    }

    pub fn make_move(&mut self, to: u8) {
        assert!(to < 64);
        assert_ne!(get_possible_legal_moves(self) & (1 << to), 0);
        
        if self.is_white_turn {
            self.blocks |= 1 << self.wqueen;
            self.wqueen = to;
        } else {
            self.blocks |= 1 << self.bqueen;
            self.bqueen = to;
        }
        self.is_white_turn = !self.is_white_turn;
    }

    pub fn result(&self) -> Option<bool> {
        if self.wqueen == self.bqueen {
            return Some(!self.is_white_turn)
        }
        if get_possible_legal_moves(self) == 0 {
            return Some(!self.is_white_turn)
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qd::utils::*;

    #[test]
    fn test_game_state() {
        let mut state = GameState::def();
        assert_eq!(state.wqueen, 4);
        assert_eq!(state.bqueen, 59);
        assert_eq!(state.blocks, 0);
        assert!(state.is_white_turn);

        state.make_move(5);
        assert_eq!(state.wqueen, 5);
        assert_eq!(state.bqueen, 59);
        assert_eq!(state.blocks, 1 << 4);
        assert!(!state.is_white_turn);

        state.make_move(58);
        assert_eq!(state.wqueen, 5);
        assert_eq!(state.bqueen, 58);
        assert_eq!(state.blocks, (1 << 4) | (1 << 59));
        assert!(state.is_white_turn);

        state.make_move(6);
        assert_eq!(state.wqueen, 6);
        assert_eq!(state.bqueen, 58);
    }

    #[test]
    fn test_game_result() {
        let mut state = GameState::def();
        assert_eq!(state.result(), None);

        state.make_move(3);
        assert_eq!(state.result(), None);

        state.make_move(58);
        assert_eq!(state.result(), None);

        state.make_move(2);
        assert_eq!(state.result(), None);

        state.make_move(2);
        assert_eq!(state.result(), Some(false));
    }

    #[test]
    #[should_panic]
    fn test_illegal_move_1() {
        let mut state = GameState::def();
        state.make_move(64);
    }

    #[test]
    #[should_panic]
    fn test_illegal_move_2() {
        let mut state = GameState::def();
        state.blocks = vbb("
            ....#...
            ...#....
            ........
            ........
            ..#.....
            ..#.....
            .##...#.
            ........
        ");
        state.make_move(8 * 7 + 4);
    }
}