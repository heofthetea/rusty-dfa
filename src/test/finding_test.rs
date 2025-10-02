/// Test whether the automatas finding algorithms work correctly.
/// Note: Finding with DFAs requires two DFAs to be constructed (one normally, one in reverse).
/// So those tests are going to have a bit more logic.

#[cfg(test)]
pub mod test_finding {
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

    #[test]
    fn test_klenee_in_the_middle() {
        let pattern = "ba*b";
        let mut nfa = parse(pattern);
        println!("{:?}", &nfa);
        assert_eq!(nfa.find("bb").unwrap(), (0, 1));
        assert_eq!(nfa.find("bab").unwrap(), (0, 2));
        assert_eq!(nfa.find("baaaaab").unwrap(), (0, 6));
        assert!(nfa.find("").is_none());
        assert!(nfa.find("a").is_none());
        assert!(nfa.find("b").is_none());
        assert!(nfa.find("ab").is_none());
        assert!(nfa.find("ba").is_none());

        let reversed = nfa.reversed();
        nfa.to_finding();
        let dfa = Dfa::from(&nfa);
        let dfa_reverse = Dfa::from(&reversed);
        println!("{:?}", &dfa);
        assert_eq!(dfa.find("bb", &dfa_reverse).unwrap(), (0, 1));
        assert_eq!(dfa.find("bab", &dfa_reverse).unwrap(), (0, 2));
        assert_eq!(dfa.find("baaaaab", &dfa_reverse).unwrap(), (0, 6));
        assert!(dfa.find("", &dfa_reverse).is_none());
        assert!(dfa.find("a", &dfa_reverse).is_none());
        assert!(dfa.find("b", &dfa_reverse).is_none());
        assert!(dfa.find("ab", &dfa_reverse).is_none());
        assert!(dfa.find("ba", &dfa_reverse).is_none());
    }


    #[test]
    fn test_absurd_case_lol() {
        let pattern =  "a?b+(a|c)?|c+";
        let mut nfa = parse(&pattern);
        let dfa_reversed = Dfa::from(&nfa.reversed());
        nfa.to_finding();
        let dfa = Dfa::from(&nfa);

        // println!("{:?}", dfa);
        println!("{:?}", dfa_reversed);
        assert_eq!(dfa.find("aab", &dfa_reversed).unwrap(), (1, 2));
        assert_eq!(dfa.find("aabbaa", &dfa_reversed).unwrap(), (1, 4));
        assert_eq!(dfa.find("cccba", &dfa_reversed).unwrap(), (0, 2));
        // assert_eq!(dfa.find("c", &dfa_reversed).unwrap(), (0, 0));
        // assert_eq!(dfa.find("ccccc", &dfa_reversed).unwrap(), (0, 0));
        // assert_eq!(dfa.find("bc", &dfa_reversed).unwrap(), (0, 0));
    }
}

#[cfg(test)]
pub mod test_make_matches {
    use crate::automata::make_matches;

    #[test]
    fn test_my_paper_example() {
        let starts: Vec<usize> = vec![1, 2, 4, 6, 9];
        let ends: Vec<usize> = vec![3, 7, 8, 11];
        assert_eq!(make_matches(&starts, &ends), vec![(1, 3), (4, 8), (9, 11)]);
    }
    #[test]
    fn test_end_equal_start() {
        let starts: Vec<usize> = vec![1, 4, 5];
        let ends: Vec<usize> = vec![3, 4, 7];
        assert_eq!(make_matches(&starts, &ends), vec![(1, 4), (5, 7)]);
    }
    #[test]
    fn test_klenee_like() {
        let starts: Vec<usize> = vec![3];
        let ends: Vec<usize> = vec![6, 7, 8, 9, 10, 11];
        assert_eq!(make_matches(&starts, &ends), vec![(3, 11)]);
    }
}
