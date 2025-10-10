
use crate::automata::{Automaton, Dfa};
use crate::benchmark::benchmark_dfa::benchmark_dfa_klenee;
use crate::parse::parse;

mod automata;
mod parse;

#[cfg(test)]
mod test;
mod benchmark;

fn main() {
    benchmark_dfa_klenee(20000, 100);

}
