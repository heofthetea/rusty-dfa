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
    }
    
    #[test]
    fn from_concatenation() {
        let nfa = parse("abc");
        let dfa = Dfa::from(&nfa);
        println!("{:?}", nfa);
        println!("{:?}", dfa);
        assert!(dfa.validate().is_ok());
    }

    #[test]
    fn from_klenee_nfa() {
        let nfa = parse("a*");
        let dfa = Dfa::from(&nfa);
        println!("{:?}", nfa);
        println!("{:?}", dfa);
        assert!(dfa.validate().is_ok());
    }

    #[test]
    fn from_disjunction() {
        let nfa = parse("a|b");
        let dfa = Dfa::from(&nfa);
        println!("{:?}", nfa);
        println!("{:?}", dfa);
        assert!(dfa.validate().is_ok());
    }
}