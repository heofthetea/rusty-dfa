#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::automata::{Nfa, Automaton, Symbol};

    #[test]
    fn test_valid_nfa_construction() {
        // Test 1: A simple valid NFA with 3 states
        // This NFA accepts strings that contain 'a'
        let states = vec![0, 1, 2];
        let alphabet = HashSet::from([Symbol::CHAR('a'), Symbol::CHAR('b')]);
        let transitions = HashSet::from([
            (0, Symbol::CHAR('a'), 1),
            (0, Symbol::CHAR('b'), 0),
            (1, Symbol::CHAR('a'), 1),
            (1, Symbol::CHAR('b'), 1),
        ]);
        let q_start = 0;
        let q_accepting = HashSet::from([1]);

        // This should construct successfully without panicking
        let nfa = Nfa::new(states, alphabet, transitions, q_start, q_accepting);

        // Verify it validates correctly
        assert!(nfa.validate().is_ok());
    }

    #[test]
    #[should_panic(expected = "Requested construction of invalid NFA: q_0 ∉ Q")]
    fn test_invalid_start_state() {
        // Test violation of condition: q_start must be in states
        let states = vec![0, 1, 2];
        let alphabet = HashSet::from([Symbol::CHAR('a')]);
        let transitions = HashSet::from([(0, Symbol::CHAR('a'), 1)]);
        let q_start = 5; // Invalid: not in states
        let q_accepting = HashSet::from([1]);

        // This should panic due to invalid start state
        Nfa::new(states, alphabet, transitions, q_start, q_accepting);
    }

    #[test]
    #[should_panic(expected = "Requested construction of invalid NFA: F ⊄ Q")]
    fn test_invalid_accepting_states() {
        // Test violation of condition: all accepting states must be in states
        let states = vec![0, 1, 2];
        let alphabet = HashSet::from([Symbol::CHAR('a')]);
        let transitions = HashSet::from([(0, Symbol::CHAR('a'), 1)]);
        let q_start = 0;
        let q_accepting = HashSet::from([1, 5]); // Invalid: 5 is not in states

        // This should panic due to invalid accepting state
        Nfa::new(states, alphabet, transitions, q_start, q_accepting);
    }

    #[test]
    #[should_panic(expected = "has invalid state(s)")]
    fn test_invalid_transition_from_state() {
        // Test violation of condition: transition from state must be in states
        let states = vec![0, 1, 2];
        let alphabet = HashSet::from([Symbol::CHAR('a')]);
        let transitions = HashSet::from([
            (0, Symbol::CHAR('a'), 1),
            (5, Symbol::CHAR('a'), 2), // Invalid: from state 5 is not in states
        ]);
        let q_start = 0;
        let q_accepting = HashSet::from([1]);

        // This should panic due to invalid transition from state
        Nfa::new(states, alphabet, transitions, q_start, q_accepting);
    }

    #[test]
    #[should_panic(expected = "has invalid state(s)")]
    fn test_invalid_transition_to_state() {
        // Test violation of condition: transition to state must be in states
        let states = vec![0, 1, 2];
        let alphabet = HashSet::from([Symbol::CHAR('a')]);
        let transitions = HashSet::from([
            (0, Symbol::CHAR('a'), 1),
            (1, Symbol::CHAR('a'), 7), // Invalid: to state 7 is not in states
        ]);
        let q_start = 0;
        let q_accepting = HashSet::from([1]);

        // This should panic due to invalid transition to state
        Nfa::new(states, alphabet, transitions, q_start, q_accepting);
    }

    #[test]
    fn test_from_symbol_char() {
        // Test constructing NFA from a character symbol
        let symbol = Symbol::CHAR('x');
        let alphabet = HashSet::from([Symbol::CHAR('x'), Symbol::CHAR('y')]);

        let nfa = Nfa::from_symbol(&symbol, alphabet.clone());

        // Verify the NFA is valid
        assert!(nfa.validate().is_ok());

        // The NFA should have exactly 2 states (0 and 1)
        // Start state should be 0, accepting state should be 1
        // Should have one transition from 0 to 1 on 'x'
    }

    #[test]
    fn test_from_symbol_epsilon() {
        // Test constructing NFA from epsilon symbol
        let symbol = Symbol::EPSILON;
        let alphabet = HashSet::from([Symbol::CHAR('a')]);

        let nfa = Nfa::from_symbol(&symbol, alphabet);

        // Verify the NFA is valid
        assert!(nfa.validate().is_ok());
    }

    #[test]
    fn test_from_symbol_empty() {
        // Test constructing NFA from empty symbol
        let symbol = Symbol::EMPTY;
        let alphabet = HashSet::from([Symbol::CHAR('a')]);

        let nfa = Nfa::from_symbol(&symbol, alphabet);

        // Verify the NFA is valid
        assert!(nfa.validate().is_ok());
    }

    #[test]
    fn test_minimal_valid_nfa() {
        // Test a minimal valid NFA with just one state
        let states = vec![0];
        let alphabet = HashSet::from([Symbol::CHAR('a')]);
        let transitions = HashSet::new(); // No transitions
        let q_start = 0;
        let q_accepting = HashSet::from([0]); // Start state is also accepting

        let nfa = Nfa::new(states, alphabet, transitions, q_start, q_accepting);
        assert!(nfa.validate().is_ok());
    }
}
