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
        assert!(dfa.accept("a"));
        assert!(!dfa.accept(""));
        assert!(!dfa.accept("aa"));
        assert!(!dfa.accept("b"));
    }
    
    #[test]
    fn from_concatenation() {
        let nfa = parse("abc");
        let dfa = Dfa::from(&nfa);
        println!("{:?}", nfa);
        println!("{:?}", dfa);
        assert!(dfa.validate().is_ok());
        assert!(dfa.accept("abc"));
        assert!(!dfa.accept(""));
        assert!(!dfa.accept("ab"));
        assert!(!dfa.accept("bc"));
        assert!(!dfa.accept("ac"));
        assert!(!dfa.accept("abcde"));
    }

    #[test]
    fn from_klenee_nfa() {
        let nfa = parse("a*");
        let dfa = Dfa::from(&nfa);
        println!("{:?}", nfa);
        println!("{:?}", dfa);
        assert!(dfa.validate().is_ok());
        assert!(dfa.accept(""));
        assert!(dfa.accept("a"));
        assert!(dfa.accept("aa"));
        assert!(dfa.accept("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
        assert!(!dfa.accept("aaab"));
        assert!(!dfa.accept("b"));
    }

    #[test]
    fn from_disjunction() {
        let nfa = parse("a|b");
        let dfa = Dfa::from(&nfa);
        println!("{:?}", nfa);
        println!("{:?}", dfa);
        assert!(dfa.validate().is_ok());
        assert!(dfa.validate().is_ok());
        assert!(dfa.accept("a"));
        assert!(dfa.accept("b"));
        assert!(!dfa.accept("aa"));
        assert!(!dfa.accept(""));
        assert!(!dfa.accept("ab"));
        assert!(!dfa.accept("aaab"));
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
        assert!(dfa.accept("b"));
        assert!(dfa.accept("ab"));
        assert!(dfa.accept("aaaab"));
        assert!(dfa.accept("baab"));
        assert!(dfa.accept("bb"));
        // not matching
        assert!(!dfa.accept(""));
        assert!(!dfa.accept("bbab"));
        assert!(!dfa.accept("ba"));
        assert!(!dfa.accept("a"));
    }

    #[test]
    fn test_random_pattern_containing_second_iteration_syntax() {
        let pattern =  "a?b+(a|c)?|c+";
        let nfa = parse(&pattern);
        let dfa = Dfa::from(&nfa);
        print!("{:?}", dfa);
        assert!(dfa.accept("b"));
        assert!(dfa.accept("aba"));
        assert!(dfa.accept("ba"));
        assert!(dfa.accept("c"));
        assert!(dfa.accept("ccccc"));
        assert!(dfa.accept("bc"));
        // not matching
        assert!(!dfa.accept(""));
        assert!(!dfa.accept("aab"));
        assert!(!dfa.accept("ac"));
    }

    /// LADIES AND GENTLEMEN
    /// WE GOT 'EM
    #[test]
    fn test_pathological_case_runs_in_reasonable_time() {
        let pattern = "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaa";
        let nfa = parse(&pattern);
        let dfa = Dfa::from(&nfa);
        println!("{:?}", dfa);
        assert!(dfa.accept("aaaaaaaaaaaaaaaaaaaaaaaaa"))
    }

    //////////////////////////////////////////////////////// FINDING ///////////////////////////////////////////////////////
    #[test]
    fn find_easy() {
        let pattern = "a*";
        let dfa = Dfa::from(&parse(&pattern));
        
        assert_eq!(dfa.find("aaa").unwrap(), (0, 0));
    }
    
    #[test]
    fn i_think_this_will_break() {
        let pattern = "aab|ac";
        let dfa = Dfa::from(&parse(&pattern));
        print!("{:?}", dfa);
        
        assert!(dfa.find("aac").is_some());
    }

}
