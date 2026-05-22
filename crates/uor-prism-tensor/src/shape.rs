//! Higher-rank tensor `ConstrainedTypeShape` carriers
//! ([`Tensor3Shape`], [`Tensor4Shape`]).
//!
//! Per [Wiki ADR-031][09]'s `Tensor<Element, Shape>` shape commitment,
//! the tensor sub-crate declares typed shape carriers per common rank:
//!
//! - rank-1 → [`VectorShape`][crate::tensor::VectorShape]
//! - rank-2 → [`MatrixShape`][crate::tensor::MatrixShape]
//! - rank-3 → [`Tensor3Shape`] *(this module)*
//! - rank-4 → [`Tensor4Shape`] *(this module)*
//!
//! Higher ranks compose through `partition_product!` per ADR-033/044;
//! the carriers in this module cover the common GGUF / ONNX tensor
//! ranks (embeddings, attention heads, batched attention) directly.
//!
//! Per [ADR-017][09]'s closure rule each carrier shares the generic
//! `https://uor.foundation/type/ConstrainedType` IRI and content-
//! addresses through `(SITE_COUNT, CONSTRAINTS)`; the Rust-type
//! distinction is the application-level ergonomics surface for
//! variable-rank tensor shapes.
//!
//! [09]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(missing_docs)]

use uor_foundation::enforcement::GroundedShape;
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef, IntoBindingValue, TermValue};

/// Parametric `ConstrainedTypeShape` for a row-major rank-3 tensor of
/// shape `D0 × D1 × D2` carrying `ELEM_BYTES`-byte elements.
///
/// Per ADR-031's `Tensor<Element, Shape>` shape commitment for rank-3.
/// Common GGUF / ONNX usage: per-head attention key / value tensors
/// (`batch × heads × dim`), 3D image volumes, sequence-of-tokens
/// embeddings.
#[derive(Debug, Clone, Copy)]
pub struct Tensor3Shape<const D0: usize, const D1: usize, const D2: usize, const ELEM_BYTES: usize>;

impl<const D0: usize, const D1: usize, const D2: usize, const ELEM_BYTES: usize> Default
    for Tensor3Shape<D0, D1, D2, ELEM_BYTES>
{
    fn default() -> Self {
        Self
    }
}

impl<const D0: usize, const D1: usize, const D2: usize, const ELEM_BYTES: usize>
    ConstrainedTypeShape for Tensor3Shape<D0, D1, D2, ELEM_BYTES>
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = D0 * D1 * D2 * ELEM_BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow((D0 * D1 * D2 * ELEM_BYTES) as u32);
}

impl<const D0: usize, const D1: usize, const D2: usize, const ELEM_BYTES: usize>
    uor_foundation::pipeline::__sdk_seal::Sealed for Tensor3Shape<D0, D1, D2, ELEM_BYTES>
{
}
impl<const D0: usize, const D1: usize, const D2: usize, const ELEM_BYTES: usize> GroundedShape
    for Tensor3Shape<D0, D1, D2, ELEM_BYTES>
{
}
impl<'a, const D0: usize, const D1: usize, const D2: usize, const ELEM_BYTES: usize>
    IntoBindingValue<'a> for Tensor3Shape<D0, D1, D2, ELEM_BYTES>
{
    fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
        TermValue::empty()
    }
}

/// Parametric `ConstrainedTypeShape` for a row-major rank-4 tensor of
/// shape `D0 × D1 × D2 × D3` carrying `ELEM_BYTES`-byte elements.
///
/// Per ADR-031's `Tensor<Element, Shape>` shape commitment for rank-4.
/// Common GGUF / ONNX usage: batched multi-head attention
/// (`batch × heads × seq × dim`), 4D conv weight tensors
/// (`out_channels × in_channels × kernel_h × kernel_w`).
#[derive(Debug, Clone, Copy)]
pub struct Tensor4Shape<
    const D0: usize,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const ELEM_BYTES: usize,
>;

impl<
        const D0: usize,
        const D1: usize,
        const D2: usize,
        const D3: usize,
        const ELEM_BYTES: usize,
    > Default for Tensor4Shape<D0, D1, D2, D3, ELEM_BYTES>
{
    fn default() -> Self {
        Self
    }
}

impl<
        const D0: usize,
        const D1: usize,
        const D2: usize,
        const D3: usize,
        const ELEM_BYTES: usize,
    > ConstrainedTypeShape for Tensor4Shape<D0, D1, D2, D3, ELEM_BYTES>
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = D0 * D1 * D2 * D3 * ELEM_BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow((D0 * D1 * D2 * D3 * ELEM_BYTES) as u32);
}

impl<
        const D0: usize,
        const D1: usize,
        const D2: usize,
        const D3: usize,
        const ELEM_BYTES: usize,
    > uor_foundation::pipeline::__sdk_seal::Sealed for Tensor4Shape<D0, D1, D2, D3, ELEM_BYTES>
{
}
impl<
        const D0: usize,
        const D1: usize,
        const D2: usize,
        const D3: usize,
        const ELEM_BYTES: usize,
    > GroundedShape for Tensor4Shape<D0, D1, D2, D3, ELEM_BYTES>
{
}
impl<
        'a,
        const D0: usize,
        const D1: usize,
        const D2: usize,
        const D3: usize,
        const ELEM_BYTES: usize,
    > IntoBindingValue<'a> for Tensor4Shape<D0, D1, D2, D3, ELEM_BYTES>
{
    fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
        TermValue::empty()
    }
}
