use crate::{Annotation, Error, tensor};

/// A composed inference flow (tokenize -> session -> decode) that satisfies
/// exactly one capability. Concrete pipelines additionally implement the
/// matching capability trait (Embedder / Classifier / TokenClassifier).
pub trait Pipeline: Send + Sync {}

/// Produces dense embeddings for input texts.
pub trait Embedder: Pipeline {
    /// Embed each input text, returning one vector per input.
    fn embed(&self, texts: &[&str]) -> impl Future<Output = Result<Vec<tensor::FTensor>, Error>>;
}

/// Assigns one or more labeled scores to whole input sequences.
pub trait Classifier: Pipeline {
    /// Classify each input text, returning the annotations per input.
    fn classify(&self, texts: &[&str])
    -> impl Future<Output = Result<Vec<Vec<Annotation>>, Error>>;
}

/// Assigns labeled scores to spans/tokens within each input sequence.
pub trait TokenClassifier: Pipeline {
    /// Classify the tokens of each input text, returning span annotations
    /// per input.
    fn classify_tokens(
        &self,
        texts: &[&str],
    ) -> impl Future<Output = Result<Vec<Vec<Annotation>>, Error>>;
}
