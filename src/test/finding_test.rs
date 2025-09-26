/// Test whether the automatas finding algorithms work correctly.
/// Note: Finding with DFAs requires two DFAs to be constructed (one normally, one in reverse).
/// So those tests are going to have a bit more logic.

#[cfg(test)]
pub mod test_for_finding {
    use crate::automata::{Automaton, Dfa};
    use crate::parse::{parse};

    #[test]
    fn test_simple_find() {
        let pattern = "aab|ac";
        let mut nfa = parse(pattern);
        println!("{:?}", &nfa);
        assert_eq!(nfa.find("aab").unwrap(), (0, 2));
        assert_eq!(nfa.find("ac").unwrap(), (0, 1));
        assert_eq!(nfa.find("aac").unwrap(), (1, 2));
        assert_eq!(nfa.find("abaac").unwrap(), (3, 4));
        assert_eq!(nfa.find("cadaabf").unwrap(), (3, 5));
        assert!(nfa.find("ab").is_none());
        assert!(nfa.find("abc").is_none());
        assert!(nfa.find("cab").is_none());

        // dfa
        let reversed = nfa.reversed();
        nfa.to_finding();
        // ah yes the double powerset constructions very efficient
        let dfa = Dfa::from(&nfa);
        let dfa_reverse = Dfa::from(&reversed);
        println!("{:?}", &dfa);
        assert_eq!(dfa.find("ac", &dfa_reverse).unwrap(), (0, 1));
        assert_eq!(dfa.find("aac", &dfa_reverse).unwrap(), (1, 2));
        assert_eq!(dfa.find("abaac", &dfa_reverse).unwrap(), (3, 4));
        assert_eq!(dfa.find("cadaabf", &dfa_reverse).unwrap(), (3, 5));
        // not matching
        assert!(dfa.find("ab", &dfa_reverse).is_none());
        assert!(dfa.find("abc", &dfa_reverse).is_none());
        assert!(dfa.find("cab", &dfa_reverse).is_none());
    }
    #[test]
    fn test_find_with_precedence() {
        let pattern = "(a|b)c";
        let mut nfa = parse(pattern);
        println!("{:?}", &nfa);
        assert_eq!(nfa.find("bc").unwrap(), (0, 1));
        assert_eq!(nfa.find("ac").unwrap(), (0, 1));
        assert_eq!(nfa.find("aac").unwrap(), (1, 2));
        assert_eq!(nfa.find("ababc").unwrap(), (3, 4));
        assert!(nfa.find("ab").is_none());
        assert!(nfa.find("cab").is_none());

        let reversed = nfa.reversed();
        nfa.to_finding();
        let dfa = Dfa::from(&nfa);
        let dfa_reverse = Dfa::from(&reversed);
        println!("{:?}", &dfa);
        assert_eq!(dfa.find("bc", &dfa_reverse).unwrap(), (0, 1));
        assert_eq!(dfa.find("ac", &dfa_reverse).unwrap(), (0, 1));
        assert_eq!(dfa.find("aac", &dfa_reverse).unwrap(), (1, 2));
        assert_eq!(dfa.find("ababc", &dfa_reverse).unwrap(), (3, 4));
        assert!(dfa.find("ab", &dfa_reverse).is_none());
        assert!(dfa.find("cab", &dfa_reverse).is_none());

    }

    // if this doesn't kill me I'll kiss the rust crab
    #[test]
    fn test_klenee_aka_im_dead() {
        let pattern = "a*b";
        let mut nfa = parse(pattern);
        println!("{:?}", &nfa);
        assert_eq!(nfa.find("b").unwrap(), (0, 0));
        assert_eq!(nfa.find("ab").unwrap(), (0, 1));
        assert_eq!(nfa.find("aaaaab").unwrap(), (0, 5));
        assert!(nfa.find("a").is_none());
        assert!(nfa.find("").is_none());

        let reversed = nfa.reversed();
        nfa.to_finding();
        let dfa = Dfa::from(&nfa);
        let dfa_reverse = Dfa::from(&reversed);
        println!("{:?}", &dfa);
        assert_eq!(dfa.find("b", &dfa_reverse).unwrap(), (0, 0));
        assert_eq!(dfa.find("ab", &dfa_reverse).unwrap(), (0, 1));
        assert_eq!(dfa.find("aaaaab", &dfa_reverse).unwrap(), (0, 5));
        assert!(dfa.find("a", &dfa_reverse).is_none());
        assert!(dfa.find("", &dfa_reverse).is_none());
    }
}
