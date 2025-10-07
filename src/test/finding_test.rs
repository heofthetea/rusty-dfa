/// Test whether the automatas finding algorithms work correctly.
/// Note: Finding with DFAs requires two DFAs to be constructed (one normally, one in reverse).
/// So those tests are going to have a bit more logic.

#[cfg(test)]
pub mod test_finding {
    use crate::automata::{find_all_with_dfa_i_hate_my_life, Automaton, Dfa};
    use crate::parse::parse;

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

        assert_eq!(*find_all_with_dfa_i_hate_my_life(pattern, "ac").unwrap().first().unwrap(), (0, 1));
        // assert_eq!(*find_all_with_dfa_i_hate_my_life(pattern, "caa").unwrap().first().unwrap(), (1, 2));
        // assert_eq!(*find_all_with_dfa_i_hate_my_life(pattern, "caaba").unwrap().first().unwrap(), (3, 4));
        // assert_eq!(*find_all_with_dfa_i_hate_my_life(pattern, "fbaadac").unwrap().first().unwrap(), (3, 5));
        // // not matching
        // assert!(find_all_with_dfa_i_hate_my_life(pattern, "ba").unwrap().is_empty());
        // assert!(find_all_with_dfa_i_hate_my_life(pattern, "cba").unwrap().is_empty());
        // assert!(find_all_with_dfa_i_hate_my_life(pattern, "bac").unwrap().is_empty());
    }
    #[test]
    fn test_find_with_precedence() {
        let pattern = "(a|b)c";
        let nfa = parse(pattern);
        println!("{:?}", &nfa);
        assert_eq!(nfa.find("bc").unwrap(), (0, 1));
        assert_eq!(nfa.find("ac").unwrap(), (0, 1));
        assert_eq!(nfa.find("aac").unwrap(), (1, 2));
        assert_eq!(nfa.find("ababc").unwrap(), (3, 4));
        assert!(nfa.find("ab").is_none());
        assert!(nfa.find("cab").is_none());

        let reversed = nfa.reversed();
        let dfa = Dfa::from(&nfa.to_finding());
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
        let nfa = parse(pattern);
        println!("{:?}", &nfa);
        assert_eq!(nfa.find("b").unwrap(), (0, 0));
        assert_eq!(nfa.find("ab").unwrap(), (0, 1));
        assert_eq!(nfa.find("aaaaab").unwrap(), (0, 5));
        assert!(nfa.find("a").is_none());
        assert!(nfa.find("").is_none());

        let reversed = nfa.reversed();
        let dfa = Dfa::from(&nfa.to_finding());
        let dfa_reverse = Dfa::from(&reversed);
        println!("{:?}", &dfa);
        assert_eq!(dfa.find("b", &dfa_reverse).unwrap(), (0, 0));
        assert_eq!(dfa.find("ab", &dfa_reverse).unwrap(), (0, 1));
        assert_eq!(dfa.find("aaaaab", &dfa_reverse).unwrap(), (0, 5));
        assert!(dfa.find("a", &dfa_reverse).is_none());
        assert!(dfa.find("", &dfa_reverse).is_none());
    }
    #[test]
    fn test_klenee_aka_im_dead_2() {
        let pattern = "ba*";
        let nfa = parse(pattern);
        println!("{:?}", &nfa);
        assert_eq!(nfa.find("b").unwrap(), (0, 0));
        assert_eq!(nfa.find("ba").unwrap(), (0, 1));
        assert_eq!(nfa.find("baaaaa").unwrap(), (0, 5));
        assert!(nfa.find("a").is_none());
        assert!(nfa.find("").is_none());

        let reversed = nfa.reversed();
        let dfa = Dfa::from(&nfa.to_finding());
        let dfa_reverse = Dfa::from(&reversed);
        println!("{:?}", &dfa);
        assert_eq!(dfa.find("b", &dfa_reverse).unwrap(), (0, 0));
        assert_eq!(dfa.find("ba", &dfa_reverse).unwrap(), (0, 1));
        assert_eq!(dfa.find("baaaaa", &dfa_reverse).unwrap(), (0, 5));
        assert!(dfa.find("a", &dfa_reverse).is_none());
        assert!(dfa.find("", &dfa_reverse).is_none());
    }

    #[test]
    fn test_klenee_in_the_middle() {
        let pattern = "ba*b";
        let nfa = parse(pattern);
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
        let dfa = Dfa::from(&nfa.to_finding());
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
        let pattern = "a?b+(a|c)?|c+";
        let nfa = parse(&pattern);
        assert_eq!(nfa.find("aab").unwrap(), (1, 2));
        assert_eq!(nfa.find("aabbaa").unwrap(), (1, 4));
        assert_eq!(nfa.find("cccba").unwrap(), (0, 2));
        assert_eq!(nfa.find("acba").unwrap(), (1, 1));
        assert_eq!(nfa.find("bbbccc").unwrap(), (0, 3));
        assert_eq!(
            nfa.find("the bbc is the british broadcasting network")
                .unwrap(),
            (4, 6)
        );
        // no match
        assert!(nfa.find("aa").is_none());
        assert!(nfa.find("dfekjoei").is_none());

        let dfa_reversed = Dfa::from(&nfa.reversed());
        let dfa = Dfa::from(&nfa.to_finding());

        // println!("{:?}", benchmark);
        assert_eq!(dfa.find("aab", &dfa_reversed).unwrap(), (1, 2));
        assert_eq!(dfa.find("aabbaa", &dfa_reversed).unwrap(), (1, 4));
        assert_eq!(dfa.find("cccba", &dfa_reversed).unwrap(), (0, 2));
        assert_eq!(dfa.find("acba", &dfa_reversed).unwrap(), (1, 1));
        assert_eq!(dfa.find("bbbccc", &dfa_reversed).unwrap(), (0, 3));
        assert_eq!(
            dfa.find("the bbc is the british broadcasting network", &dfa_reversed)
                .unwrap(),
            (4, 6)
        );
        // no match
        assert!(dfa.find("aa", &dfa_reversed).is_none());
        assert!(dfa.find("dfekjoei", &dfa_reversed).is_none());
    }

    #[test]
    fn test_pathological_nfa() {
        use std::time::Instant;

        let n = 256;
        let pattern = format!("{}{}", "a?".repeat(n), "a".repeat(n));
        let test_str = "a".repeat(n);

        let start = Instant::now();
        let nfa = parse(&pattern);
        let dfa_reversed = Dfa::from(&nfa.reversed());
        let dfa = Dfa::from(&nfa.to_finding());
        let construction_time = start.elapsed();
        println!("Construction took: {:?}", construction_time);

        let start = Instant::now();
        let result = dfa.find(&test_str, &dfa_reversed).unwrap();
        let finding_time = start.elapsed();
        println!("Finding took: {:?}", finding_time);

        assert_eq!(result, (0, n - 1));
    }

    #[test]
    fn test_mika() {
        let pattern = "aba";
        let input = "bababababa";
        println!("{:?}", find_all_with_dfa_i_hate_my_life(pattern, input));

        // assert_eq!(dfa.find("aab", &dfa_reversed).unwrap(), (1, 2));
        // assert_eq!(dfa.find("aabbaa", &dfa_reversed).unwrap(), (1, 4));
        // assert_eq!(dfa.find("cccba", &dfa_reversed).unwrap(), (0, 2));
        // assert_eq!(dfa.find("acba", &dfa_reversed).unwrap(), (1, 1));
        // assert_eq!(dfa.find("bbbccc", &dfa_reversed).unwrap(), (0, 3));
        // assert_eq!(
        //     dfa.find("the bbc is the british broadcasting network", &dfa_reversed)
        //        .unwrap(),
        //     (4, 6)
        // );
        // // no match
        // assert!(dfa.find("aa", &dfa_reversed).is_none());
        // assert!(dfa.find("dfekjoei", &dfa_reversed).is_none());

    }
}
