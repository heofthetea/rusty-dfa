use crate::automata::{Nfa, Dfa, Symbol, reset_state_counter};
use std::collections::HashSet;
use std::time::Instant;



/// Run the complete benchmark suite with different NFA sizes
pub fn run_benchmark_suite(n: usize) {
    println!("{}", "=".repeat(60));
    println!("POWERSET CONSTRUCTION BENCHMARK");
    println!("{}", "=".repeat(60));

    // Benchmark with increasing NFA sizes
    for q in 3..=n {
        benchmark_powerset_construction(q);
    }
}


/// Generate a pathological NFA that creates exponential blowup in powerset construction.
/// Creates an NFA that matches strings where the nth character from the end is 'a'.
/// This forces the DFA to remember all possible positions, creating 2^n states.
fn generate_pathological_nfa(n: usize) -> Nfa {
    if n < 2 {
        panic!("Need at least 2 states for pathological NFA");
    }

    reset_state_counter();
    let mut states = Vec::new();
    let mut transitions = HashSet::new();

    // Create n+1 states: 0, 1, 2, ..., n
    for i in 0..=n {
        states.push(i);
    }

    let q_start = 0;
    let q_accepting = HashSet::from([n]);

    // State 0: can stay in state 0 on any symbol, or go to state 1 on 'a'
    transitions.insert((0, Symbol::CHAR('a'), 0));
    transitions.insert((0, Symbol::CHAR('b'), 0));
    transitions.insert((0, Symbol::CHAR('a'), 1));

    // States 1 to n-1: must advance on any symbol (tracking position from 'a')
    for i in 1..n {
        transitions.insert((i, Symbol::CHAR('a'), i + 1));
        transitions.insert((i, Symbol::CHAR('b'), i + 1));
    }

    // State n: accepting state, can loop on any symbol
    transitions.insert((n, Symbol::CHAR('a'), n));
    transitions.insert((n, Symbol::CHAR('b'), n));

    Nfa::new(states, transitions, q_start, q_accepting)
}

/// Benchmark the two powerset construction algorithms
fn benchmark_powerset_construction(q: usize) {
    println!("Benchmarking powerset construction with {} states", q);

    let nfa = generate_pathological_nfa(q);
    println!("Generated pathological NFA with {} states and {} transitions",
             nfa.states.len(), nfa.transitions.len());

    // Benchmark the new algorithm (from)
    let start = Instant::now();
    let _dfa1 = Dfa::from(&nfa);
    let time1 = start.elapsed();

    // Benchmark the old algorithm (from_old)
    let start = Instant::now();
    let _dfa2 = Dfa::from_old(&nfa);
    let time2 = start.elapsed();

    println!("Results:");
    println!("  from()     : {:?}", time1);
    println!("  from_old() : {:?}", time2);
    println!("  Speedup    : {:.2}x", time2.as_nanos() as f64 / time1.as_nanos() as f64);
    println!("  DFA states : {}", _dfa1.states.len());
    println!();
}