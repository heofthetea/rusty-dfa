use crate::automata::{Automaton, Nfa, Symbol};

/// Parse `pattern` into a Non-deterministic Finite Automaton.
///
/// Uses an algorithm is an optimized version of a recursive descent, constructed over a grammar where
/// choices can be made deterministically without a need for backtracking.
/// The pattern is parsed by order of precedence according to the following grammar:
/// ```
/// EXPR -> <EXPR>|<DISJUNCT> / <DISJUNCT>
/// DISJUNCT -> <DISJUNCT><FACTOR> / <FACTOR>
/// FACTOR -> <ATOM>* / <ATOM>
/// ATOM -> (EXPR) / symbol
/// ```
/// Where `EXPR` is the start symbol.
///
/// For every NTS, a corresponding function exists tokenizing its passed string according to its production rules.
/// For example, the string `(a|b)|c` will be parsed by `_expr` into the strings `(a|b)` and `c`. These are then passed
/// to the `_disjunct` function, and the returned NFAs are unionized.
///
/// Note that there is a recursive pattern hidden here: An atom can be either a symbol, or a fully quallified
/// Regular Expression inside of parantheses. This recursion should not cause any overflows, as there is no backtracking
/// involved.
pub fn parse(pattern: &str) -> Nfa {
    _expr(pattern)
}

fn _expr(pattern: &str) -> Nfa {
    let tokens: Vec<String> = _tokenize_expr(pattern);
    let mut nfa = _disjunct(&tokens[0]);
    for token in &tokens[1..] {
        nfa.union(_disjunct(token));
    }
    nfa
}

fn _disjunct(disjunct: &str) -> Nfa {
    let tokens = _tokenize_disjunct(disjunct);
    let mut nfa = _factor(&tokens[0]);
    for token in &tokens[1..] {
        nfa.concat(_factor(token));
    }
    nfa
}

fn _factor(factor: &str) -> Nfa {
    // aaaaaaaaaaaaaaaaaa every solution to this is so ugly wtf
    let (atom, suffix) = match factor.chars().rev().next() {
        Some('*') | Some('+') | Some('?') if factor.len() > 1 => {
            (&factor[..factor.len() - 1], Some(factor.chars().last().unwrap()))
        }
        _ => (factor, None),
    };

    // we deliberately don't support non-greediness becaus that concept is irrelevant for a DFA based engine
    if suffix.is_some() && (atom.ends_with(|c| ['*', '+', '?'].contains(&c))) {
        panic!("Illegal stacking of quantifiers")
    }

    let mut nfa = _atom(atom);
    if let Some(c) = suffix {
        match c {
            '?' => {nfa.optional()}
            _ => nfa.klenee(c == '*')
        }
    }
    nfa
}

fn _atom(atom: &str) -> Nfa {
    // TODO: in the future, escape sequences need to be treated as atoms and handled accordingly
    // TODO: (this length check then isn't a reliable check anymore)
    if atom.len() == 1 {
        let symbol = Symbol::CHAR(atom.chars().nth(0).unwrap());
        return Nfa::from_symbol(&symbol);
    }
    return _expr(&atom[1..atom.len() - 1]);
}

///////////////////////////////////////////////////// TOKENIZATIONS ////////////////////////////////////////////////////

/// O(n)
fn _tokenize_expr(pattern: &str) -> Vec<String> {
    let mut tokens = vec!["".to_string()];
    // stack to keep track of encountered brackets
    let mut brackets: usize = 0;

    for c in pattern.chars() {
        if c == '(' {
            brackets += 1;
        }
        if c == ')' {
            if brackets == 0 {
                panic!("Unexpected ')'");
            }
            brackets -= 1;
        }
        // '|' actually encountered on root level and not deeper -> new token
        if c == '|' && brackets == 0 {
            tokens.push("".to_string());
            continue;
        }
        tokens.last_mut().unwrap().push(c);
    }
    tokens
}

/// O(n)
fn _tokenize_disjunct(pattern: &str) -> Vec<String> {
    // not allowed - quantifiers always need to reference a valid regular expression
    if pattern.starts_with(|c| ['*', '+', '?'].contains(&c)) {
        panic!("Nothing to quantify")
    }
    // safe to initialize empty,as first run wil ALWAYS perform a push
    let mut tokens: Vec<String> = Vec::new();
    let mut brackets: usize = 0;

    for c in pattern.chars() {
        // no new factor if we're either inside brackets or have a quantifier
        if brackets != 0 || ['*', '+', '?'].contains(&c) {
            let last = tokens.last_mut().unwrap();
            last.push(c);
        } else {
            tokens.push(c.to_string());
        }

        if c == '(' {
            brackets += 1;
        }
        if c == ')' {
            if brackets == 0 {
                panic!("Unexpected ')'");
            }
            brackets -= 1;
        }
    }
    tokens
}
