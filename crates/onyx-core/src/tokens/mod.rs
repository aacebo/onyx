mod config;

pub use config::*;

use crate::Span;

pub trait Token {
    fn id(&self) -> u32;
    fn text(&self) -> &str;
    fn span(&self) -> Span;
}

pub trait Tokenizer {
    type Token: Token;

    fn tokenize(&self, input: &str) -> crate::error::Result<Vec<Self::Token>>;
    fn token_to_id(&self, token: &str) -> Option<u32>;
    fn id_to_token(&self, id: u32) -> Option<String>;
}
