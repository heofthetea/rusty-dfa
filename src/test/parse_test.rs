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
/////////////////////////////////////////////////// PARSE FOR FINDING //////////////////////////////////////////////////

#[cfg(test)]
pub mod test_parse_for_finding {
    use crate::automata::{Automaton, Dfa};
    use crate::parse::{parse, parse_for_dfa_finding};

    #[test]
    fn test_simple_find() {
        let pattern = "aab|ac";
        let nfa = parse(pattern);
        println!("{:?}", &nfa);
        assert_eq!(nfa.find("aab").unwrap(), (0, 2));
        assert_eq!(nfa.find("ac").unwrap(), (0, 1));
        assert_eq!(nfa.find("aac").unwrap(), (1, 2));
        assert_eq!(nfa.find("abaac").unwrap(), (3, 4));
        assert_eq!(nfa.find("cadaabf").unwrap(), (3, 5));
        assert!(nfa.find("ab").is_none());
        assert!(nfa.find("abc").is_none());
        assert!(nfa.find("cab").is_none());

        let nfa = &parse_for_dfa_finding(pattern);
        // dfa
        let dfa = Dfa::from(&nfa);
        println!("{:?}", &dfa);
        assert_eq!(dfa.find("ac").unwrap(), (0, 1));
        assert_eq!(dfa.find("aac").unwrap(), (1, 2));
        assert_eq!(dfa.find("abaac").unwrap(), (3, 4));
        assert_eq!(dfa.find("cadaabf").unwrap(), (3, 5));
        // not matching
        assert!(dfa.find("ab").is_none());
        assert!(dfa.find("abc").is_none());
        assert!(dfa.find("cab").is_none());
    }
}
