mod map;
mod schema;

pub use map::*;
pub use schema::*;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F16,
    F32,
    F64,
    String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum Dim {
    Fixed(usize),
    Symbolic(String),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Shape(Vec<Dim>);

impl Shape {
    pub fn dims(&self) -> &[Dim] {
        &self.0
    }

    pub fn rank(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<usize>> for Shape {
    fn from(dims: Vec<usize>) -> Self {
        Self(dims.into_iter().map(Dim::Fixed).collect())
    }
}

impl From<&[usize]> for Shape {
    fn from(dims: &[usize]) -> Self {
        Self(dims.iter().copied().map(Dim::Fixed).collect())
    }
}

impl<const N: usize> From<[usize; N]> for Shape {
    fn from(dims: [usize; N]) -> Self {
        Self(dims.into_iter().map(Dim::Fixed).collect())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "dtype", rename_all = "snake_case")]
pub enum Tensor {
    I64(ndarray::ArrayD<i64>),
    I32(ndarray::ArrayD<i32>),
    F32(ndarray::ArrayD<f32>),
    F64(ndarray::ArrayD<f64>),
    String(ndarray::ArrayD<String>),
    Bool(ndarray::ArrayD<bool>),
}

impl Tensor {
    pub fn i32(data: ndarray::ArrayD<i32>) -> Self {
        Self::I32(data)
    }

    pub fn i64(data: ndarray::ArrayD<i64>) -> Self {
        Self::I64(data)
    }

    pub fn f32(data: ndarray::ArrayD<f32>) -> Self {
        Self::F32(data)
    }

    pub fn f64(data: ndarray::ArrayD<f64>) -> Self {
        Self::F64(data)
    }

    pub fn string(data: ndarray::ArrayD<String>) -> Self {
        Self::String(data)
    }

    pub fn bool(data: ndarray::ArrayD<bool>) -> Self {
        Self::Bool(data)
    }

    /// The element type of this tensor.
    pub fn dtype(&self) -> DType {
        match self {
            Self::I64(_) => DType::I64,
            Self::I32(_) => DType::I32,
            Self::F32(_) => DType::F32,
            Self::F64(_) => DType::F64,
            Self::String(_) => DType::String,
            Self::Bool(_) => DType::Bool,
        }
    }

    /// Concrete dims of the backing array as a `Fixed`-dim [`Shape`].
    pub fn shape(&self) -> Shape {
        match self {
            Self::I64(data) => data.shape().into(),
            Self::I32(data) => data.shape().into(),
            Self::F32(data) => data.shape().into(),
            Self::F64(data) => data.shape().into(),
            Self::String(data) => data.shape().into(),
            Self::Bool(data) => data.shape().into(),
        }
    }
}

impl From<ndarray::ArrayD<i64>> for Tensor {
    fn from(data: ndarray::ArrayD<i64>) -> Self {
        Self::I64(data)
    }
}

impl From<ndarray::ArrayD<i32>> for Tensor {
    fn from(data: ndarray::ArrayD<i32>) -> Self {
        Self::I32(data)
    }
}

impl From<ndarray::ArrayD<f32>> for Tensor {
    fn from(data: ndarray::ArrayD<f32>) -> Self {
        Self::F32(data)
    }
}

impl From<ndarray::ArrayD<f64>> for Tensor {
    fn from(data: ndarray::ArrayD<f64>) -> Self {
        Self::F64(data)
    }
}

impl From<ndarray::ArrayD<String>> for Tensor {
    fn from(data: ndarray::ArrayD<String>) -> Self {
        Self::String(data)
    }
}

impl From<ndarray::ArrayD<bool>> for Tensor {
    fn from(data: ndarray::ArrayD<bool>) -> Self {
        Self::Bool(data)
    }
}
