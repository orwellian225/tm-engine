use std::rc::Rc;
use std::default::Default;

use crate::{errors::TmError, util::{State, Symbol, Transition}};
use super::computation::Computation;

pub struct TuringMachine {
    states: Vec<State>,
    tape_symbols: Vec<Symbol>,
    language_symbols: Vec<Symbol>,
    transitions: Vec<Vec<Transition>>,
    start_state: usize,
    accept_state: usize,
    reject_state: usize
}

impl TuringMachine {
    pub fn compute(&self, word: &String) -> Result<Computation, TmError> {
        Computation::start(self, word)
    }

    pub fn bounded_compute(&self, word: &String, limits: (Option<usize>, Option<usize>)) -> Result<Computation, TmError> {
        Computation::bounded_start(self, word, limits)
    }
}

impl TuringMachine {
    pub fn view_states(&self) -> &Vec<State> { &self.states  }
    pub fn view_tape_symbols(&self) -> &Vec<Symbol> { &self.tape_symbols }
    pub fn view_language_symbols(&self) -> &Vec<Symbol> { &self.language_symbols }
    pub fn view_transitions(&self) -> &Vec<Vec<Transition>> { &self.transitions }
    pub fn view_start_state(&self) -> usize { self.start_state }
    pub fn view_accept_state(&self) -> usize { self.accept_state }
    pub fn view_reject_state(&self) -> usize { self.reject_state }
}

impl Default for TuringMachine {
    fn default() -> Self {
        Self {
            states: vec![
                State::new_state("q0".to_string()),
                State::new_state("q1".to_string()), 
                State::new_state("q2".to_string())
            ],
            language_symbols: vec![
                Symbol::new_symbol('0'), 
                Symbol::new_symbol('1')
            ],
            tape_symbols: vec![
                Symbol::new_symbol('_'), 
                Symbol::new_symbol('>'),
            ],
            transitions: vec![
                vec![
                    Transition::new_transition(1, 0, 1),
                    Transition::new_transition(1, 1, 1),
                    Transition::new_transition(1, 2, 1),
                    Transition::new_transition(1, 3, 1),
                ]
            ],
            start_state: 0,
            accept_state: 1,
            reject_state: 2,
        }
    }
}