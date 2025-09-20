use crate::automata::{Automaton, Nfa, Symbol};

/// Parse `pattern` into a Non-deterministic finite automaton.
///
/// Uses an algorithm that _may_ be derived from a recursive descent, however there's no Backtracking involved.
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

/// This function is my demise I just can't get this clean AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
fn _factor(factor: &str) -> Nfa {
    let limit: usize = if factor.ends_with('*') {
        factor.len() - 1
    } else {
        factor.len()
    };
    if factor.ends_with("**") {
        panic!("Encountered two '*' back to back");
    }
    let mut nfa = _atom(&factor[..limit]);
    if limit != factor.len() {
        nfa.klenee();
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
    // not allowed - klenee star always nees to reference a valid regular expression
    if pattern.starts_with("*") {
        panic!("Unexpected '*'")
    }
    // safe to initialize empty,as first run wil ALWAYS perform a push
    let mut tokens: Vec<String> = Vec::new();
    let mut brackets: usize = 0;

    for c in pattern.chars() {
        // no new factor if we're either inside brackets or have a klenee star
        if brackets != 0 || c == '*' {
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
