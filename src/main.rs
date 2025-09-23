use crate::automata::Automaton;
use crate::parse::parse;

mod automata;
mod parse;
mod benchmark;

#[cfg(test)]
mod test;

fn main() {
    println!();
    benchmark::run_benchmark_suite(20);
}
