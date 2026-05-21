use crate::Span;

pub trait Token {
    fn id(&self) -> u32;
    fn text(&self) -> &str;
    fn span(&self) -> Span;
}

pub trait Tokenizer {
    type Token: Token;
    type Error: std::error::Error;

    fn tokenize(&self, input: &str) -> Result<Vec<Self::Token>, Self::Error>;
    fn token_to_id(&self, token: &str) -> Option<u32>;
    fn id_to_token(&self, id: u32) -> Option<String>;
}
