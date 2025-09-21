
use crate::automata::Automaton;
use crate::parse::parse;

mod automata;
mod parse;

#[cfg(test)]
mod test;

fn main() {
    let pattern = "(a|b)?o*b";
    let nfa = parse(&pattern);
    println!("{:?}", nfa);
    let input = "boooob";
    println!("Matching {} against {} -> {}", input, pattern, nfa._match(nfa.q_start, input) )
}
