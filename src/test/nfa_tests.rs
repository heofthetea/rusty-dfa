#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::automata::{Nfa, Automaton, Symbol};

    /// GIVEN: A valid NFA with 3 states, alphabet {a, b}, and transitions that accept strings containing 'a'
    /// WHEN: Constructing the NFA with valid parameters
    /// THEN: The NFA should be created successfully and validate without errors
    #[test]
    fn test_valid_nfa_construction() {
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

        let nfa = Nfa::new(states, transitions, q_start, q_accepting);

        assert!(nfa.validate().is_ok());
    }

    /// GIVEN: States [0, 1, 2] and a start state that is not in the states set
    /// WHEN: Attempting to construct an NFA with invalid start state (5)
    /// THEN: The constructor should panic with "q_0 ∉ Q" error message
    #[test]
    #[should_panic(expected = "Requested construction of invalid NFA: q_0 ∉ Q")]
    fn test_invalid_start_state() {
        let states = vec![0, 1, 2];
        let alphabet = HashSet::from([Symbol::CHAR('a')]);
        let transitions = HashSet::from([(0, Symbol::CHAR('a'), 1)]);
        let q_start = 5; // Invalid: not in states
        let q_accepting = HashSet::from([1]);

        Nfa::new(states, transitions, q_start, q_accepting);
    }

    /// GIVEN: States [0, 1, 2] and accepting states that include a state not in the states set
    /// WHEN: Attempting to construct an NFA with invalid accepting state (5)
    /// THEN: The constructor should panic with "F ⊄ Q" error message
    #[test]
    #[should_panic(expected = "Requested construction of invalid NFA: F ⊄ Q")]
    fn test_invalid_accepting_states() {
        let states = vec![0, 1, 2];
        let alphabet = HashSet::from([Symbol::CHAR('a')]);
        let transitions = HashSet::from([(0, Symbol::CHAR('a'), 1)]);
        let q_start = 0;
        let q_accepting = HashSet::from([1, 5]); // Invalid: 5 is not in states

        Nfa::new(states, transitions, q_start, q_accepting);
    }

    /// GIVEN: States [0, 1, 2] and a transition with a from-state not in the states set
    /// WHEN: Attempting to construct an NFA with invalid transition from state (5)
    /// THEN: The constructor should panic with "has invalid state(s)" error message
    #[test]
    #[should_panic(expected = "has invalid state(s)")]
    fn test_invalid_transition_from_state() {
        let states = vec![0, 1, 2];
        let alphabet = HashSet::from([Symbol::CHAR('a')]);
        let transitions = HashSet::from([
            (0, Symbol::CHAR('a'), 1),
            (5, Symbol::CHAR('a'), 2), // Invalid: from state 5 is not in states
        ]);
        let q_start = 0;
        let q_accepting = HashSet::from([1]);

        Nfa::new(states, transitions, q_start, q_accepting);
    }

    /// GIVEN: States [0, 1, 2] and a transition with a to-state not in the states set
    /// WHEN: Attempting to construct an NFA with invalid transition to state (7)
    /// THEN: The constructor should panic with "has invalid state(s)" error message
    #[test]
    #[should_panic(expected = "has invalid state(s)")]
    fn test_invalid_transition_to_state() {
        let states = vec![0, 1, 2];
        let alphabet = HashSet::from([Symbol::CHAR('a')]);
        let transitions = HashSet::from([
            (0, Symbol::CHAR('a'), 1),
            (1, Symbol::CHAR('a'), 7), // Invalid: to state 7 is not in states
        ]);
        let q_start = 0;
        let q_accepting = HashSet::from([1]);

        Nfa::new(states, transitions, q_start, q_accepting);
    }

    /// GIVEN: A character symbol 'x' and an alphabet containing 'x' and 'y'
    /// WHEN: Constructing an NFA from the character symbol
    /// THEN: The NFA should be valid and properly constructed with 2 states and one transition
    #[test]
    fn test_from_symbol_char() {
        let symbol = Symbol::CHAR('x');
        let alphabet = HashSet::from([Symbol::CHAR('x'), Symbol::CHAR('y')]);

        let nfa = Nfa::from_symbol(&symbol);

        assert!(nfa.validate().is_ok());

        assert_eq!(nfa.states.len(), 2);
        assert_eq!(nfa.q_start, 0);
        assert_eq!(nfa.q_accepting, HashSet::from([1]));
        assert_eq!(
            nfa.transitions,
            HashSet::from([(0, Symbol::CHAR('x'), 1)])
        );
    }

    /// GIVEN: An epsilon symbol and an alphabet containing character 'a'
    /// WHEN: Constructing an NFA from the epsilon symbol
    /// THEN: The NFA should be valid and properly handle epsilon transitions
    #[test]
    fn test_from_symbol_epsilon() {
        let symbol = Symbol::EPSILON;
        let alphabet = HashSet::from([Symbol::CHAR('a')]);

        let nfa = Nfa::from_symbol(&symbol);

        assert!(nfa.validate().is_ok());
    }

    /// GIVEN: An empty symbol and an alphabet containing character 'a'
    /// WHEN: Constructing an NFA from the empty symbol
    /// THEN: The NFA should be valid and represent the empty language
    #[test]
    fn test_from_symbol_empty() {
        let symbol = Symbol::EMPTY;
        let alphabet = HashSet::from([Symbol::CHAR('a')]);

        let nfa = Nfa::from_symbol(&symbol);

        assert!(nfa.validate().is_ok());
    }

    /// GIVEN: A minimal configuration with one state that serves as both start and accepting state
    /// WHEN: Constructing an NFA with this minimal valid configuration
    /// THEN: The NFA should be created successfully and validate without errors
    #[test]
    fn test_minimal_valid_nfa() {
        let states = vec![0];
        let alphabet = HashSet::from([Symbol::CHAR('a')]);
        let transitions = HashSet::new(); // No transitions
        let q_start = 0;
        let q_accepting = HashSet::from([0]); // Start state is also accepting

        let nfa = Nfa::new(states, transitions, q_start, q_accepting);
        assert!(nfa.validate().is_ok());
    }
}
