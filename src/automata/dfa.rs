
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use bimap::BiMap;
use crate::automata::automaton::{next_state, Automaton};
use crate::automata::automaton::Symbol;
use crate::automata::automaton::Symbol::CHAR;
use crate::automata::nfa::{Nfa};

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

    /// non-greedy currently
    fn _find(&self, input: &str) -> Option<(usize, usize)> {
        let mut current = self.q_start;
        let mut start: usize = 0; // not the actual start of the match - just a lower bound
        for (pos, c) in input.chars().enumerate() {
            if let Some(next) = self.transitions.get(&(current, CHAR(c))) {
                current = *next;
                if self.q_accepting.contains(&current) {
                    return Some((start, pos));
                }
                continue;
            }
            current = self.q_start;
            start = 0;
        }
        None
    }

    pub fn find(&self, input: &str, reversed: &Dfa) -> Option<(usize, usize)> {
        if let Some((_, end)) = self._find(input) {
            let reversed_input: String = input.chars().rev().collect();
            let rev_end = input.len() - end - 1;
            // should always find a match because of reversal
            let (_, start) = reversed._find(&reversed_input[rev_end..]).unwrap();
            return Some((&reversed_input[rev_end..].len() - start - 1, end))

        }
        None
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