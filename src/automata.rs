use crate::automata::Symbol::CHAR;
use std::arch::x86_64::_mm_undefined_si128;
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

    // fixme: non-determinism due to how iter() on hashsets works but okay for prototyping
    // in order to get greedy (but deterministic) behaviour, I should probably move all hashsets to vectors
    fn find_transitions(&self, from: usize, c: Symbol) -> Vec<(usize, Symbol, usize)> {
        self.transitions
            .iter()
            .filter(|(f, w, _)| *f == from && (*w == c || *w == Symbol::EPSILON))
            .cloned()
            .collect()
    }

    pub fn epsilon_closure(&self, state: usize, ec: &mut HashSet<usize>) {
        ec.insert(state);

        for transition in &self.find_transitions(state, Symbol::EPSILON) {
            let state = transition.2;
            if ec.contains(&state) {
                continue
            }
            self.epsilon_closure(state, ec);
        }
    }

    /////////////////////////////////////////////// CONSTRUCTION METHODS ///////////////////////////////////////////////
    // note: these methods are supposed to be used in a sort of Accumulator pattern, with self being the accumulator

    /// '*' and '+' quantifiers
    pub fn klenee(&mut self, allow_empty: bool) {
        let klenee_state = next_state();
        self.states.push(klenee_state);
        self.transitions
            .insert((klenee_state, Symbol::EPSILON, self.q_start));
        self.q_start = klenee_state;
        for f in self.q_accepting.iter() {
            self.transitions.insert((*f, Symbol::EPSILON, self.q_start));
        }
        // * accepts empty word, + does not
        // if we do, we can remove all other accepting states
        if allow_empty {
            self.q_accepting = HashSet::from([self.q_start]);
        }
    }

    /// '?' quantifier
    pub fn optional(&mut self) {
        self.union(Nfa::from_symbol(&Symbol::EPSILON))
    }

    pub fn concat(&mut self, other: Nfa) {
        self.states.extend(&other.states);
        self.transitions.extend(other.transitions);
        for f in self.q_accepting.iter() {
            self.transitions
                .insert((*f, Symbol::EPSILON, other.q_start));
        }
        self.q_accepting = other.q_accepting;
    }

    pub fn union(&mut self, other: Nfa) {
        self.states.extend(&other.states);
        self.transitions.extend(other.transitions);

        let union_state = next_state();
        self.states.push(union_state);
        self.transitions
            .insert((union_state, Symbol::EPSILON, self.q_start));
        self.transitions
            .insert((union_state, Symbol::EPSILON, other.q_start));
        self.q_start = union_state;

        self.q_accepting.extend(other.q_accepting);
    }
}

impl Automaton for Nfa {
    fn from_symbol(s: &Symbol) -> Self {
        match s {
            CHAR(c) => {
                let states = next_states(2);
                // deliberately cloning c, because constructed NFA needs to be logically independent of original pattern
                let transition: (usize, Symbol, usize) = (states[0], Symbol::CHAR(*c), states[1]);
                let q_start = states[0];
                let q_accepting = HashSet::from([states[1]]);
                Nfa::new(
                    states,
                    HashSet::from_iter(vec![transition]),
                    q_start,
                    q_accepting,
                )
            }
            Symbol::EPSILON => {
                let q_0 = next_state();
                Nfa::new(vec![q_0], HashSet::new(), q_0, HashSet::from([q_0]))
            }
            Symbol::EMPTY => {
                let q_0 = next_state();
                Nfa::new(vec![q_0], HashSet::new(), q_0, HashSet::new())
            }
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

    /// Simulate a run of `self` on the word `input`.
    /// Uses simple backtracking to get hold of NFAs non-determinism
    fn _match(&self, state: usize, word: &str) -> bool {
        if word.is_empty() {
            let mut ec = HashSet::new();
            self.epsilon_closure(state, &mut ec);
            return ec.iter().find(|q| self.q_accepting.contains(q)).is_some();
        }
        let sc = Symbol::CHAR(word.chars().nth(0).unwrap());
        for transition in self.find_transitions(state, sc) {
            let from = match transition.1 {
                CHAR(_) => 1,
                Symbol::EPSILON | Symbol::EMPTY => 0,
            };
            if self._match(transition.2, &word[from..]) {
                return true;
            }
        }
        false
    }
}

impl Debug for Nfa {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Nfa {{")?;
        writeln!(f, "\tQ: {:?},", self.states)?;

        let mut transitions: Vec<_> = self.transitions.iter().collect();
        transitions.sort_by_key(|t| &t.0);
        writeln!(f, "\tD: {{")?;
        for t in transitions {
            writeln!(f, "\t\t{:?},", t)?;
        }
        writeln!(f, "\t}}")?;
        writeln!(f, "\tq_0: {:?},", self.q_start)?;
        writeln!(f, "\tF: {:?},", self.q_accepting)?;
        write!(f, "}}")
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
    fn minimize(&self) {
        todo!()
    }
}

///////////////////////////////////////////////////// HELPER TYPES ////////////////////////////////////////////////////

// I don't plan on threading this (yet) so for now it's fine
thread_local! {
    static STATE_GEN: RefCell<usize> = RefCell::new(0);
}

fn next_state() -> usize {
    STATE_GEN.with(|g| {
        let mut cell = g.borrow_mut();
        let next = *cell;
        *cell += 1;
        next
    })
}

fn next_states(n: usize) -> Vec<usize> {
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

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum Symbol {
    CHAR(char),
    EPSILON,
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
