//! Prism standard-library homomorphic-encryption sub-crate.
//!
//! `prism-fhe` realizes the homomorphic-encryption Layer-3 of the
//! standard library named in [Wiki ADR-031][09-adr-031]: declares
//! `FheAxis` through the [`axis!`][09-adr-030] SDK macro and supplies
//! a reference impl suitable for conformance testing.
//!
//! ## Scope
//!
//! The wiki's ADR-031 names canonical FHE-scheme impls (TFHE, BGV,
//! CKKS); per ADR-031 the specific impl roster is operational policy.
//! This crate ships the reference impl [`OneTimePadFheAxis`] —
//! a one-time-pad (XOR with a key-stream) "homomorphic" scheme that
//! satisfies the additive-over-ciphertexts axis contract trivially.
//! Production FHE schemes are application-level integrations; the
//! `axis!` declaration here is what makes them composable through the
//! prism standard-library Layer-3 surface.
//!
//! ## Closure under uor-foundation (ADR-013)
//!
//! The `FheAxis` trait has `::uor_foundation::pipeline::AxisExtension`
//! as a supertrait; the reference impl is registered via
//! `axis_extension_impl_for_fhe_axis!` per ADR-030.
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

pub mod fhe;

pub use fhe::{FheAxis, OneTimePadFheAxis};

/// Wiki ADR-031 standard-library version banner.
pub const STANDARD_LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");
