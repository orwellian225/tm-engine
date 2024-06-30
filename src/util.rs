use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct State(String);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(char);
#[derive(Debug, PartialEq, Eq)]
pub struct Transition(usize, usize, i32);

impl State {
    pub fn new_state(state_identifier: String) -> Self {
        State(state_identifier)
    }

    pub fn as_string(&self) -> &String {
        &self.0
    }
    
    pub fn as_str(&self) -> &str {
        &self.0.as_str()
    }
}

impl Symbol {
    pub fn new_symbol(symbol_identifier: char) -> Self {
        Symbol(symbol_identifier)
    }

    pub fn as_char(&self) -> char {
        self.0
    }
}

impl Transition {
    pub fn new_transition(next_state: usize, write_symbol: usize, direction: i32) -> Self {
        Transition(next_state, write_symbol, direction)
    }

    pub fn next_state_id(&self) -> usize { self.0 }
    pub fn write_symbol_id(&self) -> usize { self.1 }
    pub fn direction(&self) -> i32 { self.2 }
    pub fn direction_to_string(&self) -> String { 
        if self.2 == 0 {
            "S".to_string()
        } else if self.2 < 0 {
            format!("L{}", self.2.unsigned_abs())
        } else {
            format!("R{}", self.2.unsigned_abs())
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let direction: String;
        if self.2 == 0 {
            direction = "S".to_string();
        } else if self.2 < 0 {
            direction = format!("L{}", self.2.unsigned_abs());
        } else {
            direction = format!("R{}", self.2.unsigned_abs());
        }

        write!(f, "{},{},{}", self.0, self.1, direction)
    }
}

#[derive(Debug)]
 pub struct ComputationClock {
    pub time: usize,
    pub space: usize,
    pub max_time: Option<usize>,
    pub max_space: Option<usize>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ComputationStatus {
    #[default]
    Executing,
    Accept,
    Reject,
    Timeout,
    Spaceout
}


impl Default for ComputationClock {
    fn default() -> Self {
        Self { max_time: None, max_space: None, time: 0, space: 1 }
    }
}

impl ComputationClock {
    pub fn clock(max_time: Option<usize>, max_space: Option<usize>, tape_length: usize) -> Self {
        Self {
            max_time, max_space, time: 0, space: tape_length
        }
    }
}