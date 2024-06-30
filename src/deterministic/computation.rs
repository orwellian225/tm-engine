use crate::errors::TmError;
use crate::util::{ ComputationStatus, ComputationClock, Symbol, State, Transition };
use super::machine::TuringMachine;


pub struct Computation<'a> {
    machine: &'a TuringMachine,
    current_state: usize,
    head_position: usize,
    tape: Vec<usize>,
    status: ComputationStatus,
    clock: ComputationClock
}

impl<'a> Computation<'a> {
    pub fn start(machine: &'a TuringMachine, word: &String) -> Result<Self, TmError> {
        Self::bounded_start(machine, word, (None, None))
    }

    pub fn bounded_start(machine: &'a TuringMachine, word: &String, limits: (Option<usize>, Option<usize>)) -> Result<Self, TmError> {
        let mut tape = vec![1];

        for (i, w) in word.chars().enumerate() {
            let symbol = Symbol::new_symbol(w);
            let index = match machine.view_language_symbols().iter().position(|x| { x == &symbol }) {
                Some(val) => val + machine.view_tape_symbols().len(),
                None => match machine.view_tape_symbols().iter().position(|x| { x == &symbol }) {
                    Some(val) => val,
                    None => return Err(TmError::new(
                        format!("Invalid symbol {} found at position {}.", w, i)
                    ))
                }
            };

            tape.push(index);
        }

        Ok(Computation {
            machine,
            current_state: machine.view_start_state(),
            head_position: 0,
            status: ComputationStatus::Executing,
            clock: ComputationClock::clock(limits.0, limits.1, tape.len()),
            tape
        })
    }

    pub fn step(&mut self) {
        match self.status {
            ComputationStatus::Executing => (),
            _ => return
        }

        self.clock.time += 1;
        match self.clock.max_time {
            Some(limit) => if self.clock.time >= limit {
                self.status = ComputationStatus::Timeout;
                return;
            },
            None => ()
        }

        let read_symbol = self.tape[self.head_position];
        let transition = &self.machine.view_transitions()[self.current_state][read_symbol];

        let new_head_position = match transition.direction().is_negative() {
            false => match self.head_position.checked_add(transition.direction() as usize) {
                Some(val) => val,
                None => panic!("Head position exceded max usize")
            },
            true => match self.head_position.checked_sub(transition.direction().wrapping_abs() as usize) {
                Some(val) => val,
                None => 0
            }
        };

        self.current_state = transition.next_state_id();
        self.tape[self.head_position] = transition.write_symbol_id();
        self.head_position = new_head_position;

        if self.head_position >= self.tape.len() {
            self.tape.push(0 as usize);
            self.clock.space += 1;

            match self.clock.max_space {
                Some(limit) => if self.clock.space >= limit {
                    self.status = ComputationStatus::Spaceout;
                    return;
                },
                None => ()
            }
        }

        if transition.next_state_id() == self.machine.view_accept_state() {
            self.status = ComputationStatus::Accept;
            return;
        } else if transition.next_state_id() == self.machine.view_reject_state() {
            self.status = ComputationStatus::Reject;
            return;
        }
    }

    pub fn run(&mut self) {
        while self.status == ComputationStatus::Executing {
            self.step();
        }
    }
}