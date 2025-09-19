use std::collections::HashSet;

pub struct Nfa {
    states: Vec<u32>,
    alphabet: HashSet<char>,
    transitions: HashSet<(u32, char, u32)>,
    q_start: u32,
    q_accepting: Vec<u32>,
}

pub struct Dfa {
    states: Vec<u32>,
    alphabet: HashSet<char>,
    // yes, this is the only difference to Nfa, but I wanted to work with function pointers a bit so here you go
    transitions: fn(from: u32, with: char) -> u32,
    q_start: u32,
    q_accepting: Vec<u32>,
}
