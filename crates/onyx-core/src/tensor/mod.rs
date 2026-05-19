mod map;
mod schema;

pub use map::*;
pub use schema::*;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
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
#[serde(tag = "dtype", content = "data", rename_all = "snake_case")]
pub enum Tensor {
    Number(NTensor),
    String(ndarray::ArrayD<String>),
    Bool(ndarray::ArrayD<bool>),
}

impl Tensor {
    /// The element type of this tensor.
    pub fn dtype(&self) -> DType {
        match self {
            Self::Number(v) => v.dtype(),
            Self::String(_) => DType::String,
            Self::Bool(_) => DType::Bool,
        }
    }

    /// Concrete dims of the backing array as a `Fixed`-dim [`Shape`].
    pub fn shape(&self) -> Shape {
        match self {
            Self::Number(v) => v.shape(),
            Self::String(data) => data.shape().into(),
            Self::Bool(data) => data.shape().into(),
        }
    }
}

impl From<NTensor> for Tensor {
    fn from(value: NTensor) -> Self {
        Self::Number(value)
    }
}

impl From<UTensor> for Tensor {
    fn from(value: UTensor) -> Self {
        Self::Number(value.into())
    }
}

impl From<ITensor> for Tensor {
    fn from(value: ITensor) -> Self {
        Self::Number(value.into())
    }
}

impl From<FTensor> for Tensor {
    fn from(value: FTensor) -> Self {
        Self::Number(value.into())
    }
}

impl From<ndarray::ArrayD<i8>> for Tensor {
    fn from(value: ndarray::ArrayD<i8>) -> Self {
        Self::Number(ITensor::from(value).into())
    }
}

impl From<ndarray::ArrayD<i16>> for Tensor {
    fn from(value: ndarray::ArrayD<i16>) -> Self {
        Self::Number(ITensor::from(value).into())
    }
}

impl From<ndarray::ArrayD<i32>> for Tensor {
    fn from(value: ndarray::ArrayD<i32>) -> Self {
        Self::Number(ITensor::from(value).into())
    }
}

impl From<ndarray::ArrayD<i64>> for Tensor {
    fn from(value: ndarray::ArrayD<i64>) -> Self {
        Self::Number(ITensor::from(value).into())
    }
}

impl From<ndarray::ArrayD<u8>> for Tensor {
    fn from(value: ndarray::ArrayD<u8>) -> Self {
        Self::Number(UTensor::from(value).into())
    }
}

impl From<ndarray::ArrayD<u16>> for Tensor {
    fn from(value: ndarray::ArrayD<u16>) -> Self {
        Self::Number(UTensor::from(value).into())
    }
}

impl From<ndarray::ArrayD<u32>> for Tensor {
    fn from(value: ndarray::ArrayD<u32>) -> Self {
        Self::Number(UTensor::from(value).into())
    }
}

impl From<ndarray::ArrayD<u64>> for Tensor {
    fn from(value: ndarray::ArrayD<u64>) -> Self {
        Self::Number(UTensor::from(value).into())
    }
}

impl From<ndarray::ArrayD<f32>> for Tensor {
    fn from(value: ndarray::ArrayD<f32>) -> Self {
        Self::Number(FTensor::from(value).into())
    }
}

impl From<ndarray::ArrayD<f64>> for Tensor {
    fn from(value: ndarray::ArrayD<f64>) -> Self {
        Self::Number(FTensor::from(value).into())
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "dtype", content = "data", rename_all = "snake_case")]
pub enum NTensor {
    Signed(ITensor),
    Unsigned(UTensor),
    Float(FTensor),
}

impl NTensor {
    /// The element type of this tensor.
    pub fn dtype(&self) -> DType {
        match self {
            Self::Signed(v) => v.dtype(),
            Self::Unsigned(v) => v.dtype(),
            Self::Float(v) => v.dtype(),
        }
    }

    /// Concrete dims of the backing array as a `Fixed`-dim [`Shape`].
    pub fn shape(&self) -> Shape {
        match self {
            Self::Signed(v) => v.shape(),
            Self::Unsigned(v) => v.shape(),
            Self::Float(v) => v.shape(),
        }
    }
}

impl From<ITensor> for NTensor {
    fn from(value: ITensor) -> Self {
        Self::Signed(value)
    }
}

impl From<UTensor> for NTensor {
    fn from(value: UTensor) -> Self {
        Self::Unsigned(value)
    }
}

