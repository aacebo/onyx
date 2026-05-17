//! High-level task traits layered on top of [`Session`](crate::runtime::Session).
//!
//! Each trait maps 1:1 to a [`ModelFeature`](crate::model::ModelFeature)
//! variant. These are *abstractions only* — peer backend crates implement
//! them in terms of a concrete `Session`. The classification tasks reuse the
//! crate-level [`Annotation`](crate::Annotation) type as their output.
//!
//! Like [`Runtime`](crate::runtime::Runtime)/[`Session`](crate::runtime::Session),
//! these traits use `async fn` in trait (RPITIT) and are not `dyn`-compatible
//! out of the box; prefer static dispatch via generics.

use crate::Annotation;
use crate::error::Error;

/// Produces dense embeddings for input texts.
///
/// Corresponds to [`ModelFeature::Embeddings`](crate::model::ModelFeature::Embeddings).
pub trait Embedder {
    /// Embed each input text, returning one vector per input.
    async fn embed(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, Error>;
}

/// Assigns one or more labeled scores to whole input sequences.
///
/// Corresponds to
/// [`ModelFeature::SequenceClassification`](crate::model::ModelFeature::SequenceClassification).
pub trait Classifier {
    /// Classify each input text, returning the annotations per input.
    async fn classify(&self, texts: &[&str]) -> Result<Vec<Vec<Annotation>>, Error>;
}

/// Assigns labeled scores to spans/tokens within each input sequence.
///
/// Corresponds to
/// [`ModelFeature::TokenClassification`](crate::model::ModelFeature::TokenClassification).
pub trait TokenClassifier {
    /// Classify the tokens of each input text, returning span annotations
    /// per input.
    async fn classify_tokens(&self, texts: &[&str]) -> Result<Vec<Vec<Annotation>>, Error>;
}
