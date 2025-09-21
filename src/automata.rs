use crate::automata::Symbol::CHAR;
use bimap::{BiBTreeMap, BiMap};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter, Write};

pub trait Automaton {
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

    // todo: replace this with sneaky workaround using Symbol::EVERYTHING in find_transitions
    fn find_symbol_transitions(&self, from: &usize) -> Vec<&(usize, Symbol, usize)> {
        self.transitions
            .iter()
            .filter(|t| &t.0 == from && t.1 != Symbol::EPSILON)
            .collect()
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

    /// Calculate all possible successor states for a single state
    fn successors_single(&self) -> HashMap<(usize, Symbol), BTreeSet<usize>> {
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
    fn successors_multiple(
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

    fn intersect_q_accepting(&self, states: &BTreeSet<usize>) -> bool {
        for partial in states {
            if self.q_accepting.contains(partial) {
                return true;
            }
        }
        false
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
        drop(num_state);

        Ok(())
    }

    /// Simulate a run of `self` on the word `input`.
    /// Uses simple backtracking to get hold of NFAs non-determinism
    fn _match(&self, state: usize, word: &str) -> bool {
        if word.is_empty() {
            let ec = self.ec(state);
            return ec.iter().find(|q| self.q_accepting.contains(q)).is_some();
        }
        let sc = CHAR(word.chars().nth(0).unwrap());
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
    transitions: HashMap<(usize, Symbol), usize>,
    q_start: usize,
    q_accepting: HashSet<usize>,
}

impl Dfa {
    pub fn new(
        states: Vec<usize>,
        transitions: HashMap<(usize, Symbol), usize>,
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

    pub fn from(nfa: &Nfa) -> Dfa {
        let successors = nfa.successors_single();
        let new_q0: BTreeSet<usize> = nfa.ec(nfa.q_start).drain(..).collect();
        let new_q0_id = next_state();

        let mut states: BiMap<usize, BTreeSet<usize>> = BiMap::new();
        states.insert(new_q0_id.clone(), new_q0.clone());

        let mut dfa = Dfa::new(
            states.iter().map(|(q, _)| *q).collect(),
            HashMap::new(),
            new_q0_id,
            HashSet::new(),
        );
        if nfa.intersect_q_accepting(&new_q0) {
            dfa.q_accepting.insert(new_q0_id);
        }

        let mut i: usize = 0;
        while let Some(state) = dfa.states.get(i).cloned() {
            let old_states = states.get_by_left(&state).unwrap();
            let transitions = nfa.successors_multiple(&old_states, &successors);
            for (with, target) in transitions {
                let to = if let Some(state) = states.get_by_right(&target) {
                    //fixme extract
                    if nfa.intersect_q_accepting(&target) {
                        dfa.q_accepting.insert(*state);
                    }
                    *state
                } else {
                    let new_state = next_state();
                    if nfa.intersect_q_accepting(&target) {
                        dfa.q_accepting.insert(new_state);
                    }

                    states.insert(new_state, target);
                    dfa.states.push(new_state);
                    new_state
                };
                // only for debugging purposes
                if dfa.transitions.contains_key(&(state, with.clone())) {
                    panic!("{} already has a transition for symbol {}", state, with)
                }
                // insert the appropriate transition to this state
                dfa.transitions.insert((state, with.clone()), to);
            }
            i += 1
        }

        dfa
    }

    pub fn minimize(&self) {
        todo!()
    }
}

impl Automaton for Dfa {
    fn validate(&self) -> Result<(), String> {
        if !self.states.contains(&self.q_start) {
            return Err(String::from("q_0 ∉ Q"));
        }
        if self.q_accepting.iter().any(|q| !self.states.contains(q)) {
            return Err(String::from("F ⊄ Q"));
        }
        for transition in self.transitions.iter() {
            if !self.states.contains(&transition.0.0) || !self.states.contains(&transition.1) {
                return Err(format!("{:?} has invalid state(s)", transition));
            }
            if transition.0.1 == Symbol::EPSILON {
                return Err(format!(
                    "{:?} is a forbidden epsilon-transition",
                    transition
                ));
            }
        }

        let mut num_state: HashSet<usize> = HashSet::new();
        for state in &self.states {
            if num_state.contains(&state) {
                return Err(format!("State {} exists twice", state));
            }
            num_state.insert(*state);
        }
        drop(num_state);

        Ok(())
    }

    fn _match(&self, state: usize, input: &str) -> bool {
        todo!()
    }
}

impl Debug for Dfa {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Dfa {{")?;
        writeln!(f, "\tQ: {:?},", self.states)?;

        let mut transitions: Vec<_> = self.transitions.iter().collect();
        transitions.sort_by_key(|t| &t.0.0);
        writeln!(f, "\td: {{")?;
        for t in transitions {
            writeln!(f, "\t\t{:?},", t)?;
        }
        writeln!(f, "\t}}")?;
        writeln!(f, "\tq_0: {:?},", self.q_start)?;
        writeln!(f, "\tF: {:?},", self.q_accepting)?;
        write!(f, "}}")
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
