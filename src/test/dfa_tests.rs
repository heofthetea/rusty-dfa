#[cfg(test)]
mod test_powerset_construction {
    use std::collections::{HashMap, HashSet};
    use crate::automata::{Automaton, Dfa, Nfa, Symbol};
    use crate::parse::parse;

    #[test]
    fn from_simple_nfa() {
        let nfa = Nfa::from_symbol(&Symbol::CHAR('a'));
        let dfa = Dfa::from(&nfa);
        println!("{:?}", dfa);
        assert!(dfa.validate().is_ok());
        assert!(dfa._accept(dfa.q_start, "a"));
        assert!(!dfa._accept(dfa.q_start, ""));
        assert!(!dfa._accept(dfa.q_start, "aa"));
        assert!(!dfa._accept(dfa.q_start, "b"));
    }
    
    #[test]
    fn from_concatenation() {
        let nfa = parse("abc");
        let dfa = Dfa::from(&nfa);
        println!("{:?}", nfa);
        println!("{:?}", dfa);
        assert!(dfa.validate().is_ok());
        assert!(dfa._accept(dfa.q_start, "abc"));
        assert!(!dfa._accept(dfa.q_start, ""));
        assert!(!dfa._accept(dfa.q_start, "ab"));
        assert!(!dfa._accept(dfa.q_start, "bc"));
        assert!(!dfa._accept(dfa.q_start, "ac"));
        assert!(!dfa._accept(dfa.q_start, "abcde"));
    }

    #[test]
    fn from_klenee_nfa() {
        let nfa = parse("a*");
        let dfa = Dfa::from(&nfa);
        println!("{:?}", nfa);
        println!("{:?}", dfa);
        assert!(dfa.validate().is_ok());
        assert!(dfa._accept(dfa.q_start, ""));
        assert!(dfa._accept(dfa.q_start, "a"));
        assert!(dfa._accept(dfa.q_start, "aa"));
        assert!(dfa._accept(dfa.q_start, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
        assert!(!dfa._accept(dfa.q_start, "aaab"));
        assert!(!dfa._accept(dfa.q_start, "b"));
    }

    #[test]
    fn from_disjunction() {
        let nfa = parse("a|b");
        let dfa = Dfa::from(&nfa);
        println!("{:?}", nfa);
        println!("{:?}", dfa);
        assert!(dfa.validate().is_ok());
        assert!(dfa.validate().is_ok());
        assert!(dfa._accept(dfa.q_start, "a"));
        assert!(dfa._accept(dfa.q_start, "b"));
        assert!(!dfa._accept(dfa.q_start, "aa"));
        assert!(!dfa._accept(dfa.q_start, ""));
        assert!(!dfa._accept(dfa.q_start, "ab"));
        assert!(!dfa._accept(dfa.q_start, "aaab"));
    }

}
/////////////////////////////////////////////////////// MATCHING ///////////////////////////////////////////////////////
/// We've asserted now that the DFA matches simple patterns correctly
/// time for the big guns

#[cfg(test)]
pub mod test_dfa_matching {
    use crate::automata::{Dfa, Automaton};
    use crate::parse::parse;

    #[test]
    fn test_fsa_uebung_2_39() {
        let pattern = "(a|b)?a*b";
        let nfa = parse(&pattern);
        let dfa = Dfa::from(&nfa);
        print!("{:?}", dfa);
        assert!(dfa._accept(dfa.q_start, "b"));
        assert!(dfa._accept(dfa.q_start, "ab"));
        assert!(dfa._accept(dfa.q_start, "aaaab"));
        assert!(dfa._accept(dfa.q_start, "baab"));
        assert!(dfa._accept(dfa.q_start, "bb"));
        // not matching
        assert!(!dfa._accept(dfa.q_start, ""));
        assert!(!dfa._accept(dfa.q_start, "bbab"));
        assert!(!dfa._accept(dfa.q_start, "ba"));
        assert!(!dfa._accept(dfa.q_start, "a"));
    }

    #[test]
    fn test_random_pattern_containing_second_iteration_syntax() {
        let pattern =  "a?b+(a|c)?|c+";
        let nfa = parse(&pattern);
        let dfa = Dfa::from(&nfa);
        print!("{:?}", dfa);
        assert!(dfa._accept(dfa.q_start, "b"));
        assert!(dfa._accept(dfa.q_start, "aba"));
        assert!(dfa._accept(dfa.q_start, "ba"));
        assert!(dfa._accept(dfa.q_start, "c"));
        assert!(dfa._accept(dfa.q_start, "ccccc"));
        assert!(dfa._accept(dfa.q_start, "bc"));
        // not matching
        assert!(!dfa._accept(dfa.q_start, ""));
        assert!(!dfa._accept(dfa.q_start, "aab"));
        assert!(!dfa._accept(dfa.q_start, "ac"));
    }

    /// LADIES AND GENTLEMEN
    /// WE GOT 'EM
    #[test]
    fn test_pathological_case_runs_in_reasonable_time() {
        let pattern = "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaa";
        let nfa = parse(&pattern);
        let dfa = Dfa::from(&nfa);
        println!("{:?}", dfa);
        assert!(dfa._accept(dfa.q_start, "aaaaaaaaaaaaaaaaaaaaaaaaa"))
    }

}
