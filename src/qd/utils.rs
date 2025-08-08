use crate::qd::state::GameState;

fn u64_to_visual_bitboard(num: u64) -> String {
    let mut board = String::new();
    for rank in (0..8).rev() {
        for file in 0..8 {
            if num & (1 << (rank * 8 + file)) != 0 {
                board.push('#');
            } else {
                board.push('.');
            }
        }
        board.push('\n');
    }
    board.trim_end().to_string()
}

fn game_state_to_visual(state: &GameState) -> String {
    let mut board = String::new();
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 8 + file;
            if state.blocks & (1 << index) != 0 {
                board.push('#');
            } else if state.wqueen == index as u8 && state.bqueen == index as u8 {
                if state.is_white_turn {board.push('B');}
                else {board.push('W');}
            } else if state.wqueen == index as u8 {
                board.push('W');
            } else if state.bqueen == index as u8 {
                board.push('B');
            } else {
                board.push('.');
            }
        }
        board.push('\n');
    }
    board.trim_end().to_string()
}

fn game_state_to_visual_detailed(state: &GameState) -> String {
    let mut board = String::new();
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 8 + file;
            if state.blocks & (1 << index) != 0 {
                board.push('#');
            } else if state.wqueen == index as u8 && state.bqueen == index as u8 {
                if state.is_white_turn {board.push('B');}
                else {board.push('W');}
            } else if state.wqueen == index as u8 {
                board.push('W');
            } else if state.bqueen == index as u8 {
                board.push('B');
            } else {
                board.push('.');
            }
        }
        board.push_str(&format!(" {}\n", rank+1));
    }
    board.push_str("abcdefgh");
    board.trim_end().to_string()
}

fn visual_bitboard_to_u64(board: &str) -> u64 {
    let mut bitboard = 0u64;
    let lines = board.lines();
    let mut rank = 0;
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.len() != 8 { continue; }
        
        for (file, ch) in trimmed.chars().enumerate() {
            if ch == '#' {
                let bit_index = (7 - rank) * 8 + file;
                bitboard |= 1u64 << bit_index;
            } else {
                assert_eq!(ch, '.');
            }
        }

        rank += 1;
    }
    assert!(rank == 8);

    bitboard
}

fn visual_to_game_state(state_str: &str, is_white_turn: bool) -> GameState {
    let mut wqueen: Option<u8> = None;
    let mut bqueen: Option<u8> = None;
    let mut blocks: u64 = 0;
    let lines = state_str.lines();
    let mut rank = 0;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.len() != 8 { continue; }

        for (file, ch) in trimmed.chars().enumerate() {
            let bit_index: u8 = (7 - rank) * 8 + file as u8;
            if ch == '#' {
                blocks |= 1u64 << bit_index;
            } else if ch == 'W' {
                assert_eq!(wqueen, None);
                wqueen = Some(bit_index);
            } else if ch == 'B' {
                assert_eq!(bqueen, None);
                bqueen = Some(bit_index);
            } else {
                assert_eq!(ch, '.');
            }
        }

        rank += 1;
    }
    assert!(rank == 8);
    assert_ne!(wqueen, None);
    assert_ne!(bqueen, None);

    GameState::new(wqueen, bqueen, Some(blocks), Some(is_white_turn))
}

pub fn vbb(board: &str) -> u64 {visual_bitboard_to_u64(board)}
pub fn vgs(state_str: &str, is_white_turn: bool) -> GameState {visual_to_game_state(state_str, is_white_turn)}
pub fn bbv(num: u64) -> String {u64_to_visual_bitboard(num)}
pub fn gsv(state: &GameState) -> String {game_state_to_visual(state)}
pub fn gsvd(state: &GameState) -> String {game_state_to_visual_detailed(state)}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_vbb_1() {
        let board = "
            ........
            ........
            ........
            ........
            ........
            ........
            ........
            ....#...
        ";
        assert_eq!(vbb(board), 1u64 << 4);
    }
    
    #[test]
    fn test_vbb_2() {
        let board = "
            ........
            ........
            ........
            .......#
            #.......
            ........
            ........
            ........
        ";
        assert_eq!(vbb(board), (1u64 << (3*8)) + (1u64 << (4*8+7)));
    }

    #[test]
    #[should_panic]
    fn test_vbb_3() {
        let board = "
        some gibberish which 
        is definitely not a board
        ";
        vbb(board);
    }

    #[test]
    fn test_vgs_1() {
        let state_str = "
            ...B....
            ........
            ........
            ........
            ........
            ........
            ........
            ....W...
        ";
        let game_state = vgs(state_str, true);
        assert_eq!(game_state, GameState::def());
    }

    #[test]
    #[should_panic]
    fn test_vgs_2() {
        let state_str = "
            ...B....
            ........
            ........
            ........
            ........
            ........
            ........
            ....W..
        ";
        vgs(state_str, true);
    }

    #[test]
    fn test_bbv() {
        let mut rng = rand::thread_rng();
        for _i in 0..100 {
            let test_val: u64 = rng.r#gen();
            assert_eq!(test_val, vbb(&bbv(test_val)));
        }
    }

    #[test]
    fn test_gsv() {
        let mut rng = rand::thread_rng();
        for _i in 0..100 {
            let wqueen: u8 = rng.gen_range(0..64);
            let bqueen: u8 = rng.gen_range(0..64);
            if wqueen == bqueen {continue}
            let blocks: u64 = rng.r#gen();
            let blocks: u64 = blocks & (!(1u64 << wqueen)) & (!(1u64 << bqueen));
            let is_white_turn: bool = rng.r#gen();
            let test_state: GameState = GameState::new(
                Some(wqueen),
                Some(bqueen),
                Some(blocks),
                Some(is_white_turn)
            );
            assert_eq!(test_state, vgs(&gsv(&test_state), test_state.is_white_turn));
        }
    }
}