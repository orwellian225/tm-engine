pub mod errors;
pub mod deterministic;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::util::ComputationStatus;

    use super::deterministic::machine::TuringMachine;
    use super::deterministic::computation::Computation;
    use super::util::{ State, Symbol, Transition };

    #[test]
    fn default_tm() {
        let machine = TuringMachine::default();

        for i in 0..3 {
            assert_eq!(machine.view_states()[i], State::new_state(format!("q{i}")));
        }

        assert_eq!(machine.view_tape_symbols()[0], Symbol::new_symbol('_'));
        assert_eq!(machine.view_tape_symbols()[1], Symbol::new_symbol('>'));
        assert_eq!(machine.view_language_symbols()[0], Symbol::new_symbol('0'));
        assert_eq!(machine.view_language_symbols()[1], Symbol::new_symbol('1'));

        assert_eq!(machine.view_start_state(), 0);
        assert_eq!(machine.view_accept_state(), 1);
        assert_eq!(machine.view_reject_state(), 2);

        assert_eq!(machine.view_transitions()[0][0], Transition::new_transition(1, 0, 1));
        assert_eq!(machine.view_transitions()[0][1], Transition::new_transition(1, 1, 1));
        assert_eq!(machine.view_transitions()[0][2], Transition::new_transition(1, 2, 1));
        assert_eq!(machine.view_transitions()[0][3], Transition::new_transition(1, 3, 1));
    }

    #[test]
    fn computation_on_valid_word() {
        let machine = TuringMachine::default();
        let mut computation = match Computation::start(&machine, &"".to_string()) {
            Ok(val) => val,
            Err(e) => panic!("Test failed with message: {}", e.message)
        };
        computation.step();

        assert_eq!(computation.view_status(), &ComputationStatus::Accept);
    }

    #[test]
    fn computation_on_invalid_word() {
        let machine = TuringMachine::default();
        let _ = match Computation::start(&machine, &"2".to_string()) {
            Ok(_) => panic!("Test failed with no thrown on error"),
            Err(_) => ()
        };
    }

    #[test]
    fn last_symbol_accept() {
        let machine = TuringMachine {
            states: vec![
                State::new_state("start".to_string()),
                State::new_state("accept".to_string()),
                State::new_state("reject".to_string()),
                State::new_state("iterate_to_end".to_string()),
                State::new_state("last_symbol_check".to_string()),
            ],
            transitions: vec![
                vec![
                    Transition::new_transition(3, 0, 1),
                    Transition::new_transition(3, 1, 1),
                    Transition::new_transition(3, 2, 1),
                    Transition::new_transition(3, 3, 1),
                ],
                vec![], // accept transitions
                vec![], // reject transitions
                vec![
                    Transition::new_transition(4, 0, -1),
                    Transition::new_transition(3, 1, 1),
                    Transition::new_transition(3, 2, 1),
                    Transition::new_transition(3, 3, 1),
                ],
                vec![
                    Transition::new_transition(2, 0, 1),
                    Transition::new_transition(2, 1, 1),
                    Transition::new_transition(1, 2, 1), // accept if last symbol is 0
                    Transition::new_transition(2, 3, 1), // reject if last symbol is 1
                ],
            ],
            ..TuringMachine::default()
        };

        let accepted_computation_result = machine.compute(&"1110".to_string());

        let mut accept_computation = match accepted_computation_result {
            Ok(computation) => computation,
            Err(_) => panic!("Error in accepted computation")
        };

        accept_computation.run();

        assert_eq!(accept_computation.view_status(), &ComputationStatus::Accept);
        assert_eq!(accept_computation.view_tape().len(), 6);
        assert_eq!(accept_computation.view_tape(), &vec![1, 3, 3, 3, 2, 0]);
        assert_eq!(accept_computation.view_head_position(), 5);
        assert_eq!(accept_computation.view_clock().time, 7);
        assert_eq!(accept_computation.view_clock().space, 6);
    }

    #[test]
    fn last_symbol_reject() {
        let machine = TuringMachine {
            states: vec![
                State::new_state("start".to_string()),
                State::new_state("accept".to_string()),
                State::new_state("reject".to_string()),
                State::new_state("iterate_to_end".to_string()),
                State::new_state("last_symbol_check".to_string()),
            ],
            transitions: vec![
                vec![
                    Transition::new_transition(3, 0, 1),
                    Transition::new_transition(3, 1, 1),
                    Transition::new_transition(3, 2, 1),
                    Transition::new_transition(3, 3, 1),
                ],
                vec![], // accept transitions
                vec![], // reject transitions
                vec![
                    Transition::new_transition(4, 0, -1),
                    Transition::new_transition(3, 1, 1),
                    Transition::new_transition(3, 2, 1),
                    Transition::new_transition(3, 3, 1),
                ],
                vec![
                    Transition::new_transition(2, 0, 1),
                    Transition::new_transition(2, 1, 1),
                    Transition::new_transition(1, 2, 1), // accept if last symbol is 0
                    Transition::new_transition(2, 3, 1), // reject if last symbol is 1
                ],
            ],
            ..TuringMachine::default()
        };

        let accepted_computation_result = machine.compute(&"0001".to_string());

        let mut accept_computation = match accepted_computation_result {
            Ok(computation) => computation,
            Err(_) => panic!("Error in accepted computation")
        };

        accept_computation.run();

        assert_eq!(accept_computation.view_status(), &ComputationStatus::Reject);
        assert_eq!(accept_computation.view_tape().len(), 6);
        assert_eq!(accept_computation.view_tape(), &vec![1, 2, 2, 2, 3, 0]);
        assert_eq!(accept_computation.view_head_position(), 5);
        assert_eq!(accept_computation.view_clock().time, 7);
        assert_eq!(accept_computation.view_clock().space, 6);
    }

    #[test]
    fn last_symbol_timeout() {
        let machine = TuringMachine {
            states: vec![
                State::new_state("start".to_string()),
                State::new_state("accept".to_string()),
                State::new_state("reject".to_string()),
                State::new_state("iterate_to_end".to_string()),
                State::new_state("last_symbol_check".to_string()),
            ],
            transitions: vec![
                vec![
                    Transition::new_transition(3, 0, 1),
                    Transition::new_transition(3, 1, 1),
                    Transition::new_transition(3, 2, 1),
                    Transition::new_transition(3, 3, 1),
                ],
                vec![], // accept transitions
                vec![], // reject transitions
                vec![
                    Transition::new_transition(4, 0, -1),
                    Transition::new_transition(3, 1, 1),
                    Transition::new_transition(3, 2, 1),
                    Transition::new_transition(3, 3, 1),
                ],
                vec![
                    Transition::new_transition(2, 0, 1),
                    Transition::new_transition(2, 1, 1),
                    Transition::new_transition(1, 2, 1), // accept if last symbol is 0
                    Transition::new_transition(2, 3, 1), // reject if last symbol is 1
                ],
            ],
            ..TuringMachine::default()
        };

        let timeout_computation_result = machine.bounded_compute(&"1110".to_string(), (Some(3), None));

        let mut timeout_computation = match timeout_computation_result {
            Ok(computation) => computation,
            Err(_) => panic!("Error in accepted computation")
        };

        timeout_computation.run();

        assert_eq!(timeout_computation.view_status(), &ComputationStatus::Timeout);
        assert_eq!(timeout_computation.view_tape().len(), 5);
        assert_eq!(timeout_computation.view_tape(), &vec![1, 3, 3, 3, 2]);
        assert_eq!(timeout_computation.view_head_position(), 2);
        assert_eq!(timeout_computation.view_clock().time, 3);
    }

    #[test]
    fn last_symbol_spaceout() {
        let machine = TuringMachine {
            states: vec![
                State::new_state("start".to_string()),
                State::new_state("accept".to_string()),
                State::new_state("reject".to_string()),
                State::new_state("iterate_to_end".to_string()),
                State::new_state("last_symbol_check".to_string()),
            ],
            transitions: vec![
                vec![
                    Transition::new_transition(3, 0, 1),
                    Transition::new_transition(3, 1, 1),
                    Transition::new_transition(3, 2, 1),
                    Transition::new_transition(3, 3, 1),
                ],
                vec![], // accept transitions
                vec![], // reject transitions
                vec![
                    Transition::new_transition(4, 0, -1),
                    Transition::new_transition(3, 1, 1),
                    Transition::new_transition(3, 2, 1),
                    Transition::new_transition(3, 3, 1),
                ],
                vec![
                    Transition::new_transition(2, 0, 1),
                    Transition::new_transition(2, 1, 1),
                    Transition::new_transition(1, 2, 1), // accept if last symbol is 0
                    Transition::new_transition(2, 3, 1), // reject if last symbol is 1
                ],
            ],
            ..TuringMachine::default()
        };

        let timeout_computation_result = machine.bounded_compute(&"1110".to_string(), (None, Some(5)));

        let mut timeout_computation = match timeout_computation_result {
            Ok(computation) => computation,
            Err(_) => panic!("Error in accepted computation")
        };

        timeout_computation.run();

        assert_eq!(timeout_computation.view_status(), &ComputationStatus::Spaceout);
        assert_eq!(timeout_computation.view_tape().len(), 6);
        assert_eq!(timeout_computation.view_tape(), &vec![1, 3, 3, 3, 2, 0]);
        assert_eq!(timeout_computation.view_head_position(), 5);
        assert_eq!(timeout_computation.view_clock().time, 5);
        assert_eq!(timeout_computation.view_clock().space, 6);
    }
}