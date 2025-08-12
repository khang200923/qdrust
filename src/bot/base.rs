use rand::{thread_rng, Rng};
use dyn_clone::DynClone;
use crate::qd::state::GameState;

pub trait Bot: Send + Sync + DynClone {
    fn decide(&self, state: GameState) -> u8;
}

dyn_clone::clone_trait_object!(Bot);

pub fn bots_fight_rand(a: &dyn Bot, b: &dyn Bot) -> bool {
    let mut rng = thread_rng();
    let flip = rng.gen_bool(0.5);
    let (white, black) = if flip { (b, a) } else { (a, b) };
    let mut state = GameState::def_rand();
    while state.result() == None {
        if state.is_white_turn {
            let move_to = white.decide(state);
            state.make_move(move_to);
        } else {
            let move_to = black.decide(state);
            state.make_move(move_to);
        }
    }
    state.result().unwrap() != flip
}