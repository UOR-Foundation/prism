//! Prism standard-library tensor-compute sub-crate.
//!
//! `prism-tensor` realizes the tensor Layer-3 of the standard library
//! named in [Wiki ADR-031][09-adr-031]: declares `TensorAxis` and
//! `ActivationAxis` through the [`axis!`][09-adr-030] SDK macro and
//! supplies CPU integer-precision reference impls preserving
//! bit-determinism per fixed `(HostTypes, HostBounds, AxisTuple)`
//! selection (per ADR-030's per-axis substitution-determinism note).
//!
//! ## Scope
//!
//! The byte-slice-in-byte-slice-out kernel signature ADR-030 mandates
//! (`fn k(input: &[u8], out: &mut [u8])`) constrains kernels to fixed
//! shapes: variable-rank tensor compute is a verb-level concern
//! composed through `partition_product!` per ADR-033/044. The
//! reference impls expose canonical fixed-shape kernels:
//!
//! - **`CpuI8Tensor4x4Matmul`** — `TensorAxis::matmul` over two 4×4
//!   `i8` matrices, emitting a 4×4 `i16` product (saturating).
//! - **`CpuI8VectorActivation16`** — `ActivationAxis::{relu, sigmoid_q}`
//!   over a fixed-length 16-element `i8` vector.
//!
//! ## Closure under uor-foundation (ADR-013)
//!
//! Every axis trait declared here has `::uor_foundation::pipeline::AxisExtension`
//! as a supertrait; each impl is registered via the companion macro
//! `axis_extension_impl_for_<axis>!` per ADR-030.
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

pub use activation::{ActivationAxis, CpuI8VectorActivation16};
pub use tensor::{CpuI8Tensor4x4Matmul, TensorAxis};

/// Wiki ADR-031 standard-library version banner.
pub const STANDARD_LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");
