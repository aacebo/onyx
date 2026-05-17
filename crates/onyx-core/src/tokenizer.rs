use async_trait::async_trait;

use crate::{Error, Tensor};

/// The result of tokenizing a single input text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Encoding {
    pub ids: ndarray::ArrayD<i64>,
    pub attention_mask: ndarray::ArrayD<i64>,
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
        // Shape `[len]` always matches the buffer length, so this cannot fail.
        Tensor::i64(self.ids.clone())
    }

    /// The attention mask as a `[len]`-shaped `i64` tensor.
    pub fn attention_mask_tensor(&self) -> Tensor {
        Tensor::i64(self.attention_mask.clone())
    }
}

/// Encodes text into token ids / attention masks for a model.
#[async_trait]
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
