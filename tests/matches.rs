use automaton::{dfa::DFA, dfa::subset::Subset};
use regex2nfa::regex2nfa;

fn matches<D: DFA<A = char>>(dfa: &D, input: &str) -> bool {
    let mut q = dfa.q_init();
    for c in input.chars() {
        q = dfa.q_next(&q, &c);
    }
    dfa.is_q_accept(&q)
}

#[test]
fn it_matches_precedence_concat_gt_union() {
    // a|bc  => a|(bc)
    let nfa = regex2nfa!("a|bc");
    let dfa = Subset(nfa);

    assert!(matches(&dfa, "a"));
    assert!(matches(&dfa, "bc"));
    assert!(!matches(&dfa, "ab"));
    assert!(!matches(&dfa, "ac"));
}

#[test]
fn it_matches_precedence_star_gt_concat() {
    // ab* => a(b*)
    let nfa = regex2nfa!("ab*");
    let dfa = Subset(nfa);

    assert!(matches(&dfa, "a"));
    assert!(matches(&dfa, "ab"));
    assert!(matches(&dfa, "abbb"));
    assert!(!matches(&dfa, "ababa"));
    assert!(!matches(&dfa, ""));
}

#[test]
fn it_matches_empty_star() {
    // (empty)*
    let nfa = regex2nfa!("()*");
    let dfa = Subset(nfa);

    assert!(matches(&dfa, ""));
    assert!(!matches(&dfa, "a"));
}

#[test]
fn it_matches_union_with_empty() {
    // a|empty
    let nfa = regex2nfa!("a|");
    let dfa = Subset(nfa);

    assert!(matches(&dfa, "a"));
    assert!(matches(&dfa, ""));
    assert!(!matches(&dfa, "b"));
}

#[test]
fn it_matches_nested_star() {
    // (a*)*
    let nfa = regex2nfa!("(a*)*");
    let dfa = Subset(nfa);

    assert!(matches(&dfa, ""));
    assert!(matches(&dfa, "a"));
    assert!(matches(&dfa, "aaaaa"));
    assert!(!matches(&dfa, "b"));
}

#[test]
fn it_matches_union_in_star() {
    // (a|a)*
    let nfa = regex2nfa!("(a|a)*");
    let dfa = Subset(nfa);

    assert!(matches(&dfa, ""));
    assert!(matches(&dfa, "a"));
    assert!(matches(&dfa, "aaaaa"));
    assert!(!matches(&dfa, "b"));
}

#[test]
fn it_matches_optional_match() {
    // (a|b)*abb
    let nfa = regex2nfa!("(a|b)*abb");
    let dfa = Subset(nfa);

    assert!(matches(&dfa, "abb"));
    assert!(matches(&dfa, "aabb"));
    assert!(matches(&dfa, "bababb"));
    assert!(!matches(&dfa, "ab"));
    assert!(!matches(&dfa, ""));
}
