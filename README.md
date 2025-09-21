# Rusty DFA
Regular Expression Engine using DFA construction, written in Rust.

> This is, primarily, another step in my journey to desperately learn Rust for my Studienarbeit lol

[//]: # (<br>The DFA is constructed via [powerset construction]&#40;https://en.wikipedia.org/wiki/Powerset_construction&#41; from an appropriate NFA.)

## Supported Syntax
This will never be a fully perl-comatible expression engine, for the very simple reason that it uses a DFA and thus cannot possibly
support backreferences. However, I'll probably also not include all of the syntactic sugar of deterministic perl-compatible expressions,
as a simple form of regular expressions are equally strong from a theoretical standpoint.

### Required Features to accept regular languages
- [x] Klenee Star quantifier (`a*`)
- [x] Union (`a|b`)
- [x] Explicit precedence (`(r)`)
- [ ] Escaping (`\r` where `r` is a reserved symbol)
- [ ] `^` and `$` quantifiers and making the automata behave correctly to accpet _parts_ of a word instead of the entire word

### Syntactic sugar
- [x] One-or-more quantifier (`a+`)
- [x] Zero-or-one quantifier (`a?`)
- [ ] Everything matcher (`.`)
- [ ] Character classes (`[ab]`)
  - [ ] Ranges (`[a-f]`)
- [ ] n-times (`r{n}`)
- [ ] Escape sequences for non-reserved symbols (`\d`)

### Practical Syntax not rooted in Language Theory
- [ ] Non-greedy quantifiers
- [ ] Capture groups


## Representation
Automata are represented as by their theoretical tuplet definition.
<br>Caveat: An automaton has no explicit alphabet associated with it. As I only care about accepting words, if I encounter a symbol that's not present in any 
transition it simply means the Automaton does not accept the word (i.e. not match in this case). 
This goes for both NFAs and DFAs (latter of which usually must have a transition for every symbol of the alphabet for every state).
When finding patterns in a string, an unknown symbol simply means the run is reset back to `q_start`.

## Algorithm
1. Expression is parsed into a NFA using a predictive recursive descent
2. NFA is transformed into a DFA using a [powerset construction](https://en.wikipedia.org/wiki/Powerset_construction)
3. A run of the DFA on the input word is simulated

> Note: I will also include a naive and non-optimized backtracking simulation of an NFA, as this is an easy byproduct of building NFAs correctly.

## Future Scope
I may try my hands at a [Thompson NFA](https://swtch.com/~rsc/regexp/regexp1.html) at some point, to see how my approach stacks up.

### Performance
One of my goals for this project is to make it as fast as possible (at the very least the actual matching, the construction is fine if it's a bit slower).
This shall serve as a list of things I think I could do but don't want to now (because I'm already thinking too much and make slow progress (usual me problem)):

- [ ] Make transition relation a `Vec<(Set<state, symbol>, Vec<state>)>` for O(len) iterations & better lookup
- 