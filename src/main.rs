
use crate::automata::Automaton;
use crate::parse::parse;

mod automata;
mod parse;

#[cfg(test)]
mod test;

fn main() {
    let pattern = "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaa";
    let nfa = parse(&pattern);
    println!("{:?}", nfa);

}
