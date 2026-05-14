//! Prism standard-library cryptography sub-crate.
//!
//! `prism-crypto` realizes the cryptography Layer-3 of the standard
//! library named in [Wiki ADR-031][09-adr-031]: it declares the
//! cryptographic axis traits (`HashAxis`, `CurveAxis`, `SignatureAxis`,
//! `CommitmentAxis`) through the [`axis!`][09-adr-030] SDK macro and
//! supplies canonical impls per the wiki's ADR-031 roster. Per ADR-031
//! the standard library's role is to be the canonical reference for the
//! axes it declares — two crates that emit structurally-identical
//! axis traits content-address identically per ADR-017.
//!
//! ## Scope
//!
//! - **`HashAxis`** — content-addressing function. Canonical impls:
//!   [`Sha256Hasher`], [`Sha512Hasher`], [`Sha3_256Hasher`],
//!   [`Blake3Hasher`], [`Keccak256Hasher`]. Each impl is
//!   conformance-tested against the relevant standard's published
//!   vectors (FIPS-180-4 for SHA-2, FIPS-202 for SHA-3, BLAKE3 spec
//!   for BLAKE3) — see `tests/conformance.rs`.
//! - **`CurveAxis`**, **`SignatureAxis`** — declared per ADR-031's
//!   standard-library roster; concrete reference impls are scoped per
//!   axis maintenance policy (ADR-031's "operational policy"
//!   carve-out).
//! - **`CommitmentAxis`** — declared with reference impl
//!   [`MerkleRootCommitment`] composing the `HashAxis` SHA-256
//!   primitive into a binary-tree Merkle root.
//!
//! ## Closure under uor-foundation (ADR-013)
//!
//! Every axis trait declared here has `::uor_foundation::pipeline::AxisExtension`
//! as a supertrait — the `axis!` macro enforces this. Every concrete
//! impl is registered for `AxisExtension` via the companion macro
//! `axis_extension_impl_for_<axis>!` the macro emits.
//!
//! ## See also
//!
//! - [Wiki: 09 Architecture Decisions § ADR-030 — `axis!` SDK macro][09-adr-030]
//! - [Wiki: 09 Architecture Decisions § ADR-031 — `prism` is the standard library][09-adr-031]
//! - [Wiki: 09 Architecture Decisions § ADR-024 — Three-layer algebraic closure][09-adr-024]
//! - [Wiki: 12 Glossary § Crypto][12-glossary]
//!
//! [09-adr-024]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-030]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [12-glossary]: https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod commitment;
pub mod curve;
pub mod hash;
pub mod signature;

pub use commitment::{CommitmentAxis, MerkleRootCommitment};
pub use curve::CurveAxis;
pub use hash::{
    Blake3Hasher, HashAxis, Keccak256Hasher, Sha256Hasher, Sha3_256Hasher, Sha512Hasher,
};
pub use signature::SignatureAxis;

/// Wiki ADR-031 standard-library version banner. Each prism standard-
/// library sub-crate exposes this so application authors can introspect
/// the canonical-reference version of the axes it declares.
pub const STANDARD_LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");
