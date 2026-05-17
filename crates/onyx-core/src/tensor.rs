#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape(Vec<Dim>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dim {
    Fixed(usize),
    Symbolic(String),
    Unknown,
}

impl Shape {
    pub fn dims(&self) -> &[Dim] {
        &self.0
    }

    pub fn rank(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<Dim>> for Shape {
    fn from(dims: Vec<Dim>) -> Self {
        Self(dims)
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

#[derive(Debug, Clone)]
pub enum TensorData {
    I64(Vec<i64>),
    I32(Vec<i32>),
    F32(Vec<f32>),
    F64(Vec<f64>),
    String(Vec<String>),
    Bool(Vec<bool>),
}

impl From<Vec<i64>> for TensorData {
    fn from(v: Vec<i64>) -> Self {
        Self::I64(v)
    }
}

impl From<Vec<i32>> for TensorData {
    fn from(v: Vec<i32>) -> Self {
        Self::I32(v)
    }
}

impl From<Vec<f32>> for TensorData {
    fn from(v: Vec<f32>) -> Self {
        Self::F32(v)
    }
}

impl From<Vec<f64>> for TensorData {
    fn from(v: Vec<f64>) -> Self {
        Self::F64(v)
    }
}

impl From<Vec<String>> for TensorData {
    fn from(v: Vec<String>) -> Self {
        Self::String(v)
    }
}

impl From<Vec<bool>> for TensorData {
    fn from(v: Vec<bool>) -> Self {
        Self::Bool(v)
    }
}

#[derive(Debug, Clone)]
pub struct Tensor {
    pub dtype: DType,
    pub shape: Shape,
    pub data: TensorData,
}

impl Tensor {
    pub fn i64(shape: impl Into<Shape>, data: Vec<i64>) -> Self {
        Self {
            dtype: DType::I64,
            shape: shape.into(),
            data: TensorData::I64(data),
        }
    }

    pub fn f32(shape: impl Into<Shape>, data: Vec<f32>) -> Self {
        Self {
            dtype: DType::F32,
            shape: shape.into(),
            data: TensorData::F32(data),
        }
    }
}
