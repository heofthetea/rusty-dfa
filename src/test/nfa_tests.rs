#[cfg(test)]
mod test_nfa_construction {
    use crate::automata::{Automaton, Nfa, Symbol};
    use std::collections::HashSet;

    /// GIVEN: A valid NFA with 3 states, alphabet {a, b}, and transitions that accept strings containing 'a'
    /// WHEN: Constructing the NFA with valid parameters
    /// THEN: The NFA should be created successfully and validate without errors
    #[test]
    fn test_valid_nfa_construction() {
        let states = vec![0, 1, 2];
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

        let nfa = Nfa::from_symbol(&symbol);

        assert!(nfa.validate().is_ok());

        assert_eq!(nfa.states.len(), 2);
        assert_eq!(nfa.q_start, 0);
        assert_eq!(nfa.q_accepting, HashSet::from([1]));
        assert_eq!(nfa.transitions, HashSet::from([(0, Symbol::CHAR('x'), 1)]));
    }

    /// GIVEN: An epsilon symbol and an alphabet containing character 'a'
    /// WHEN: Constructing an NFA from the epsilon symbol
    /// THEN: The NFA should be valid and properly handle epsilon transitions
    #[test]
    fn test_from_symbol_epsilon() {
        let symbol = Symbol::EPSILON;

        let nfa = Nfa::from_symbol(&symbol);

        assert!(nfa.validate().is_ok());
    }

    /// GIVEN: An empty symbol and an alphabet containing character 'a'
    /// WHEN: Constructing an NFA from the empty symbol
    /// THEN: The NFA should be valid and represent the empty language
    #[test]
    fn test_from_symbol_empty() {
        let symbol = Symbol::EMPTY;

        let nfa = Nfa::from_symbol(&symbol);

        assert!(nfa.validate().is_ok());
    }

    /// GIVEN: A minimal configuration with one state that serves as both start and accepting state
    /// WHEN: Constructing an NFA with this minimal valid configuration
    /// THEN: The NFA should be created successfully and validate without errors
    #[test]
    fn test_minimal_valid_nfa() {
        let states = vec![0];
        let transitions = HashSet::new(); // No transitions
        let q_start = 0;
        let q_accepting = HashSet::from([0]); // Start state is also accepting

        let nfa = Nfa::new(states, transitions, q_start, q_accepting);
        assert!(nfa.validate().is_ok());
    }
}


#[cfg(test)]
mod test_nfa_combinations {
    use crate::automata::{Automaton, Nfa, Symbol, reset_state_counter};

    /// GIVEN: An NFA left accepting the language {"a"}
    /// GIVEN: An NFA right accepting the language {"b"}
    /// WHEN: the NFAs are concatenated
    /// THEN The resulting NFA accepts the language {"ab"}
    #[test]
    fn test_nfa_concatenation() {
        reset_state_counter();
        let mut left = Nfa::from_symbol(&Symbol::CHAR('a'));
        let right = Nfa::from_symbol(&Symbol::CHAR('b'));
        left.concat(right);
        assert!(left.validate().is_ok());
        println!("concat: {:?}", left);
        assert!(left._match(left.q_start, "ab"));
        assert!(!left._match(left.q_start, "a"));
        assert!(!left._match(left.q_start, "b"));
        assert!(!left._match(left.q_start, ""));
        assert!(!left._match(left.q_start, "abc"));
    }

    /// GIVEN: An NFA left accepting the language {"a"}
    /// GIVEN: An NFA right accepting the language {"b"}
    /// WHEN: the NFAs are unioned
    /// THEN: The resulting NFA accepts the language {"a", "b"}
    #[test]
    fn test_nfa_union() {
        reset_state_counter();
        let mut left = Nfa::from_symbol(&Symbol::CHAR('a'));
        let right = Nfa::from_symbol(&Symbol::CHAR('b'));
        left.union(right);
        assert!(left.validate().is_ok());
        println!("union: {:?}", left);
        assert!(left._match(left.q_start, "a"));
        assert!(left._match(left.q_start, "b"));
        assert!(!left._match(left.q_start, "ab"));
        assert!(!left._match(left.q_start, ""));
        assert!(!left._match(left.q_start, "c"));
        assert!(!left._match(left.q_start, "abc"));
    }

    /// GIVEN: An NFA accepting the language {"a"}
    /// WHEN: the Kleene star operation is applied
    /// THEN: The resulting NFA accepts the language {w | w \in {"a"}*}
    #[test]
    fn test_nfa_kleene() {
        reset_state_counter();
        let mut nfa = Nfa::from_symbol(&Symbol::CHAR('a'));
        nfa.klenee(true);
        assert!(nfa.validate().is_ok());
        println!("klenee: {:?}", nfa);
        assert!(nfa._match(nfa.q_start, ""));
        assert!(nfa._match(nfa.q_start, "a"));
        assert!(nfa._match(nfa.q_start, "aa"));
        assert!(nfa._match(nfa.q_start, "aaa"));
        assert!(!nfa._match(nfa.q_start, "b"));
        assert!(!nfa._match(nfa.q_start, "ab"));
        assert!(!nfa._match(nfa.q_start, "ba"));
    }
}

#[cfg(test)]
mod test_nfa_to_dfa {
    use std::collections::HashSet;
    use crate::automata::{Nfa, Symbol};

    /// GIVEN: An NFA with cyclical epsilon transitions
    /// WHEN: ec(0) is calculated
    /// THEN: ec(0) = {0, 1} and no infinite recursion occurs
    #[test]
    fn test_cyclic_ec() {
        let nfa = Nfa::new(
            vec![0, 1],
            HashSet::from([
                (0, Symbol::EPSILON, 1),
                (1, Symbol::EPSILON, 0)
            ]),
            0,
            HashSet::from([1])
        );
        let mut ec = HashSet::new();
        nfa.epsilon_closure(0, &mut ec);
        assert!(ec.contains(&0));
        assert!(ec.contains(&1));
    }

}
