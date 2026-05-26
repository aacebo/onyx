mod architecture;
pub mod bert;
mod config;
mod types;

pub use architecture::*;
pub use config::*;
pub use types::*;

use crate::error::{InferenceError, ParseError};

pub trait ModelResolver: Send + Sync {
    fn resolve(&self, id: &ModelId) -> crate::error::Result<Model>;
}

#[derive(Clone, PartialEq, Eq)]
pub struct ModelId {
    group: Option<Box<str>>,
    name: Box<str>,
}

impl ModelId {
    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl std::str::FromStr for ModelId {
    type Err = crate::OnyxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("/") {
            let (group, name) = match s.split_once("/") {
                None => return Err(ParseError::InvalidModelId(s.to_string()).into()),
                Some(v) => v,
            };

            if group.is_empty() || name.is_empty() {
                return Err(ParseError::InvalidModelId(s.to_string()).into());
            }

            return Ok(Self {
                group: Some(group.into()),
                name: name.into(),
            });
        }

        if s.is_empty() {
            return Err(ParseError::InvalidModelId(s.to_string()).into());
        }

        Ok(Self {
            group: None,
            name: s.into(),
        })
    }
}

impl std::fmt::Debug for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(group) = &self.group {
            return write!(f, "{}/{}", group, &self.name);
        }

        write!(f, "{}", &self.name)
    }
}

impl serde::Serialize for ModelId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ModelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::str::FromStr;

        let value = String::deserialize(deserializer)?;
        Self::from_str(&value).map_err(serde::de::Error::custom)
    }
}

pub trait Forward: Send + Sync {
    type Input;
    type Output;

    fn forward<'a>(&'a self, input: Self::Input) -> crate::BoxFuture<'a, crate::error::Result<Self::Output>>;
}

pub enum ModelInput {
    Bert(bert::BertInput),
}

pub enum ModelOutput {
    Bert(bert::BertOutput),
}

impl From<bert::BertInput> for ModelInput {
    fn from(value: bert::BertInput) -> Self {
        Self::Bert(value)
    }
}

impl From<bert::BertOutput> for ModelOutput {
    fn from(value: bert::BertOutput) -> Self {
        Self::Bert(value)
    }
}

pub enum Model {
    Bert(std::sync::Arc<dyn bert::BertModel>),
    Custom(std::sync::Arc<dyn Forward<Input = ModelInput, Output = ModelOutput>>),
}

impl Model {
    pub fn custom<T>(value: T) -> Self
    where
        T: Forward<Input = ModelInput, Output = ModelOutput> + 'static,
    {
        Self::Custom(std::sync::Arc::new(value) as std::sync::Arc<dyn Forward<Input = ModelInput, Output = ModelOutput>>)
    }
}

impl<T: bert::BertModel + 'static> From<T> for Model {
    fn from(value: T) -> Self {
        Self::Bert(std::sync::Arc::new(value))
    }
}

impl Forward for Model {
    type Input = ModelInput;
    type Output = ModelOutput;

    fn forward<'a>(&'a self, input: Self::Input) -> crate::BoxFuture<'a, crate::error::Result<Self::Output>> {
        Box::pin(async move {
            match (self, input) {
                (Model::Bert(m), ModelInput::Bert(i)) => m.infer(i).await.map(ModelOutput::Bert),
                (Model::Custom(m), input) => m.forward(input).await,
                // TODO: add mismatch test when second model family lands
                #[allow(unreachable_patterns)]
                _ => Err(InferenceError::InvalidInput("Model/ModelInput variant mismatch".into()).into()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn parse_valid() {
        let id = ModelId::from_str("facebook/bart-large").expect("should parse");
        assert_eq!(id.group(), Some("facebook"));
        assert_eq!(id.name(), "bart-large");
        assert_eq!(id.to_string(), "facebook/bart-large");
    }

    #[test]
    fn parse_nested_slash() {
        let id = ModelId::from_str("facebook/bart/large").expect("should parse");
        assert_eq!(id.group(), Some("facebook"));
        assert_eq!(id.name(), "bart/large");
        assert_eq!(id.to_string(), "facebook/bart/large");
    }

    #[test]
    fn parse_empty_segments() {
        use crate::OnyxError;
        for input in ["/name", "group/", "/", ""] {
            let err = ModelId::from_str(input).expect_err("should fail");
            assert!(
                matches!(err, OnyxError::Parse(ParseError::InvalidModelId(_))),
                "expected Parse error for {input:?}",
            );
        }
    }

    #[test]
    fn display_roundtrip() {
        let id = ModelId::from_str("facebook/bart-large").unwrap();
        assert_eq!(format!("{id}"), "facebook/bart-large");
        assert_eq!(format!("{id:?}"), "facebook/bart-large");
    }

    #[test]
    fn serde_roundtrip() {
        let id = ModelId::from_str("facebook/bart-large").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"facebook/bart-large\"");

        let back: ModelId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);

        assert_eq!(serde_json::from_str::<ModelId>("\"nogroup\"").unwrap().to_string(), "nogroup");
    }

    fn dummy_bert_input() -> bert::BertInput {
        let ids: ndarray::ArrayD<i64> = ndarray::ArrayD::zeros(ndarray::IxDyn(&[1, 4]));
        bert::BertInput {
            input_ids: ids.into(),
            attention_mask: None,
            token_type_ids: None,
            position_ids: None,
        }
    }

    fn dummy_bert_output() -> bert::BertOutput {
        let h: ndarray::ArrayD<f32> = ndarray::ArrayD::zeros(ndarray::IxDyn(&[1, 4, 8]));
        bert::BertOutput {
            last_hidden_state: h.into(),
            pooled_output: None,
            hidden_states: None,
            attentions: None,
        }
    }

    struct Echo;

    impl Forward for Echo {
        type Input = ModelInput;
        type Output = ModelOutput;

        fn forward<'a>(&'a self, input: Self::Input) -> crate::BoxFuture<'a, crate::error::Result<Self::Output>> {
            Box::pin(async move {
                match input {
                    ModelInput::Bert(_) => Ok(ModelOutput::Bert(dummy_bert_output())),
                }
            })
        }
    }

    fn block_on<F: std::future::Future>(mut fut: F) -> F::Output {
        use std::sync::Arc;
        use std::task::{Context, Poll, Wake, Waker};

        struct NoopWake;
        impl Wake for NoopWake {
            fn wake(self: Arc<Self>) {}
        }

        let waker: Waker = Arc::new(NoopWake).into();
        let mut cx = Context::from_waker(&waker);
        // Safety: `fut` is owned by this stack frame and never moved after pinning.
        let mut pinned = unsafe { std::pin::Pin::new_unchecked(&mut fut) };
        loop {
            if let Poll::Ready(v) = pinned.as_mut().poll(&mut cx) {
                return v;
            }
        }
    }

    #[test]
    fn custom_variant_dispatches_to_dyn_forward() {
        let m = Model::custom(Echo);
        let out = block_on(m.forward(ModelInput::Bert(dummy_bert_input()))).unwrap();
        assert!(matches!(out, ModelOutput::Bert(_)));
    }

    fn _accepts_forward<F>(_: F)
    where
        F: Forward<Input = ModelInput, Output = ModelOutput>,
    {
    }

    #[test]
    fn model_implements_forward_at_type_level() {
        // Compile-only: passing a Model into a function generic over `Forward`
        // must type-check. We never call it, just construct one and pass it.
        let m = Model::custom(Echo);
        _accepts_forward(m);
    }
}
