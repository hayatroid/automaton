use automaton::nfa::{concat::Concat, empty::Empty, star::Star, symbol::Symbol, union::Union};
use regex2nfa::regex2nfa;

#[test]
fn it_parses_precedence_concat_gt_union() {
    // a|bc  => a|(bc)
    let nfa = regex2nfa!("a|bc");
    assert!(matches!(
        nfa,
        Union(Symbol('a'), Concat(Symbol('b'), Symbol('c')))
    ));
}

#[test]
fn it_parses_precedence_star_gt_concat() {
    // ab* => a(b*)
    let nfa = regex2nfa!("ab*");
    assert!(matches!(nfa, Concat(Symbol('a'), Star(Symbol('b')))));
}

#[test]
fn it_parses_empty_star() {
    // (empty)*
    let nfa = regex2nfa!("()*");
    assert!(matches!(nfa, Star(Empty(..))));
}

#[test]
fn it_parses_union_with_empty() {
    // a|empty
    let nfa = regex2nfa!("a|");
    assert!(matches!(nfa, Union(Symbol('a'), Empty(..))));
}

#[test]
fn it_parses_nested_star() {
    // (a*)*
    let nfa = regex2nfa!("(a*)*");
    assert!(matches!(nfa, Star(Star(Symbol('a')))));
}

#[test]
fn it_parses_union_in_star() {
    // (a|a)*
    let nfa = regex2nfa!("(a|a)*");
    assert!(matches!(nfa, Star(Union(Symbol('a'), Symbol('a')))));
}

#[test]
fn it_parses_optional_match() {
    // (a|b)*abb
    let nfa = regex2nfa!("(a|b)*abb");
    assert!(matches!(
        nfa,
        Concat(
            Star(Union(Symbol('a'), Symbol('b'))),
            Concat(Symbol('a'), Concat(Symbol('b'), Symbol('b')))
        )
    ));
}
