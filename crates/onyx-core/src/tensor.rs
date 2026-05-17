use ndarray::ArrayD;

use crate::error::{Error, RuntimeError};

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
#[serde(rename_all = "snake_case")]
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

    /// Concrete dimension sizes, erroring if any dim is `Symbolic`/`Unknown`.
    ///
    /// A concrete tensor requires fully-known dims; symbolic shapes only make
    /// sense for declared model signatures ([`IOSpec`](crate::runtime::IOSpec)).
    pub fn to_fixed_dims(&self) -> Result<Vec<usize>, Error> {
        self.0
            .iter()
            .map(|d| match d {
                Dim::Fixed(n) => Ok(*n),
                Dim::Symbolic(s) => Err(Error::Runtime(RuntimeError::ShapeMismatch {
                    expected: "fixed dimension".into(),
                    got: format!("symbolic dimension `{s}`"),
                })),
                Dim::Unknown => Err(Error::Runtime(RuntimeError::ShapeMismatch {
                    expected: "fixed dimension".into(),
                    got: "unknown dimension".into(),
                })),
            })
            .collect()
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

/// Reshape a flat buffer into an `ArrayD`, mapping shape/length errors onto
/// [`RuntimeError::ShapeMismatch`].
fn into_array<T>(shape: impl Into<Shape>, data: Vec<T>) -> Result<ArrayD<T>, Error> {
    let dims = shape.into().to_fixed_dims()?;
    let len = data.len();

    ArrayD::from_shape_vec(dims.clone(), data).map_err(|_| {
        Error::Runtime(RuntimeError::ShapeMismatch {
            expected: format!(
                "{} elements for shape {dims:?}",
                dims.iter().product::<usize>()
            ),
            got: format!("{len} elements"),
        })
    })
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "dtype", rename_all = "snake_case")]
pub enum Tensor {
    I64 { data: ArrayD<i64> },
    I32 { data: ArrayD<i32> },
    F32 { data: ArrayD<f32> },
    F64 { data: ArrayD<f64> },
    String { data: ArrayD<String> },
    Bool { data: ArrayD<bool> },
}

impl Tensor {
    pub fn i64(shape: impl Into<Shape>, data: Vec<i64>) -> Result<Self, Error> {
        Ok(Self::I64 {
            data: into_array(shape, data)?,
        })
    }

    pub fn f32(shape: impl Into<Shape>, data: Vec<f32>) -> Result<Self, Error> {
        Ok(Self::F32 {
            data: into_array(shape, data)?,
        })
    }

    /// The element type of this tensor.
    pub fn dtype(&self) -> DType {
        match self {
            Self::I64 { .. } => DType::I64,
            Self::I32 { .. } => DType::I32,
            Self::F32 { .. } => DType::F32,
            Self::F64 { .. } => DType::F64,
            Self::String { .. } => DType::String,
            Self::Bool { .. } => DType::Bool,
        }
    }

    /// Concrete dims of the backing array as a `Fixed`-dim [`Shape`].
    pub fn shape(&self) -> Shape {
        match self {
            Self::I64 { data } => data.shape().into(),
            Self::I32 { data } => data.shape().into(),
            Self::F32 { data } => data.shape().into(),
            Self::F64 { data } => data.shape().into(),
            Self::String { data } => data.shape().into(),
            Self::Bool { data } => data.shape().into(),
        }
    }
}

impl From<ArrayD<i64>> for Tensor {
    fn from(data: ArrayD<i64>) -> Self {
        Self::I64 { data }
    }
}

impl From<ArrayD<i32>> for Tensor {
    fn from(data: ArrayD<i32>) -> Self {
        Self::I32 { data }
    }
}

impl From<ArrayD<f32>> for Tensor {
    fn from(data: ArrayD<f32>) -> Self {
        Self::F32 { data }
    }
}

impl From<ArrayD<f64>> for Tensor {
    fn from(data: ArrayD<f64>) -> Self {
        Self::F64 { data }
    }
}

impl From<ArrayD<String>> for Tensor {
    fn from(data: ArrayD<String>) -> Self {
        Self::String { data }
    }
}

impl From<ArrayD<bool>> for Tensor {
    fn from(data: ArrayD<bool>) -> Self {
        Self::Bool { data }
    }
}
