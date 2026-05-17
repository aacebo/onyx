use std::collections::BTreeMap;

use crate::Tensor;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TensorMap(BTreeMap<String, Tensor>);

impl TensorMap {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn get(&self, name: &str) -> Option<&Tensor> {
        self.0.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Tensor)> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, name: impl Into<String>, tensor: Tensor) -> &mut Self {
        self.0.insert(name.into(), tensor);
        self
    }
}

impl FromIterator<(String, Tensor)> for TensorMap {
    fn from_iter<I: IntoIterator<Item = (String, Tensor)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<K: Into<String>, const N: usize> From<[(K, Tensor); N]> for TensorMap {
    fn from(entries: [(K, Tensor); N]) -> Self {
        Self(entries.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }
}
