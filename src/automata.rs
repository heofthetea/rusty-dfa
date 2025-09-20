use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Write};

pub trait Automaton {
    /// Construct a fully valid `Automaton` accepting exactly the passed `Symbol`.
    fn from_symbol(s: &Symbol, alphabet: HashSet<Symbol>) -> Self;

    /// Validate the `Automaton`
    /// returns: Ok(()) if valid, Err(reason) if not
    fn validate(&self) -> Result<(), String>;

    /// Simulate a run of `self` on the word `input`.
    /// returns: true if `input` is accepted, false otherwise.
    /// todo I may rework this in the future to also return where we matched or sth but for now is fine
    fn _match(&self, state: u32, input: &str) -> bool;
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
        match self  {
            Symbol::CHAR(c) => {f.write_char(c.clone())}
            Symbol::EPSILON => {f.write_str("")}
            Symbol::EMPTY => {f.write_str("")}
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Nfa {
    states: Vec<u32>, // vec makes sense here because states are always counted upwards
    alphabet: HashSet<Symbol>,
    transitions: HashSet<(u32, Symbol, u32)>,
    q_start: u32,
    q_accepting: HashSet<u32>,
}

impl Nfa {
    pub fn new(
        states: Vec<u32>,
        alphabet: HashSet<Symbol>,
        transitions: HashSet<(u32, Symbol, u32)>,
        q_start: u32,
        q_accepting: HashSet<u32>,
    ) -> Nfa {
        let nfa = Nfa {
            states,
            alphabet,
            transitions,
            q_start,
            q_accepting,
        };
        match nfa.validate() {
            Ok(_) => {}
            Err(e) => {
                panic!("Requested construction of invalid NFA: {}", format!("{}\n{:?}", e, nfa))
            }
        };

        nfa
    }

    // todo: non-determinism due to how iter() on hashsets works but okay for prototyping
    // in order to get greedy (but deterministic) behaviour, I should probably move all hashsets to vectors
    fn find_transitions(&self, from: u32, c: Symbol) -> Vec<(u32, Symbol, u32)> {
        self.transitions.iter().filter(|(f, w, _)| *f == from && *w == c).cloned().collect()
    }
}

impl Automaton for Nfa {
    fn from_symbol(s: &Symbol, alphabet: HashSet<Symbol>) -> Nfa {
        match s {
            Symbol::CHAR(c) => Nfa::new(
                vec![0, 1],
                alphabet,
                // deliberately cloning c, because constructed NFA needs to be logically independent of original pattern
                HashSet::from_iter(vec![(0, Symbol::CHAR(c.clone()), 1)]),
                0,
                HashSet::from([1]),
            ),
            Symbol::EPSILON => Nfa::new(vec![0], alphabet, HashSet::new(), 0, HashSet::from([0])),
            Symbol::EMPTY => Nfa::new(vec![0], alphabet, HashSet::new(), 0, HashSet::new()),
        }
    }

    fn validate(&self) -> Result<(), String> {
        if !self.states.contains(&self.q_start) {
            return Err(String::from("q_0 ∉ Q"));
        }
        if self.q_accepting.iter().any(|q| !self.states.contains(q)) {
            return Err(String::from("F ⊄ Q"))
        }
        for transition in self.transitions.iter() {
            if !self.states.contains(&transition.0) || !self.states.contains(&transition.2) {
                return Err(format!("{:?} has invalid state(s)", transition))
            }
        }

        Ok(())
    }

    fn _match(&self, state: u32, input: &str) -> bool {
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
    states: Vec<u32>,
    alphabet: HashSet<Symbol>,
    // using a hashmap should make the thing go speeeeed
    transitions: HashSet<(u32, Symbol), u32>,
    q_start: u32,
    q_accepting: HashSet<u32>,
}

impl Dfa {
    fn new(
        states: Vec<u32>,
        alphabet: HashSet<Symbol>,
        transitions: HashSet<(u32, Symbol), u32>,
        q_start: u32,
        q_accepting: HashSet<u32>,
    ) -> Dfa {
        Dfa {
            states,
            alphabet,
            transitions,
            q_start,
            q_accepting,
        }
    }
}
