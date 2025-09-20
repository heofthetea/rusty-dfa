#[cfg(test)]
mod test_parse {
    use crate::automata::{Automaton, Nfa};
    use crate::parse::parse;

    #[test]
    fn test_random_pattern_containing_all_syntax() {
        let pattern = "a*|(ab(a|b)*)b|b";
        let nfa = parse(&pattern);
        println!("{:?}", nfa);
        assert!(nfa._match(nfa.q_start,"a"));
        assert!(nfa._match(nfa.q_start,"aaaa"));
        assert!(nfa._match(nfa.q_start,"abb"));
        assert!(nfa._match(nfa.q_start,"abab"));
        assert!(nfa._match(nfa.q_start,"abbb"));
        assert!(nfa._match(nfa.q_start,"b"));
        assert!(!nfa._match(nfa.q_start,"c"));
        assert!(!nfa._match(nfa.q_start,"ac"));
        assert!(!nfa._match(nfa.q_start,"ba"));
        assert!(!nfa._match(nfa.q_start, "abaaabbbbbaaaabbbbbabbaabba"))
    }
}