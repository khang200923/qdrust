use dyn_clone::DynClone;
use crate::qd::state::GameState;

pub trait Bot: Send + Sync + DynClone {
    fn decide(&self, state: GameState) -> u8;
}

dyn_clone::clone_trait_object!(Bot);

pub fn bots_fight(a: &dyn Bot, b: &dyn Bot) -> bool {
    let mut state = GameState::def();
    while state.result() == None {
        if state.is_white_turn {
            let move_to = a.decide(state);
            state.make_move(move_to);
        } else {
            let move_to = b.decide(state);
            state.make_move(move_to);
        }
    }
    state.result().unwrap()
}

pub fn bots_fight_rand(a: &dyn Bot, b: &dyn Bot) -> bool {
    let mut state = GameState::def_rand();
    while state.result() == None {
        if state.is_white_turn {
            let move_to = a.decide(state);
            state.make_move(move_to);
        } else {
            let move_to = b.decide(state);
            state.make_move(move_to);
        }
    }
    state.result().unwrap()
}