use crate::automata::Symbol::CHAR;
use bimap::BiMap;
use std::any::type_name;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter, Write};
use std::thread::current;

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
            let mut best_end: Option<usize> = None;
            self._accept(self.q_start, &input[id..], id, &mut best_end, false);
            if let Some(end) = best_end {
                return Some((id, end));
            }
        }
        None
    }

    /// Backtracking implementation for finding a greedy match of the pattern represented by `self` in `word`
    /// The index of the character at which the longest match ends is stored in `last_accepted` (`None` if no match)
    /// Probably the worst code you'll ever see, I'm surprised it hasn't panicked on me yet
    fn _accept(
        &self,
        state: usize,
        word: &str,
        depth: usize,
        last_accepted: &mut Option<usize>,
        full_accept: bool,
    ) {
        if word.is_empty() || !full_accept {
            let ec = self.ec(state);
            if ec.iter().find(|q| self.q_accepting.contains(q)).is_some() {
                // if q_0 accepts we won't do a recursive loop and thus
                // won't have added anything to the depth
                if depth == 0 {
                    *last_accepted = Some(depth);
                }
                // ah yes options how pretty
                if let Some(best_end) = last_accepted {
                    if depth - 1 > *best_end {
                        *last_accepted = Some(depth - 1); // undo last addition
                    }
                } else {
                    *last_accepted = Some(depth - 1);
                }
            }
        }
        if let Some(c) = word.chars().nth(0) {
            for transition in self.find_transitions(state, CHAR(c)) {
                let consumed = match transition.1 {
                    CHAR(_) => 1,
                    Symbol::EPSILON | Symbol::EMPTY => 0,
                };
                self._accept(
                    transition.2,
                    &word[consumed..],
                    depth + consumed,
                    last_accepted,
                    full_accept,
                );
            }
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

    fn contains_accepting_state(&self, states: &BTreeSet<usize>) -> bool {
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
        let mut best_end = None;
        self._accept(self.q_start, word, 0, &mut best_end, true);
        best_end.is_some()
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

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Dfa {
    states: Vec<usize>,
    // using a hashmap should make the thing go speeeeed
    transitions: HashMap<(usize, Symbol), usize>,
    pub q_start: usize,
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

    /// Powerset Construction of a DFA from the passed `Nfa`.
    pub fn from(nfa: &Nfa) -> Dfa {
        let successors = nfa.successors_single();
        let new_q0: BTreeSet<usize> = nfa.ec(nfa.q_start).drain(..).collect();
        let new_q0_id = next_state();

        let mut id_to_state_set: BiMap<usize, BTreeSet<usize>> = BiMap::new();
        id_to_state_set.insert(new_q0_id.clone(), new_q0.clone());

        let mut dfa = Dfa::new(vec![new_q0_id], HashMap::new(), new_q0_id, HashSet::new());
        if nfa.contains_accepting_state(&new_q0) {
            dfa.q_accepting.insert(new_q0_id);
        }
        drop(new_q0);

        let mut i: usize = 0;
        while let Some(state) = dfa.states.get(i).cloned() {
            // states in the nfa
            let old_states = id_to_state_set.get_by_left(&state).unwrap();
            let transitions = nfa.successors_multiple(&old_states, &successors);
            for (with, target) in transitions {
                let to = if let Some(state) = id_to_state_set.get_by_right(&target) {
                    // state set has been previously generated
                    *state
                } else {
                    let new_state = next_state();
                    dfa.states.push(new_state);
                    if nfa.contains_accepting_state(&target) {
                        dfa.q_accepting.insert(new_state);
                    }
                    id_to_state_set.insert(new_state, target);
                    new_state
                };
                // insert the appropriate transition to this state
                dfa.transitions.insert((state, with.clone()), to);
            }
            i += 1
        }

        dfa
    }

    /// Find all characters at which the Dfa is in an accepting state
    fn _find_ends(&self, input: &str, allow_resets: bool) -> Vec<usize> {
        let mut current = self.q_start;
        let mut ends: Vec<usize> = Vec::new();
        for (pos, c) in input.chars().enumerate() {
            if let Some(next) = self.transitions.get(&(current, CHAR(c))) {
                current = *next;
                if self.q_accepting.contains(&current) {
                    ends.push(pos);
                }
                continue;
            }
            if !allow_resets {
                break;
            }
            current = self.q_start;
        }
        ends
    }

    /// Find all matches of the pattern represented by `self` in `input`.
    /// Returns an ordered vector of tuples `(start, end)`, where each tuple represents an individual match.
    /// Greediness of the matches is achieved by
    /// 1. collecting all possible ends of a match,
    /// 2. Running the `reversed` version of the DFA on the reversed input for each of these ends (see https://swtch.com/~rsc/regexp/regexp3.html#submatch)
    ///     to find the possibile starts for each match
    /// 3. Consolidating this mapping of `end -> possible starts` to only pick the earliest start possible for every `end`.
    ///     By using a hash map for this, different ends that have the same earliest start are consolidated as well, to choose the latest end.
    ///
    /// Don't ask me why this works I'm not even 100% certain that it does at all
    pub fn find_all(&self, input: &str, reversed: &Dfa) -> Option<Vec<(usize, usize)>> {
        let ends = self._find_ends(input, true);
        if ends.is_empty() {
            return None;
        }
        let reversed_input: String = input.chars().rev().collect();
        let mut potential_matches: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for end in ends {
            let rev_end = input.len() - end - 1;
            potential_matches.insert(
                end,
                reversed
                    ._find_ends(&reversed_input[rev_end..], false)
                    .iter()
                    .map(|x| input.len() - x - 1 - rev_end)
                    .rev()
                    .collect(),
            );
        }
        // println!("potential: {:?}", consolidate(&potential_matches));
        Some(consolidate(&potential_matches))
    }

    /// Find the first match of the pattern represented by `self` in `input`.
    pub fn find(&self, input: &str, reversed: &Dfa) -> Option<(usize, usize)> {
        if let Some(matches) = self.find_all(input, reversed) {
            matches.first().cloned()
        } else {
            None
        }
    }

    // may not actually be needed we'll see
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
        for state in self.states.iter() {
            if num_state.contains(state) {
                return Err(format!("State {} exists twice", state));
            }
            num_state.insert(*state);
        }

        Ok(())
    }

    fn accept(&self, input: &str) -> bool {
        let mut current = self.q_start;
        'main: loop {
            for c in input.chars() {
                if let Some(next) = self.transitions.get(&(current, CHAR(c))) {
                    current = *next;
                    continue;
                }
                break 'main false;
            }
            break self.q_accepting.contains(&current);
        }
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
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Consolidate a set of potential matches to greedy matches.
/// idfk how else I should explain this
pub fn consolidate(ends_to_starts: &BTreeMap<usize, Vec<usize>>) -> Vec<(usize, usize)> {
    let mut starts_to_ends: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for (end, starts) in ends_to_starts {
        starts_to_ends
            .entry(*starts.first().unwrap())
            .or_default()
            .insert(*end);
    }
    starts_to_ends
        .iter()
        .map(|(start, ends)| (*start, *ends.last().unwrap()))
        .collect()
}
