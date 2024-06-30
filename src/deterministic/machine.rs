//! # Deterministic Turing Machines
//! 
//! Inspired by the 'Introduction to the Theory of Computation' by Micheal Sipser
//! 
//! Turing Machines are mathematical objects that can be used to describe any computation algorithms.
//! 
//! $$
//! TM = \left< Q, \Delta, \Gamma, \delta, q_0, q_1, q_2 \right> 
//! $$
//! 
//! * $Q$ - The set of all states in the Turing Machine
//! * $\Delta$ - The set of symbols used in the problem language
//! * $\Gamma$ - The set of tape symbols and language symbols
//! * $\delta$ - The set of transitions $\delta_{ij}: Q \times \Gamma \mapsto Q \times \Gamma \times {-1,1}

use crate::{errors::TmError, util::{State, Symbol, Transition}};
use super::computation::Computation;

/// The TM 7-Tuple definition found in Sipser
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
    /// Construct an unbounded computation using the current machine and a specified word
    pub fn compute(&self, word: &String) -> Result<Computation, TmError> {
        Computation::start(self, word)
    }

    /// Construct a bounded computation using the current machine and a specified word
    pub fn bounded_compute(&self, word: &String, limits: (Option<usize>, Option<usize>)) -> Result<Computation, TmError> {
        Computation::bounded_start(self, word, limits)
    }
}

impl TuringMachine {
    pub fn new(states: Vec<State>, language_symbols: Vec<Symbol>, tape_symbols: Vec<Symbol>, transitions: Vec<Vec<Transition>>, start_state: usize, accept_state: usize, reject_state: usize) -> Self {
        Self {
            states, tape_symbols, language_symbols, transitions, start_state, accept_state, reject_state
        }
    }

    pub fn view_states(&self) -> &Vec<State> { &self.states }
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