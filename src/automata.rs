use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Write};

pub trait Automaton {
    /// Construct a fully valid `Automaton` accepting exactly the passed `Symbol`.
    fn from_symbol(s: &Symbol) -> Self;

    /// Validate the `Automaton`
    /// returns: Ok(()) if valid, Err(reason) if not
    fn validate(&self) -> Result<(), String>;

    /// Simulate a run of `self` on the word `input`.
    /// returns: true if `input` is accepted, false otherwise.
    /// todo I may rework this in the future to also return where we matched or sth but for now is fine
    fn _match(&self, state: usize, input: &str) -> bool;

    fn concat(&mut self, other: &Self) -> &Self;
    fn union(&mut self, other: &Self) -> &Self;
    fn klenee(&mut self, other: &Self) -> &Self;
}

pub struct Nfa {
    pub states: Vec<usize>,
    pub transitions: HashSet<(usize, Symbol, usize)>,
    pub q_start: usize,
    pub q_accepting: HashSet<usize>,
}

impl Nfa {
    pub fn new(
        states: Vec<usize>,
        transitions: HashSet<(usize, Symbol, usize)>,
        q_start: usize,
        q_accepting: HashSet<usize>,
    ) -> Nfa {
        let nfa = Nfa {
            states,
            transitions,
            q_start,
            q_accepting,
        };
        match nfa.validate() {
            Ok(_) => {}
            Err(e) => {
                panic!(
                    "Requested construction of invalid NFA: {}",
                    format!("{}\n{:?}", e, nfa)
                )
            }
        };

        nfa
    }

    // todo: non-determinism due to how iter() on hashsets works but okay for prototyping
    // in order to get greedy (but deterministic) behaviour, I should probably move all hashsets to vectors
    fn find_transitions(&self, from: usize, c: Symbol) -> Vec<(usize, Symbol, usize)> {
        self.transitions
            .iter()
            .filter(|(f, w, _)| *f == from && *w == c)
            .cloned()
            .collect()
    }
}

impl Automaton for Nfa {
    fn from_symbol(s: &Symbol) -> Self {
        match s {
            Symbol::CHAR(c) => {
                let states = next_states(2);
                // deliberately cloning c, because constructed NFA needs to be logically independent of original pattern
                let transition: (usize, Symbol, usize) = (
                    states[0],
                    Symbol::CHAR(c.clone()),
                    states[1],
                );
                let q_start = states[0];
                let q_accepting = HashSet::from([states[1]]);
                Nfa::new(
                    states,
                    HashSet::from_iter(vec![transition]),
                    q_start,
                    q_accepting,
                )
            }
            Symbol::EPSILON =>  {
                let q_0 = next_state();
                Nfa::new(vec![q_0], HashSet::new(), q_0, HashSet::from([q_0]))
            },
            Symbol::EMPTY => {
                let q_0 = next_state();
                Nfa::new(vec![q_0], HashSet::new(), q_0, HashSet::new())
            },
        }
    }

    fn validate(&self) -> Result<(), String> {
        if !self.states.contains(&self.q_start) {
            return Err(String::from("q_0 ∉ Q"));
        }
        if self.q_accepting.iter().any(|q| !self.states.contains(q)) {
            return Err(String::from("F ⊄ Q"));
        }
        for transition in self.transitions.iter() {
            if !self.states.contains(&transition.0) || !self.states.contains(&transition.2) {
                return Err(format!("{:?} has invalid state(s)", transition));
            }
        }

        Ok(())
    }

    fn _match(&self, state: usize, input: &str) -> bool {
        if input.is_empty() {
            return true;
        }
        for c in input.chars() {
            let sc = Symbol::CHAR(c);
            for transition in self.find_transitions(state, sc) {
                if self._match(transition.2, &input[1..]) {
                    return true;
                }
            }
        }
        return false;
    }

    fn concat(&mut self, other: &Nfa) -> &Nfa {
        self.states.extend(&other.states);
        self
    }

    fn union(&mut self, other: &Nfa) -> &Nfa {
        self
    }

    fn klenee(&mut self, other: &Nfa) -> &Nfa {
        self
    }
}

impl Debug for Nfa {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Nfa")
            .field("Q", &self.states)
            .field("d", &self.transitions)
            .field("q_0", &self.q_start)
            .field("F", &self.q_accepting)
            .finish()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Dfa {
    states: Vec<usize>,
    // using a hashmap should make the thing go speeeeed
    transitions: HashSet<(usize, Symbol), usize>,
    q_start: usize,
    q_accepting: HashSet<usize>,
}

impl Dfa {
    fn new(
        states: Vec<usize>,
        transitions: HashSet<(usize, Symbol), usize>,
        q_start: usize,
        q_accepting: HashSet<usize>,
    ) -> Dfa {
        Dfa {
            states,
            transitions,
            q_start,
            q_accepting,
        }
    }
}

///////////////////////////////////////////////////// HELPER TYPES ////////////////////////////////////////////////////

// I don't plan on threading this (yet) so for now it's fine
thread_local! {
    static STATE_GEN: RefCell<usize> = RefCell::new(0);
}

fn next_state() -> usize {
    STATE_GEN.with(|g| {
        let mut id = g.borrow_mut();
        let next_state = *id;
        *id += 1;
        next_state
    })
}

fn next_states(n: usize) -> Vec<usize> {
    let range = STATE_GEN.with(|g| {
        let mut id = g.borrow_mut();
        let first = *id;
        *id += n;
        // both bounds are inclusive
        (first, *id - 1)
    });
    vec![range.0, range.1]
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum Symbol {
    CHAR(char),
    EPSILON,
    EMPTY, // the empty language -> not sure if I actually need it. If not: todo rework this enum to an Optional
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::CHAR(c) => f.write_char(c.clone()),
            Symbol::EPSILON => f.write_str(""),
            Symbol::EMPTY => f.write_str(""),
        }
    }
}
