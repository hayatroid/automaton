use std::{iter::Peekable, str::Chars};

use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn regex2nfa(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_str = parse_macro_input!(input as LitStr).value();
    let mut source = input_str.chars().peekable();
    parse_union(&mut source).into()
}

fn eat(source: &mut Peekable<Chars>, c: char) {
    assert_eq!(source.next(), Some(c)); // consume with assertion
}

fn parse_union(source: &mut Peekable<Chars>) -> proc_macro2::TokenStream {
    let concat = parse_concat(source);
    if let Some('|') = source.peek() {
        eat(source, '|');
        let union = parse_union(source);
        quote!(::automaton::nfa::union::Union(#concat, #union))
    } else {
        concat
    }
}

fn parse_concat(source: &mut Peekable<Chars>) -> proc_macro2::TokenStream {
    if matches!(source.peek(), None | Some('|') | Some(')')) {
        return quote!(::automaton::nfa::empty::Empty(
            std::marker::PhantomData::<char>
        ));
    }
    let star = parse_star(source);
    if !matches!(source.peek(), None | Some('|') | Some(')') | Some('*')) {
        let concat = parse_concat(source);
        quote!(::automaton::nfa::concat::Concat(#star, #concat))
    } else {
        star
    }
}

fn parse_star(source: &mut Peekable<Chars>) -> proc_macro2::TokenStream {
    let factor = parse_factor(source);
    if let Some('*') = source.peek() {
        eat(source, '*');
        quote!(::automaton::nfa::star::Star(#factor))
    } else {
        factor
    }
}

fn parse_factor(source: &mut Peekable<Chars>) -> proc_macro2::TokenStream {
    match source.next() {
        None | Some('|') | Some(')') | Some('*') => unreachable!(),
        Some('(') => {
            let res = parse_union(source);
            eat(source, ')');
            res
        }
        Some('\\') => match source.next() {
            Some(c) => quote!(::automaton::nfa::symbol::Symbol(#c)),
            _ => unreachable!(),
        },
        Some(c) => quote!(::automaton::nfa::symbol::Symbol(#c)),
    }
}
