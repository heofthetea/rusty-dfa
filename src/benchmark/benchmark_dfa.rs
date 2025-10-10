use std::time::{Duration, Instant};
use std::fs::File;
use std::io::{Write, BufWriter};
use crate::automata::Dfa;
use crate::parse::parse;

/// I think this behaves quadratically, so yay i guess at least better than exponential lol oops
pub fn benchmark_dfa_klenee(max: usize, step_size: usize) {
    let pattern = "a*";
    let before_parse = Instant::now();
    let (dfa, dfa_reversed) = build_finding_dfas(pattern);
    println!("construction: {:?}", before_parse.elapsed());

    let mut times: Vec<Duration> = Vec::new();
    for i in (1..=(max + 1)).step_by(step_size) {
        let mut word = String::from("");
        word.extend("a".repeat(i).chars());
        // word.push('b');
        let before_match = Instant::now();
        let found_match = dfa.find(&word, &dfa_reversed);
        times.push(before_match.elapsed());
        assert_eq!(found_match, Some((0, i - 1)));
        println!("{}: {:?}", i, times.last().unwrap())
    }
    export_benchmark_to_csv("dfa_klenee.csv", times, step_size);
}


fn export_benchmark_to_csv(filename: &str, times: Vec<Duration>, step_size: usize) {
    let full_filename = format!("src/benchmark/results/{}", filename);
    let file = File::create(&full_filename).expect("Unable to create file");
    println!("Dumped to: {}", full_filename);
    let mut writer = BufWriter::new(file);
    writeln!(writer, "number_of_characters,duration_seconds").unwrap();
    for (i, duration) in times.iter().enumerate() {
        let num_chars = i * step_size + 1;
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


