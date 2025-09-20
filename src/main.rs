use std::collections::HashSet;
use crate::automata::Automaton;
use crate::automata::Nfa;
use crate::automata::Symbol;

mod automata;
mod parse;

#[cfg(test)]
mod test;

fn main() {
    let alphabet = HashSet::from([Symbol::CHAR('a'), Symbol::CHAR('b'), Symbol::EMPTY, Symbol::EPSILON]);
    let pattern = Symbol::CHAR('a');
    let input = "a";
    let nfa = Nfa::from_symbol(&pattern, alphabet);
    println!("{:?}", nfa);
    println!("Matching '{}' against '{}': {}", input, pattern, nfa._match(0, input))
}
