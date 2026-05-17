use std::collections::BTreeMap;

use crate::tensor::{DType, Shape};

/// Declared I/O signature of a session: a named tensor slot with its dtype
/// and (possibly symbolic) shape.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TensorSpec {
    pub dtype: DType,
    pub shape: Shape,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TensorSchema(BTreeMap<String, TensorSpec>);

impl TensorSchema {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn get(&self, name: &str) -> Option<&TensorSpec> {
        self.0.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &TensorSpec)> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, name: impl Into<String>, tensor: TensorSpec) -> &mut Self {
        self.0.insert(name.into(), tensor);
        self
    }
}

impl FromIterator<(String, TensorSpec)> for TensorSchema {
    fn from_iter<I: IntoIterator<Item = (String, TensorSpec)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<K: Into<String>, const N: usize> From<[(K, TensorSpec); N]> for TensorSchema {
    fn from(entries: [(K, TensorSpec); N]) -> Self {
        Self(entries.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }
}
