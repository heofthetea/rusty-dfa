use std::time::{Duration, Instant};
use std::fs::File;
use std::io::{Write, BufWriter};
use crate::automata::Dfa;
use crate::parse::parse;

/// I think this behaves quadratically, so yay i guess at least better than exponential lol oops
pub fn benchmark_dfa_klenee(max: usize) {
    let pattern = "a*";
    let before_parse = Instant::now();
    let (dfa, dfa_reversed) = build_finding_dfas(pattern);
    println!("construction: {:?}", before_parse.elapsed());

    let mut times: Vec<Duration> = Vec::new();
    for i in 1..max {
        let word = "a".repeat(i);
        let before_match = Instant::now();
        let found_match = dfa.find(&word, &dfa_reversed);
        times.push(before_match.elapsed());
        assert_eq!(found_match, Some((0, i - 1)))
    }
    println!("{:?}", times);
    export_benchmark_to_csv(times);
}


fn export_benchmark_to_csv(times: Vec<Duration>) {
    let file = File::create("../../benchmark.csv").expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    writeln!(writer, "number_of_characters,duration_seconds").unwrap();
    for (i, duration) in times.iter().enumerate() {
        let num_chars = i + 1;
        let secs = duration.as_secs_f64();
        writeln!(writer, "{},{}", num_chars, secs).unwrap();
    }
}

fn build_finding_dfas(pattern: &str) -> (Dfa, Dfa) {
    let mut nfa = parse(&pattern);
    let dfa_reversed = Dfa::from(&nfa.reversed());
    nfa.to_finding();
    let dfa = Dfa::from(&nfa);
    (dfa, dfa_reversed)
}


