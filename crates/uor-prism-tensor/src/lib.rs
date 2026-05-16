//! Prism standard-library tensor-compute sub-crate.
//!
//! `prism-tensor` realizes the tensor Layer-3 of the standard library
//! named in [Wiki ADR-031][09-adr-031]: declares `TensorAxis` and
//! `ActivationAxis` through the [`axis!`][09-adr-030] SDK macro and
//! supplies parametric CPU integer-precision reference impls
//! preserving bit-determinism per fixed `(HostTypes, HostBounds,
//! AxisTuple)` selection (per ADR-030's per-axis
//! substitution-determinism note).
//!
//! ## Scope
//!
//! - **`TensorAxis`** — fixed-shape matmul. Parametric:
//!   [`CpuI8MatmulSquare<DIM>`] for `DIM × DIM` `i8` × `i8` → `i16`
//!   matrices with `DIM ≤ MAX_TENSOR_DIM` (16). Aliases:
//!   [`CpuI8Tensor4x4Matmul`], [`CpuI8Tensor8x8Matmul`],
//!   [`CpuI8Tensor16x16Matmul`].
//! - **`ActivationAxis`** — element-wise nonlinearity. Parametric:
//!   [`CpuI8VectorActivation<N>`] for length-`N` `i8` vectors with
//!   `N ≤ MAX_ACTIVATION_LEN` (256). Aliases:
//!   [`CpuI8VectorActivation16`], [`CpuI8VectorActivation32`],
//!   [`CpuI8VectorActivation64`], [`CpuI8VectorActivation128`],
//!   [`CpuI8VectorActivation256`].
//!
//! ## ConstrainedTypeShape declarations
//!
//! Per ADR-031's `Tensor<Element, Shape>` shape commitment:
//!
//! - **[`MatrixShape<ROWS, COLS, ELEM_BYTES>`]** — rank-2 tensor shape.
//! - **[`VectorShape<N, ELEM_BYTES>`]** — rank-1 tensor shape.
//!
//! Higher-rank tensors compose through `partition_product!` per
//! ADR-033/044; the axis layer fixes the atom shape.
//!
//! ## Closure under uor-foundation (ADR-013)
//!
//! Every axis trait declared here has
//! `::uor_foundation::pipeline::AxisExtension` as a supertrait;
//! parametric impls hand-write their `AxisExtension` impl since the
//! `axis!`-emitted companion macro takes `:ident`.
//!
//! ## See also
//!
//! - [Wiki: 09 Architecture Decisions § ADR-030 — `axis!` SDK macro][09-adr-030]
//! - [Wiki: 09 Architecture Decisions § ADR-031 — `prism` is the standard library][09-adr-031]
//!
//! [09-adr-030]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod activation;
pub mod tensor;
pub mod verbs;

pub use activation::{
    ActivationAxis, CpuI8VectorActivation, CpuI8VectorActivation128, CpuI8VectorActivation16,
    CpuI8VectorActivation256, CpuI8VectorActivation32, CpuI8VectorActivation64, MAX_ACTIVATION_LEN,
};
pub use tensor::{
    CpuI8MatmulSquare, CpuI8Tensor16x16Matmul, CpuI8Tensor4x4Matmul, CpuI8Tensor8x8Matmul,
    MatrixShape, TensorAxis, VectorShape, MAX_TENSOR_DIM,
};

/// Wiki ADR-031 standard-library version banner.
pub const STANDARD_LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");
