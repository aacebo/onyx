use crate::{Span, error};

/// One tokenized text: parallel arrays aligned by token position.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Encoding {
    pub ids: Vec<u32>,
    pub attention_mask: Vec<u32>,
    pub type_ids: Vec<u32>,
    pub offsets: Vec<Span>,
}

impl Encoding {
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}

/// Converts text into model-ready token sequences.
pub trait Tokenizer: Send + Sync {
    fn encode(&self, text: &str, add_special: bool) -> Result<Encoding, error::ModelError>;

    fn encode_batch(
        &self,
        texts: &[&str],
        add_special: bool,
    ) -> Result<Vec<Encoding>, error::ModelError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Encoding {
        Encoding {
            ids: vec![101, 7592, 102],
            attention_mask: vec![1, 1, 1],
            type_ids: vec![0, 0, 0],
            offsets: vec![
                Span { start: 0, end: 0 },
                Span { start: 0, end: 5 },
                Span { start: 5, end: 5 },
            ],
        }
    }

    #[test]
    fn len_and_is_empty() {
        let enc = sample();
        assert_eq!(enc.len(), 3);
        assert!(!enc.is_empty());
        assert_eq!(enc, enc.clone());

        let empty = Encoding {
            ids: vec![],
            attention_mask: vec![],
            type_ids: vec![],
            offsets: vec![],
        };
        assert_eq!(empty.len(), 0);
        assert!(empty.is_empty());
    }
}
