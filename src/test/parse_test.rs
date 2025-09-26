#[cfg(test)]
mod test_parse {
    use crate::automata::{Automaton, Nfa, reset_state_counter};
    use crate::parse::parse;

    #[test]
    fn test_basic_syntax() {
        let pattern = "a*|(ab(a|b)*)b|b";
        let nfa = parse(&pattern);
        println!("{:?}", nfa);
        assert!(nfa.accept("a"));
        assert!(nfa.accept("aaaa"));
        assert!(nfa.accept("abb"));
        assert!(nfa.accept("abab"));
        assert!(nfa.accept("abbb"));
        assert!(nfa.accept("b"));
        assert!(!nfa.accept("c"));
        // not matching
        assert!(!nfa.accept("ac"));
        assert!(!nfa.accept("ba"));
        assert!(!nfa.accept("abaaabbbbbaaaabbbbbabbaabba"))
    }

    #[test]
    fn test_one_or_more() {
        let pattern = "a+";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);

        assert!(nfa.accept("a"));
        assert!(nfa.accept("aa"));
        assert!(!nfa.accept(""));
    }

    #[test]
    fn test_optional_quantifier() {
        let pattern = "a?";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);

        assert!(nfa.accept(""));
        assert!(nfa.accept("a"));
        assert!(!nfa.accept("aa"));
    }

    #[test]
    fn test_optional_quantifier_with_plus() {
        let pattern = "a?b+";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);

        assert!(nfa.accept("b"));
        assert!(nfa.accept("ab"));
        assert!(nfa.accept("abbb"));
        assert!(!nfa.accept(""));
        assert!(!nfa.accept("aab"));
    }

    #[test]
    fn test_optional_zeroone_optional() {
        let pattern = "b+c?";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);

        assert!(nfa.accept("b"));
        assert!(nfa.accept("b"));
        assert!(nfa.accept("bbb"));
        assert!(nfa.accept("bc"));
        assert!(nfa.accept("bc"));
        assert!(nfa.accept("bbc"));

        assert!(!nfa.accept(""));
        assert!(!nfa.accept("bcc"));
    }

    #[test]
    fn test_klenee_with_or() {
        let pattern = "(ab|cd)*";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);

        assert!(nfa.accept("abab"));
        assert!(nfa.accept("cdcd"));
        assert!(nfa.accept("abcd"));
        assert!(nfa.accept("cdab"));
        assert!(nfa.accept(""));
    }

    #[test]
    fn test_random_pattern_containing_second_iteration_syntax() {
        let pattern = "a?b+(a|c)?|c+";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);
        assert!(nfa.accept("b"));
        assert!(nfa.accept("aba"));
        assert!(nfa.accept("ba"));
        assert!(nfa.accept("c"));
        assert!(nfa.accept("ccccc"));
        assert!(nfa.accept("bc"));
        // not matching
        assert!(!nfa.accept(""));
        assert!(!nfa.accept("aab"));
        assert!(!nfa.accept("ac"));
    }

    #[test]
    fn test_fsa_uebung_2_39() {
        reset_state_counter();
        let pattern = "(a|b)?a*b";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);
        assert!(nfa.accept("b"));
        assert!(nfa.accept("ab"));
        assert!(nfa.accept("aaaab"));
        assert!(nfa.accept("baab"));
        assert!(nfa.accept("bb"));
        // not matching
        assert!(!nfa.accept(""));
        assert!(!nfa.accept("bbab"));
        assert!(!nfa.accept("ba"));
        assert!(!nfa.accept("a"));
    }

    #[test]
    fn test_pathological_backtracking_case() {
        let pattern = "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaa";
        let nfa = parse(&pattern);
        println!("{:?}", nfa);
        assert!(nfa.accept("aaaaaaaaaaaaaaaaaaaaaaaaa"))
    }
}
