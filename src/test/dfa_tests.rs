#[cfg(test)]
mod test_powerset_construction {
    use std::collections::{HashMap, HashSet};
    use crate::automata::{Automaton, Dfa, Nfa, Symbol};
    use crate::parse::parse;

    #[test]
    fn from_simple_nfa() {
        let nfa = Nfa::from_symbol(&Symbol::CHAR('a'));
        let dfa = Dfa::from(&nfa);
        let dfa_old = Dfa::from_old(&nfa);
        println!("{:?}", nfa);
        println!("new: {:?}", dfa);
        println!("old: {:?}", dfa_old);
        assert!(dfa.validate().is_ok());
        assert!(dfa_old.validate().is_ok());
    }
    
    #[test]
    fn from_concatenation() {
        let nfa = parse("abc");
        let dfa = Dfa::from(&nfa);
        let dfa_old = Dfa::from_old(&nfa);
        println!("{:?}", nfa);
        println!("new: {:?}", dfa);
        println!("old: {:?}", dfa_old);
        assert!(dfa.validate().is_ok());
        assert!(dfa_old.validate().is_ok());
    }

    #[test]
    fn from_klenee_nfa() {
        let nfa = parse("a*");
        let dfa = Dfa::from(&nfa);
        let dfa_old = Dfa::from_old(&nfa);
        println!("{:?}", nfa);
        println!("new: {:?}", dfa);
        println!("old: {:?}", dfa_old);
        assert!(dfa.validate().is_ok());
        assert!(dfa_old.validate().is_ok());
    }

    #[test]
    fn from_disjunction() {
        let nfa = parse("a|b");
        let dfa = Dfa::from(&nfa);
        let dfa_old = Dfa::from_old(&nfa);
        println!("{:?}", nfa);
        println!("{:?}", dfa);
        println!("{:?}", dfa_old);
        assert!(dfa.validate().is_ok());
        assert!(dfa_old.validate().is_ok());
    }
}