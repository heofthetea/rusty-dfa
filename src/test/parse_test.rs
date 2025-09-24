#[cfg(test)]
mod test_parse {
    use crate::automata::{reset_state_counter, Automaton, Nfa};
    use crate::parse::parse;

    #[test]
    fn test_basic_syntax() {
        let pattern = "a*|(ab(a|b)*)b|b";
        let nfa = parse(&pattern);
        println!("{:?}", nfa);
        assert!(nfa._accept(nfa.q_start, "a"));
        assert!(nfa._accept(nfa.q_start, "aaaa"));
        assert!(nfa._accept(nfa.q_start, "abb"));
        assert!(nfa._accept(nfa.q_start, "abab"));
        assert!(nfa._accept(nfa.q_start, "abbb"));
        assert!(nfa._accept(nfa.q_start, "b"));
        assert!(!nfa._accept(nfa.q_start, "c"));
        // not matching
        assert!(!nfa._accept(nfa.q_start, "ac"));
        assert!(!nfa._accept(nfa.q_start, "ba"));
        assert!(!nfa._accept(nfa.q_start, "abaaabbbbbaaaabbbbbabbaabba"))
    }

    #[test]
    fn test_one_or_more() {
        let pattern = "a+";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);

        assert!(nfa._accept(nfa.q_start, "a"));
        assert!(nfa._accept(nfa.q_start, "aa"));
        assert!(!nfa._accept(nfa.q_start, ""));
    }

    #[test]
    fn test_optional_quantifier() {
        let pattern = "a?";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);

        assert!(nfa._accept(nfa.q_start, ""));
        assert!(nfa._accept(nfa.q_start, "a"));
        assert!(!nfa._accept(nfa.q_start, "aa"));
    }

    #[test]
    fn test_optional_quantifier_with_plus() {
        let pattern = "a?b+";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);

        assert!(nfa._accept(nfa.q_start, "b"));
        assert!(nfa._accept(nfa.q_start, "ab"));
        assert!(nfa._accept(nfa.q_start, "abbb"));
        assert!(!nfa._accept(nfa.q_start, ""));
        assert!(!nfa._accept(nfa.q_start, "aab"));
    }

    #[test]
    fn test_optional_zeroone_optional() {
        let pattern = "b+c?";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);

        assert!(nfa._accept(nfa.q_start, "b"));
        assert!(nfa._accept(nfa.q_start, "b"));
        assert!(nfa._accept(nfa.q_start, "bbb"));
        assert!(nfa._accept(nfa.q_start, "bc"));
        assert!(nfa._accept(nfa.q_start, "bc"));
        assert!(nfa._accept(nfa.q_start, "bbc"));

        assert!(!nfa._accept(nfa.q_start, ""));
        assert!(!nfa._accept(nfa.q_start, "bcc"));
    }

    #[test]
    fn test_random_pattern_containing_second_iteration_syntax() {
        let pattern =  "a?b+(a|c)?|c+";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);
        assert!(nfa._accept(nfa.q_start, "b"));
        assert!(nfa._accept(nfa.q_start, "aba"));
        assert!(nfa._accept(nfa.q_start, "ba"));
        assert!(nfa._accept(nfa.q_start, "c"));
        assert!(nfa._accept(nfa.q_start, "ccccc"));
        assert!(nfa._accept(nfa.q_start, "bc"));
        // not matching
        assert!(!nfa._accept(nfa.q_start, ""));
        assert!(!nfa._accept(nfa.q_start, "aab"));
        assert!(!nfa._accept(nfa.q_start, "ac"));
    }

    #[test]
    fn test_fsa_uebung_2_39() {
        reset_state_counter();
        let pattern = "(a|b)?a*b";
        let nfa = parse(&pattern);
        print!("{:?}", nfa);
        assert!(nfa._accept(nfa.q_start, "b"));
        assert!(nfa._accept(nfa.q_start, "ab"));
        assert!(nfa._accept(nfa.q_start, "aaaab"));
        assert!(nfa._accept(nfa.q_start, "baab"));
        assert!(nfa._accept(nfa.q_start, "bb"));
        // not matching
        assert!(!nfa._accept(nfa.q_start, ""));
        assert!(!nfa._accept(nfa.q_start, "bbab"));
        assert!(!nfa._accept(nfa.q_start, "ba"));
        assert!(!nfa._accept(nfa.q_start, "a"));
    }

    #[test]
    fn test_pathological_backtracking_case() {
        let pattern = "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaa";
        let nfa = parse(&pattern);
        println!("{:?}", nfa);
        assert!(nfa._accept(nfa.q_start, "aaaaaaaaaaaaaaaaaaaaaaaaa"))
    }
}