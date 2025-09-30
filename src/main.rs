
use crate::automata::dfa::Dfa;
use crate::parse::parse;

mod automata;
mod parse;

#[cfg(test)]
mod test;

fn main() {
    let pattern = "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaa";
    let pattern = "a?a?aa";
    let nfa = parse(&pattern);
    println!("{:?}", &nfa);
    let dfa = Dfa::from(&nfa);
    println!("{:?}", &dfa)

}
