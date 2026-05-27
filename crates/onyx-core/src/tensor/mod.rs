mod dim;
mod dtype;
mod layout;
mod shape;

pub use dim::*;
pub use dtype::*;
pub use layout::*;
pub use shape::*;

pub trait Tensor {
    fn dtype(&self) -> DType;
    fn shape(&self) -> Shape;
    fn layout(&self) -> Layout;
}