impl From<FTensor> for NTensor {
    fn from(value: FTensor) -> Self {
        Self::Float(value)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "dtype", content = "data", rename_all = "snake_case")]
pub enum UTensor {
    U8(ndarray::ArrayD<u8>),
    U16(ndarray::ArrayD<u16>),
    U32(ndarray::ArrayD<u32>),
    U64(ndarray::ArrayD<u64>),
}

impl UTensor {
    /// The element type of this tensor.
    pub fn dtype(&self) -> DType {
        match self {
            Self::U8(_) => DType::U8,
            Self::U16(_) => DType::U16,
            Self::U32(_) => DType::U32,
            Self::U64(_) => DType::U64,
        }
    }

    /// Concrete dims of the backing array as a `Fixed`-dim [`Shape`].
    pub fn shape(&self) -> Shape {
        match self {
            Self::U8(data) => data.shape().into(),
            Self::U16(data) => data.shape().into(),
            Self::U32(data) => data.shape().into(),
            Self::U64(data) => data.shape().into(),
        }
    }
}

impl From<ndarray::ArrayD<u8>> for UTensor {
    fn from(value: ndarray::ArrayD<u8>) -> Self {
        Self::U8(value)
    }
}

impl From<ndarray::ArrayD<u16>> for UTensor {
    fn from(value: ndarray::ArrayD<u16>) -> Self {
        Self::U16(value)
    }
}

impl From<ndarray::ArrayD<u32>> for UTensor {
    fn from(value: ndarray::ArrayD<u32>) -> Self {
        Self::U32(value)
    }
}

impl From<ndarray::ArrayD<u64>> for UTensor {
    fn from(value: ndarray::ArrayD<u64>) -> Self {
        Self::U64(value)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "dtype", content = "data", rename_all = "snake_case")]
pub enum ITensor {
    I8(ndarray::ArrayD<i8>),
    I16(ndarray::ArrayD<i16>),
    I32(ndarray::ArrayD<i32>),
    I64(ndarray::ArrayD<i64>),
}

impl ITensor {
    /// The element type of this tensor.
    pub fn dtype(&self) -> DType {
        match self {
            Self::I8(_) => DType::I8,
            Self::I16(_) => DType::I16,
            Self::I32(_) => DType::I32,
            Self::I64(_) => DType::I64,
        }
    }

    /// Concrete dims of the backing array as a `Fixed`-dim [`Shape`].
    pub fn shape(&self) -> Shape {
        match self {
            Self::I8(data) => data.shape().into(),
            Self::I16(data) => data.shape().into(),
            Self::I32(data) => data.shape().into(),
            Self::I64(data) => data.shape().into(),
        }
    }
}

impl From<ndarray::ArrayD<i8>> for ITensor {
    fn from(value: ndarray::ArrayD<i8>) -> Self {
        Self::I8(value)
    }
}

impl From<ndarray::ArrayD<i16>> for ITensor {
    fn from(value: ndarray::ArrayD<i16>) -> Self {
        Self::I16(value)
    }
}

impl From<ndarray::ArrayD<i32>> for ITensor {
    fn from(value: ndarray::ArrayD<i32>) -> Self {
        Self::I32(value)
    }
}

impl From<ndarray::ArrayD<i64>> for ITensor {
    fn from(value: ndarray::ArrayD<i64>) -> Self {
        Self::I64(value)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "dtype", content = "data", rename_all = "snake_case")]
pub enum FTensor {
    F32(ndarray::ArrayD<f32>),
    F64(ndarray::ArrayD<f64>),
}

impl FTensor {
    /// The element type of this tensor.
    pub fn dtype(&self) -> DType {
        match self {
            Self::F32(_) => DType::F32,
            Self::F64(_) => DType::F64,
        }
    }

    /// Concrete dims of the backing array as a `Fixed`-dim [`Shape`].
    pub fn shape(&self) -> Shape {
        match self {
            Self::F32(data) => data.shape().into(),
            Self::F64(data) => data.shape().into(),
        }
    }
}

impl From<ndarray::ArrayD<f32>> for FTensor {
    fn from(value: ndarray::ArrayD<f32>) -> Self {
        Self::F32(value)
    }
}

impl From<ndarray::ArrayD<f64>> for FTensor {
    fn from(value: ndarray::ArrayD<f64>) -> Self {
        Self::F64(value)
    }
}
