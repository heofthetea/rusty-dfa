use std::cell::RefCell;
use std::fmt::{Display, Formatter, Write};


pub trait Automaton {
    /// Validate the `Automaton`
    /// returns: Ok(()) if valid, Err(reason) if not
    fn validate(&self) -> Result<(), String>;

    /// Simulate a run of `self` on the word `input`.
    /// returns: true if `input` is accepted, false otherwise.
    /// Note: This method accepts entire words, as by traditional theoretical definition
    fn accept(&self, input: &str) -> bool;

    // yeah this makes no sense this entire trait makes no sense to be honest lol
    // /// Find whether `input` contains a word of this automaton's language anywhere.
    // /// If so, returns the index of where the match starts.
    // fn find(&self, input: &str) -> Option<(usize, usize)>;
}

//////////////////////////////////////////////////// aaaaaaaaaaa //////////////////////////////////////////

// I don't plan on threading this (yet) so for now it's fine
thread_local! {
    static STATE_GEN: RefCell<usize> = RefCell::new(0);
}

pub fn next_state() -> usize {
    STATE_GEN.with(|g| {
        let mut cell = g.borrow_mut();
        let next = *cell;
        *cell += 1;
        next
    })
}

pub fn next_states(n: usize) -> Vec<usize> {
    let range = STATE_GEN.with(|g| {
        let mut cell = g.borrow_mut();
        let first = *cell;
        *cell += n;
        // both bounds are inclusive
        (first, *cell - 1)
    });
    vec![range.0, range.1]
}

/// only for testing purposes
pub fn reset_state_counter() {
    STATE_GEN.with(|g| {
        let mut cell = g.borrow_mut();
        *cell = 0usize;
    })
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Symbol {
    CHAR(char),
    EPSILON,
    // EVERYTHING, // used later on for the everything matcher .
    EMPTY, // the empty language -> not sure if I actually need it. If not: todo rework this enum to an Optional
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::CHAR(c) => f.write_char(*c),
            Symbol::EPSILON => f.write_str(""),
            Symbol::EMPTY => f.write_str(""),
        }
    }
}