use Symbol::CHAR;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{Debug, Formatter, Write};
use crate::automata::automaton::{Symbol, Automaton, next_state, next_states};

pub struct Nfa {
    pub states: Vec<usize>,
    pub transitions: HashSet<(usize, Symbol, usize)>,
    pub q_start: usize,
    pub q_accepting: HashSet<usize>,
    pub alphabet: HashSet<Symbol>,
}

impl Nfa {
    pub fn new(
        states: Vec<usize>,
        transitions: HashSet<(usize, Symbol, usize)>,
        q_start: usize,
        q_accepting: HashSet<usize>,
    ) -> Nfa {
        let alphabet = transitions
            .iter()
            .map(|(_, w, _)| w)
            .filter(|w| **w != Symbol::EPSILON)
            .cloned()
            .collect();
        let nfa = Nfa {
            states,
            transitions,
            q_start,
            q_accepting,
            alphabet,
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

    /// Construct a fully valid `Automaton` accepting exactly the passed `Symbol`.
    pub fn from_symbol(s: &Symbol) -> Self {
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

    // fixme: non-determinism due to how iter() on hashsets works but okay for prototyping
    // in order to get greedy (but deterministic) behaviour, I should probably move all hashsets to vectors
    fn find_transitions(&self, from: usize, c: Symbol) -> Vec<&(usize, Symbol, usize)> {
        self.transitions
            .iter()
            .filter(|(f, w, _)| *f == from && (*w == c || *w == Symbol::EPSILON))
            .collect()
    }

    pub fn find(&self, input: &str) -> Option<(usize, usize)> {
        for (id, c) in input.chars().enumerate() {
            if let Some(end) = self._accept(self.q_start, &input[id..], id, false) {
                return Some((id, end));
            }
        }
        None
    }

    fn _accept(&self, state: usize, word: &str, depth: usize, full_accept: bool) -> Option<usize> {
        if word.is_empty() || !full_accept {
            let ec = self.ec(state);
            if ec.iter().find(|q| self.q_accepting.contains(q)).is_some() {
                // if q_0 accepts we won't do a recursive loop and thus
                // won't have added anything to the depth
                if depth == 0 {
                    return Some(depth);
                }
                return Some(depth - 1); // undo last addition
            }
        }
        if let Some(c) = word.chars().nth(0) {
            for transition in self.find_transitions(state, CHAR(c)) {
                let consumed = match transition.1 {
                    CHAR(_) => 1,
                    Symbol::EPSILON | Symbol::EMPTY => 0,
                };
                let end = self._accept(
                    transition.2,
                    &word[consumed..],
                    depth + consumed,
                    full_accept,
                );
                if end.is_some() {
                    return end;
                }
            }
        }
        None
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
        self.alphabet.extend(other.alphabet);
        for f in self.q_accepting.iter() {
            self.transitions
                .insert((*f, Symbol::EPSILON, other.q_start));
        }
        self.q_accepting = other.q_accepting;
    }

    pub fn union(&mut self, other: Nfa) {
        self.states.extend(&other.states);
        self.transitions.extend(other.transitions);
        self.alphabet.extend(other.alphabet);

        let union_state = next_state();
        self.states.push(union_state);
        self.transitions
            .insert((union_state, Symbol::EPSILON, self.q_start));
        self.transitions
            .insert((union_state, Symbol::EPSILON, other.q_start));
        self.q_start = union_state;

        self.q_accepting.extend(other.q_accepting);
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_finding(&mut self) {
        for state in &self.states {
            self.transitions
                .insert((*state, Symbol::EPSILON, self.q_start));
        }
    }

    /// Reverse an `Nfa`.
    /// This is useful when using a DFA to _find_ a pattern in a string, see https://swtch.com/~rsc/regexp/regexp3.html#submatch for more
    /// todo when I'm bored I may try to prove that this is working as I expect it to myself, for now I'm content with testing
    pub fn reversed(&self) -> Nfa {
        let mut reversed = Nfa::new(
            self.states.clone(),
            HashSet::new(),
            self.q_start,
            HashSet::from([self.q_start]),
        );

        for (from, with, to) in &self.transitions {
            reversed.transitions.insert((*to, with.clone(), *from));
        }
        let new_q0 = next_state();
        reversed.states.push(new_q0);
        reversed.q_start = new_q0;
        for f in &self.q_accepting {
            reversed
                .transitions
                .insert((reversed.q_start, Symbol::EPSILON, *f));
        }

        reversed
    }

    /////////////////////////////////////////////// POWERSET CONSTRUCTION //////////////////////////////////////////////

    pub fn ec(&self, state: usize) -> Vec<usize> {
        let mut ec: HashSet<usize> = HashSet::new();
        self._ec(state, &mut ec);
        ec.into_iter().collect()
    }

    /// Stateful recursive stack-safe function to collect the epsilon closure of `state` into `ec`
    fn _ec(&self, state: usize, ec: &mut HashSet<usize>) {
        ec.insert(state);

        for transition in &self.find_transitions(state, Symbol::EPSILON) {
            let state = transition.2;
            if ec.contains(&state) {
                continue;
            }
            self._ec(state, ec);
        }
    }

    // todo: replace this with sneaky workaround using Symbol::EVERYTHING in find_transitions
    fn find_symbol_transitions(&self, from: &usize) -> Vec<&(usize, Symbol, usize)> {
        self.transitions
            .iter()
            .filter(|t| &t.0 == from && t.1 != Symbol::EPSILON)
            .collect()
    }

    /// Calculate all possible successor states for a single state
    pub(crate) fn successors_single(&self) -> HashMap<(usize, Symbol), BTreeSet<usize>> {
        let mut successors: HashMap<(usize, Symbol), BTreeSet<usize>> = HashMap::new();

        for state in &self.states {
            for transition in self.find_symbol_transitions(state) {
                successors
                    .entry((*state, transition.1))
                    .or_default()
                    .extend(self.ec(transition.2)); // behaves like a union here
            }
        }

        successors
    }

    /// Calculate all possible successor states for a set of `states`
    pub(crate) fn successors_multiple(
        &self,
        states: &BTreeSet<usize>,
        successors_single: &HashMap<(usize, Symbol), BTreeSet<usize>>,
    ) -> HashMap<Symbol, BTreeSet<usize>> {
        let mut successors_by_symbol: HashMap<Symbol, BTreeSet<usize>> = HashMap::new();
        for state in states {
            let alphabet: HashSet<Symbol> = self
                .find_symbol_transitions(state)
                .iter()
                .map(|t| t.1)
                .collect();
            for s in alphabet.into_iter() {
                successors_by_symbol
                    .entry(s)
                    .or_default()
                    .extend(successors_single.get(&(*state, s)).unwrap());
            }
        }
        successors_by_symbol
    }

    pub(crate) fn contains_accepting_state(&self, states: &BTreeSet<usize>) -> bool {
        for partial in states {
            if self.q_accepting.contains(partial) {
                return true;
            }
        }
        false
    }
}

impl Automaton for Nfa {
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

        let mut num_state: HashSet<usize> = HashSet::new();
        for state in &self.states {
            if num_state.contains(&state) {
                return Err(format!("State {} exists twice", state));
            }
            num_state.insert(*state);
        }

        Ok(())
    }

    /// Simulate a run of `self` on the word `input`.
    /// Uses simple backtracking to get hold of NFAs non-determinism
    /// WARNING: this will cause infinite recursion on epsilon cycles lol
    /// Maybe I can fix that by using epsilon closures instead of raw transitions...
    fn accept(&self, word: &str) -> bool {
        self._accept(self.q_start, word, 0, true).is_some()
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
        writeln!(f, "\tE: {:?}", self.alphabet)?;
        write!(f, "}}")
    }
}

