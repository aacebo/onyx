//! Tokenization abstraction implemented by peer crates (e.g. a Hugging Face
//! `tokenizers`-backed crate, or a custom WordPiece implementation).
//!
//! A [`Tokenizer`] turns text into the integer token ids and attention mask a
//! [`Session`](crate::runtime::Session) expects. Keeping this as a trait lets
//! tokenization be a swappable peer crate composed with a `Session`, mirroring
//! the [`Runtime`](crate::runtime::Runtime)/`Session` split.
//!
//! Like the other core traits this uses `async fn` in trait (RPITIT) and is
//! not `dyn`-compatible out of the box; prefer static dispatch via generics.

use crate::error::Error;
use crate::tensor::Tensor;

/// The result of tokenizing a single input text.
///
/// `ids` and `attention_mask` have the same length. `attention_mask` is `1`
/// for real tokens and `0` for padding, matching the transformer convention.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Encoding {
    pub ids: Vec<i64>,
    pub attention_mask: Vec<i64>,
}

impl Encoding {
    /// Number of tokens in this encoding.
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// The token ids as a `[len]`-shaped `i64` tensor.
    pub fn ids_tensor(&self) -> Tensor {
        Tensor::i64([self.ids.len()], self.ids.clone())
    }

    /// The attention mask as a `[len]`-shaped `i64` tensor.
    pub fn attention_mask_tensor(&self) -> Tensor {
        Tensor::i64([self.attention_mask.len()], self.attention_mask.clone())
    }
}

/// Encodes text into token ids / attention masks for a model.
pub trait Tokenizer: Send + Sync {
    /// Encode a single text.
    async fn encode(&self, text: &str) -> Result<Encoding, Error>;

    /// Encode a batch of texts. Implementations may override this for
    /// batched/padded tokenization; the default encodes sequentially.
    async fn encode_batch(&self, texts: &[&str]) -> Result<Vec<Encoding>, Error> {
        let mut out = Vec::with_capacity(texts.len());
        for text in texts {
            out.push(self.encode(text).await?);
        }
        Ok(out)
    }
}
