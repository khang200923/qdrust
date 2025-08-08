use crate::qd::state::{GameState};
use std::sync::Mutex;

static POSSIBLE_ATTACK_MASKS: Mutex<[u64; 64]> = Mutex::new([0; 64]);

pub fn get_possible_attack_mask_slow(queen: u8) -> u64 {
    let mut mask: u64 = 0;
    let row = queen / 8;
    let col = queen % 8;

    for i in 0..8 {
        mask |= 1 << (row * 8 + i);
        mask |= 1 << (i * 8 + col);
    }

    for dr in [-1i8, 1i8] {
        for dc in [-1i8, 1i8] {
            for i in 1..8 {
                let r = row as i8 + i * dr;
                let c = col as i8 + i * dc;
                if r >= 0 && r < 8 && c >= 0 && c < 8 {
                    mask |= 1 << (r as u8 * 8 + c as u8);
                } else {
                    break;
                }
            }
        }
    }

    mask &= !(1 << queen);

    mask
}

pub fn get_possible_attack_mask(queen: u8) -> u64 {
    let mut pcomp = POSSIBLE_ATTACK_MASKS.lock().unwrap();
    if pcomp[queen as usize] == 0 {
        pcomp[queen as usize] = get_possible_attack_mask_slow(queen);
    }
    pcomp[queen as usize]
}

pub fn get_possible_legal_moves_info_slow(
    squeen: u8,
    oqueen: u8,
    blocks: u64
) -> u64 {
    let mask = get_possible_attack_mask(squeen);
    let occupancy = blocks | (1 << oqueen);

    let row = squeen / 8;
    let col = squeen % 8;

    let mut res: u64 = 0;

    for dr in -1i8..=1i8 {
        for dc in -1i8..=1i8 {
            if dr == 0 && dc == 0 {continue}
            for i in 1..8 {
                let r = row as i8 + i * dr;
                let c = col as i8 + i * dc;
                let index = r * 8 + c;
                if !(r >= 0 && r < 8 && c >= 0 && c < 8) {
                    break;
                }
                if occupancy & (1 << index) != 0 {
                    if index == oqueen as i8 {
                        res |= 1 << index;
                    }
                    break;
                }
                res |= 1 << index;
            }
        }
    }

    res
}

pub fn get_possible_legal_moves_info(
    squeen: u8,
    oqueen: u8,
    blocks: u64
) -> u64 {
    // TODO: magic
    get_possible_legal_moves_info_slow(squeen, oqueen, blocks)
}

pub fn get_possible_legal_moves(
    state: &GameState
) -> u64 {
    if state.is_white_turn {
        get_possible_legal_moves_info(
            state.wqueen,
            state.bqueen,
            state.blocks
        )
    } else {
        get_possible_legal_moves_info(
            state.bqueen,
            state.wqueen,
            state.blocks
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qd::utils::*;

    #[test]
    fn test_get_possible_attack_mask() {
        let mask = get_possible_attack_mask(4);
        assert_eq!(mask, vbb("
            ....#...
            ....#...
            ....#...
            #...#...
            .#..#..#
            ..#.#.#.
            ...###..
            ####.###
        "));
        let mask = get_possible_attack_mask(59);
        assert_eq!(mask, vbb("
            ###.####
            ..###...
            .#.#.#..
            #..#..#.
            ...#...#
            ...#....
            ...#....
            ...#....
        "));
        let mask = get_possible_attack_mask(8 * 3 + 5);
        assert_eq!(mask, vbb("
            .#...#..
            ..#..#..
            ...#.#.#
            ....###.
            #####.##
            ....###.
            ...#.#.#
            ..#..#..
        "))
    }

    #[test]
    fn test_get_possible_legal_moves_info_slow_1() {
        let squeen = 4;
        let oqueen = 59;
        let blocks = 0;

        let res = get_possible_legal_moves_info_slow(squeen, oqueen, blocks);
        assert_eq!(res, vbb("
            ....#...
            ....#...
            ....#...
            #...#...
            .#..#..#
            ..#.#.#.
            ...###..
            ####.###
        "));
        let res = get_possible_legal_moves_info_slow(oqueen, squeen, blocks);
        assert_eq!(res, vbb("
            ###.####
            ..###...
            .#.#.#..
            #..#..#.
            ...#...#
            ...#....
            ...#....
            ...#....
        "))
    }

    #[test]
    fn test_get_possible_legal_moves_info_slow_2() {
        let squeen = 8 * 3 + 5;
        let oqueen = 0;
        let blocks = vbb("
            ........
            ........
            ........
            .....#..
            ..#.....
            ........
            ........
            .....#..
        ");

        let res = get_possible_legal_moves_info_slow(squeen, oqueen, blocks);
        assert_eq!(res, vbb("
        .#......
        ..#.....
        ...#...#
        ....#.#.
        ...##.##
        ....###.
        ...#.#.#
        ..#.....
        "));

        let squeen = 8 * 3 + 5;
        let oqueen = 8 * 3 + 1;
        let blocks = vbb("
            .....#..
            ........
            ........
            ....#...
            ........
            ........
            ...#....
            .....#..
        ");

        let res = get_possible_legal_moves_info_slow(squeen, oqueen, blocks);
        assert_eq!(res, vbb("
            ........
            .....#..
            .....#.#
            .....##.
            .####.##
            ....###.
            .....#.#
            ........
        "))
    }
}