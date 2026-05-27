use super::Dim;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Shape(Vec<Dim>);

impl Shape {
    pub fn new(dims: impl Into<Vec<Dim>>) -> Self {
        Self(dims.into())
    }

    pub fn from_known(dims: impl Into<Vec<usize>>) -> Self {
        Self(dims.into().into_iter().map(Dim::Known).collect())
    }

    pub fn scalar() -> Self {
        Self(Vec::new())
    }

    pub fn dims(&self) -> &[Dim] {
        &self.0
    }

    pub fn rank(&self) -> usize {
        self.0.len()
    }

    pub fn is_scalar(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_fully_known(&self) -> bool {
        self.0.iter().all(Dim::is_known)
    }

    pub fn known_dims(&self) -> Option<Vec<usize>> {
        self.0.iter().map(Dim::as_usize).collect()
    }

    pub fn element_count(&self) -> Option<usize> {
        self.0
            .iter()
            .map(Dim::as_usize)
            .try_fold(1usize, |acc, dim| dim.and_then(|d| acc.checked_mul(d)))
    }

    pub fn get(&self, index: usize) -> Option<&Dim> {
        self.0.get(index)
    }
}

impl<const N: usize> From<[usize; N]> for Shape {
    fn from(value: [usize; N]) -> Self {
        Self::from_known(value.to_vec())
    }
}

impl From<Vec<usize>> for Shape {
    fn from(value: Vec<usize>) -> Self {
        Self::from_known(value)
    }
}

impl std::ops::Index<usize> for Shape {
    type Output = Dim;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
