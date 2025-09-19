# Rusty DFA
Regular Expression Engine using DFA construction, written in Rust.

> This is, primarily, another step in my journey to desperately learn Rust for my Studienarbeit lol

[//]: # (<br>The DFA is constructed via [powerset construction]&#40;https://en.wikipedia.org/wiki/Powerset_construction&#41; from an appropriate NFA.)

## Representation
Automata are represented as by their theoretical tuplet definition.

## Algorithm
1. Expression is parsed into a NFA
2. NFA is transformed into a DFA using a [powerset construction](https://en.wikipedia.org/wiki/Powerset_construction)
3. A run of the DFA on the input word is simulated

> Note: I will also include a naive and non-optimized backtracking approach. Given the correct NFA structure, this is a trivial task. 

## Future Scope
I may try my hands at a [Thompson NFA](https://swtch.com/~rsc/regexp/regexp1.html) at some point, to see how my approach stacks up.