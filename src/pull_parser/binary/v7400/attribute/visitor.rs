//! Node attribute visitors.

use std::fmt;

use super::super::error::DataError;
use super::super::Result;

/// A trait for attribute visitor types.
// TODO: Implement binary and string attribute visitor.
pub trait VisitAttribute: Sized + fmt::Debug {
    /// Result type on successful read.
    type Output;

    /// Describes the expecting value.
    fn expecting(&self) -> String;

    /// Visit boolean value.
    fn visit_bool(self, _: bool) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "boolean".into()).into())
    }

    /// Visit `i16` value.
    fn visit_i16(self, _: i16) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "i16".into()).into())
    }

    /// Visit `i32` value.
    fn visit_i32(self, _: i32) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "i32".into()).into())
    }

    /// Visit `i64` value.
    fn visit_i64(self, _: i64) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "i64".into()).into())
    }

    /// Visit `f32` value.
    fn visit_f32(self, _: f32) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "f32".into()).into())
    }

    /// Visit `f64` value.
    fn visit_f64(self, _: f64) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "f64".into()).into())
    }

    /// Visit boolean array.
    fn visit_seq_bool(self, _: impl Iterator<Item = Result<bool>>) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "boolean array".into()).into())
    }

    /// Visit `i32` array.
    fn visit_seq_i32(self, _: impl Iterator<Item = Result<i32>>) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "i32 array".into()).into())
    }

    /// Visit `i64` array.
    fn visit_seq_i64(self, _: impl Iterator<Item = Result<i64>>) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "i64 array".into()).into())
    }

    /// Visit `f32` array.
    fn visit_seq_f32(self, _: impl Iterator<Item = Result<f32>>) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "f32 array".into()).into())
    }

    /// Visit `f64` array.
    fn visit_seq_f64(self, _: impl Iterator<Item = Result<f64>>) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "f64 array".into()).into())
    }
}

/// Visitor for primitive types.
///
/// Supported types are: [`bool`], [`i16`] , [`i32`], [`i64`], [`f32`], [`f64`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimitiveVisitor<T>(std::marker::PhantomData<T>);

/// Generates `VisitAttribute` implementations for `PrimitiveVisitor<T>`.
macro_rules! impl_visit_attribute_for_primitives {
    ($ty:ty, $method_name:ident, $expecting_type:expr) => {
        impl VisitAttribute for PrimitiveVisitor<$ty> {
            type Output = $ty;

            fn expecting(&self) -> String {
                $expecting_type.into()
            }

            fn $method_name(self, v: $ty) -> Result<Self::Output> {
                Ok(v)
            }
        }
    };
}

impl_visit_attribute_for_primitives!(bool, visit_bool, "single boolean");
impl_visit_attribute_for_primitives!(i16, visit_i16, "single i16");
impl_visit_attribute_for_primitives!(i32, visit_i32, "single i32");
impl_visit_attribute_for_primitives!(i64, visit_i64, "single i64");
impl_visit_attribute_for_primitives!(f32, visit_f32, "single f32");
impl_visit_attribute_for_primitives!(f64, visit_f64, "single f64");

/// Visitor for array types.
///
/// Supported types are: `Vec<{bool, i32, i64, f32, f64}>`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayVisitor<T>(std::marker::PhantomData<T>);

/// Generates `VisitAttribute` implementations for `PrimitiveVisitor<T>`.
macro_rules! impl_visit_attribute_for_arrays {
    ($ty:ty, $method_name:ident, $expecting_type:expr) => {
        impl VisitAttribute for ArrayVisitor<Vec<$ty>> {
            type Output = Vec<$ty>;

            fn expecting(&self) -> String {
                $expecting_type.into()
            }

            fn $method_name(self, iter: impl Iterator<Item = Result<$ty>>) -> Result<Self::Output> {
                iter.collect::<Result<_>>()
            }
        }
    };
}

impl_visit_attribute_for_arrays!(bool, visit_seq_bool, "boolean array");
impl_visit_attribute_for_arrays!(i32, visit_seq_i32, "i32 array");
impl_visit_attribute_for_arrays!(i64, visit_seq_i64, "i64 array");
impl_visit_attribute_for_arrays!(f32, visit_seq_f32, "f32 array");
impl_visit_attribute_for_arrays!(f64, visit_seq_f64, "f64 array");
