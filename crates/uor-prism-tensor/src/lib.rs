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
//! - **`TensorAxis`** — fixed-shape matmul. Parametric reference:
//!   [`CpuI8MatmulSquare<DIM>`] for `DIM × DIM` `i8` × `i8` → `i16`
//!   matrices. `DIM` is unconstrained at the axis level: per
//!   [ADR-060][09-adr-060] there is no `AXIS_OUTPUT_BYTES_MAX` cap —
//!   axis-kernel output flows through the source-polymorphic
//!   `TermValue` carrier (`Inline`/`Borrowed`/`Stream`), whose inline
//!   width derives from the application's `HostBounds` structural-count
//!   primitives via foundation `const fn`s. The axis impl performs only
//!   a `DIM == 0` structural well-formedness check.
//! - **`ActivationAxis`** — element-wise nonlinearity. Parametric
//!   reference: [`CpuI8VectorActivation<N>`] for length-`N` `i8`
//!   vectors. `N` is likewise unconstrained at the axis level per
//!   ADR-060; the impl performs only an `N == 0` structural check.
//! - **[`dtype`]** — GGML / GGUF / ONNX tensor element-type alphabet
//!   per [ADR-057][09-adr-057]: 43 sealed [`dtype::Dtype`] impls
//!   (continuous floats, ONNX FLOAT8 / complex / packed-4-bit,
//!   signed / unsigned integers, boolean, GGML legacy block-32
//!   quantization, GGML K-series block-256 quantization, GGML
//!   IQ-series importance-aware quantization) exposed through the
//!   [`dtype::TensorDtypeRegistry`] shape-IRI registry as
//!   `Term::Recurse` targets for container-format realizations.
//!   Compression-operator codomain context: per [ADR-058][09-adr-058]
//!   the κ-derivation is the framework's compression operator; tensor
//!   element types occupy the R-level of the operator-geometry
//!   codomain per [ADR-059][09-adr-059].
//! - **[`shape`]** — higher-rank tensor shape carriers:
//!   [`shape::Tensor3Shape`] (rank-3) and [`shape::Tensor4Shape`]
//!   (rank-4). Common GGUF / ONNX rank coverage; higher ranks compose
//!   through `partition_product!` per ADR-033/044.
//!
//! ## ConstrainedTypeShape declarations
//!
//! Per ADR-031's `Tensor<Element, Shape>` shape commitment:
//!
//! - **[`MatrixShape<ROWS, COLS, ELEM_BYTES>`]** — rank-2 tensor shape.
//! - **[`VectorShape<N, ELEM_BYTES>`]** — rank-1 tensor shape.
//! - **[`Tensor3Shape<D0, D1, D2, ELEM_BYTES>`][shape::Tensor3Shape]** — rank-3 tensor shape.
//! - **[`Tensor4Shape<D0, D1, D2, D3, ELEM_BYTES>`][shape::Tensor4Shape]** — rank-4 tensor shape.
//! - **[`dtype`]** — 43 fixed-byte-count element-type shapes
//!   (continuous floats, ONNX FLOAT8 / complex / packed-4-bit,
//!   signed / unsigned integers, boolean, GGML legacy / K-series /
//!   IQ-series quantization).
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
//! - [Wiki: 09 Architecture Decisions § ADR-037 — `HostBounds` ceilings on the principal data path][09-adr-037]
//! - [Wiki: 09 Architecture Decisions § ADR-057 — Bounded recursive structural typing][09-adr-057]
//! - [Wiki: 09 Architecture Decisions § ADR-058 — κ-derivation as the framework's compression operator][09-adr-058]
//! - [Wiki: 09 Architecture Decisions § ADR-059 — Atlas image inside E₈ as the codomain of κ-derivation][09-adr-059]
//! - [Wiki: 09 Architecture Decisions § ADR-060 — source-polymorphic value carrier (removes the byte-width caps)][09-adr-060]
//!
//! [09-adr-030]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-037]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-057]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-058]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-059]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-060]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod activation;
pub mod dtype;
pub mod shape;
pub mod tensor;
pub mod verbs;

pub use activation::{ActivationAxis, CpuI8VectorActivation};
pub use shape::{Tensor3Shape, Tensor4Shape};
pub use tensor::{CpuI8MatmulSquare, MatrixShape, TensorAxis, VectorShape};

/// Wiki ADR-031 standard-library version banner.
pub const STANDARD_LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");
