/// Test whether the automatas finding algorithms work correctly.
/// Note: Finding with DFAs requires two DFAs to be constructed (one normally, one in reverse).
/// So those tests are going to have a bit more logic.

#[cfg(test)]
pub mod test_finding {
    use crate::automata::{Automaton, Dfa};
    use crate::parse::parse;


    fn find_all_with_dfa(pattern: &str, input: &str) -> Option<Vec<(usize, usize)>> {
        let input_reversed: String = input.chars().rev().collect();

        let nfa = parse(&pattern);

        let dfa_reversed = Dfa::from(&nfa.reversed().to_finding());
        let dfa = Dfa::from(&nfa);
        dfa_reversed.find_all(&input_reversed, &dfa)
    }

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

        assert_eq!(*find_all_with_dfa(pattern, "ac").unwrap().first().unwrap(), (0, 1));
        assert_eq!(*find_all_with_dfa(pattern, "aac").unwrap().first().unwrap(), (1, 2));
        assert_eq!(*find_all_with_dfa(pattern, "abaac").unwrap().first().unwrap(), (3, 4));
        assert_eq!(*find_all_with_dfa(pattern, "cadaabf").unwrap().first().unwrap(), (3, 5));
        // not matching
        assert!(find_all_with_dfa(pattern, "ab").is_none());
        assert!(find_all_with_dfa(pattern, "abc").is_none());
        assert!(find_all_with_dfa(pattern, "cab").is_none());
    }

    #[test]
    fn test_abba() {

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

        assert_eq!(*find_all_with_dfa(pattern, "bc").unwrap().first().unwrap(), (0, 1));
        assert_eq!(*find_all_with_dfa(pattern, "ac").unwrap().first().unwrap(), (0, 1));
        assert_eq!(*find_all_with_dfa(pattern, "aac").unwrap().first().unwrap(), (1, 2));
        assert_eq!(*find_all_with_dfa(pattern, "ababc").unwrap().first().unwrap(), (3, 4));
        assert!(find_all_with_dfa(pattern, "ab").is_none());
        assert!(find_all_with_dfa(pattern, "cab").is_none());
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

        assert_eq!(*find_all_with_dfa(pattern, "b").unwrap().first().unwrap(), (0, 0));
        assert_eq!(*find_all_with_dfa(pattern, "ab").unwrap().first().unwrap(), (0, 1));
        assert_eq!(*find_all_with_dfa(pattern, "aaaaab").unwrap().first().unwrap(), (0, 5));
        assert!(find_all_with_dfa(pattern, "a").is_none());
        assert!(find_all_with_dfa(pattern, "").is_none());
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

        // assert_eq!(*find_all_with_dfa(pattern, "b").unwrap().first().unwrap(), (0, 0));
        // assert_eq!(*find_all_with_dfa(pattern, "ba").unwrap().first().unwrap(), (0, 1));
        assert_eq!(*find_all_with_dfa(pattern, "baaaaa").unwrap().first().unwrap(), (0, 5));
        assert!(find_all_with_dfa(pattern, "a").is_none());
        assert!(find_all_with_dfa(pattern, "").is_none());
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

        assert_eq!(*find_all_with_dfa(pattern, "bb").unwrap().first().unwrap(), (0, 1));
        assert_eq!(*find_all_with_dfa(pattern, "bab").unwrap().first().unwrap(), (0, 2));
        assert_eq!(*find_all_with_dfa(pattern, "baaaaab").unwrap().first().unwrap(), (0, 6));
        assert!(find_all_with_dfa(pattern, "").is_none());
        assert!(find_all_with_dfa(pattern, "a").is_none());
        assert!(find_all_with_dfa(pattern, "b").is_none());
        assert!(find_all_with_dfa(pattern, "ab").is_none());
        assert!(find_all_with_dfa(pattern, "ba").is_none());
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

        assert_eq!(*find_all_with_dfa(pattern, "aab").unwrap().first().unwrap(), (1, 2));
        assert_eq!(*find_all_with_dfa(pattern, "aabbaa").unwrap().first().unwrap(), (1, 4));
        assert_eq!(*find_all_with_dfa(pattern, "cccba").unwrap().first().unwrap(), (0, 2));
        assert_eq!(*find_all_with_dfa(pattern, "acba").unwrap().first().unwrap(), (1, 1));
        assert_eq!(*find_all_with_dfa(pattern, "bbbccc").unwrap().first().unwrap(), (0, 3));
        assert_eq!(
            *find_all_with_dfa(pattern, "the bbc is the british broadcasting network")
                .unwrap().first().unwrap(),
            (4, 6)
        );
        // no match
        assert!(find_all_with_dfa(pattern, "aa").is_none());
        assert!(find_all_with_dfa(pattern, "dfekjoei").is_none());
    }

    // #[test]
    // fn test_pathological_nfa() {
    //     use std::time::Instant;
    //
    //     let n = 256;
    //     let pattern = format!("{}{}", "a?".repeat(n), "a".repeat(n));
    //     let test_str = "a".repeat(n);
    //
    //     let start = Instant::now();
    //     let nfa = parse(&pattern);
    //     let construction_time = start.elapsed();
    //     println!("NFA construction took: {:?}", construction_time);
    //
    //     let start = Instant::now();
    //     let result = find_all_with_dfa(&pattern, &test_str).unwrap();
    //     let finding_time = start.elapsed();
    //     println!("DFA finding took: {:?}", finding_time);
    //
    //     assert_eq!(*result.first().unwrap(), (0, n - 1));
    // }

    #[test]
    fn test_mika() {
        let pattern = "aba";
        let input = "bababababa";
        println!("{:?}", find_all_with_dfa(pattern, input));

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
