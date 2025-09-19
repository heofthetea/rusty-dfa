use std::collections::HashSet;
use crate::automata::Automaton;
use crate::automata::Nfa;
use crate::automata::Symbol;

mod automata;
mod parse;

fn main() {
    let alphabet = HashSet::from([Symbol::CHAR('a'), Symbol::CHAR('b'), Symbol::EMPTY, Symbol::EPSILON]);
    let s  = Symbol::CHAR('a');
    let nfa = Nfa::from_symbol(s, alphabet);
    println!("{:?}", nfa);
    println!("{}", nfa._match(0, "b"))
}
