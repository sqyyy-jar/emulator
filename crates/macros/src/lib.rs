use proc_macro::token_stream::IntoIter;
use proc_macro::{Spacing, TokenStream, TokenTree};
use std::iter::Peekable;

#[proc_macro]
pub fn emulator(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter().peekable();
    while tokens.peek().is_some() {
        parse_statement(&mut tokens);
    }
    todo!()
}

fn parse_statement(tokens: &mut Peekable<IntoIter>) {
    let TokenTree::Ident(_name) = tokens.next().unwrap() else {
        panic!("A statement requires a name")
    };
    let Some(TokenTree::Punct(equals)) = tokens.next() else {
        panic!("A statement requires an equals sign")
    };
    if equals.as_char() != '=' || equals.spacing() != Spacing::Alone {
        panic!("A statement requires an equals sign")
    }
}
