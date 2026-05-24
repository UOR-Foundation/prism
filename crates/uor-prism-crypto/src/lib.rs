//! Prism standard-library cryptography sub-crate.
//!
//! `prism-crypto` realizes the cryptography Layer-3 of the standard
//! library named in [Wiki ADR-031][09-adr-031]: it declares the
//! cryptographic axis traits (`HashAxis`, `CurveAxis`, `SignatureAxis`,
//! `CommitmentAxis`) through the [`axis!`][09-adr-030] SDK macro and
//! supplies canonical impls plus matching `ConstrainedTypeShape`
//! carriers per the wiki's ADR-031 roster.
//!
//! ## Scope
//!
//! - **`HashAxis`** — content-addressing function. Canonical impls:
//!   [`Sha256Hasher`], [`Sha512Hasher`], [`Sha3_256Hasher`],
//!   [`Blake3Hasher`], [`Keccak256Hasher`]. Each impl is
//!   conformance-tested against the relevant standard's published
//!   vectors (FIPS-180-4 for SHA-2, FIPS-202 for SHA-3, BLAKE3 spec
//!   for BLAKE3) — see `tests/conformance.rs`.
//! - **`CommitmentAxis`** — composes any `HashAxis` impl into a
//!   Merkle-root commitment via [`MerkleRoot<H, LEAF_BYTES>`]. The
//!   default alias [`MerkleRootCommitment`] is SHA-256. Per ADR-031
//!   this is the canonical example of standard-library
//!   cross-sub-crate composition.
//! - **`CurveAxis`**, **`SignatureAxis`** — declared per ADR-031's
//!   standard-library roster; concrete reference impls are scoped per
//!   axis maintenance policy (ADR-031's "operational policy"
//!   carve-out).
//!
//! ## ConstrainedTypeShape declarations
//!
//! Per ADR-031's shape-declaration commitment, the canonical
//! cryptographic value-carriers are parametric over byte-width:
//!
//! - **[`Digest<N>`]** — hash output. `Digest<32>` for SHA-256 /
//!   SHA3-256 / Keccak-256 / BLAKE3; `Digest<48>` for SHA-384;
//!   `Digest<64>` for SHA-512.
//! - **[`PublicKey<N>`]** — public-key bytes.
//! - **[`Signature<N>`]** — signature bytes.
//! - **[`MerkleProofShape<MAX_DEPTH, LEAF_BYTES>`]** — Merkle-inclusion
//!   proof.
//!
//! Each shape is `GroundedShape + IntoBindingValue`-bound for use as
//! the `Output` of a `prism_model!`-declared application.
//!
//! ## Closure under uor-foundation (ADR-013)
//!
//! Every axis trait declared here has `::uor_foundation::pipeline::AxisExtension`
//! as a supertrait — the `axis!` macro enforces this. Concrete impls
//! that take no type parameters use the companion-macro lane; the
//! parametric `MerkleRoot<H, LEAF_BYTES>` hand-writes its
//! `AxisExtension` impl since the companion macro takes `:ident`.
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
pub mod shapes;
pub mod signature;
pub mod verbs;

pub use commitment::{CommitmentAxis, MerkleProofShape, MerkleRoot, MerkleRootCommitment};
pub use curve::CurveAxis;
pub use hash::{
    Blake3Hasher, HashAxis, Keccak256Hasher, Sha256Hasher, Sha3_256Hasher, Sha512Hasher,
};
pub use shapes::{Digest, PublicKey, Signature};
pub use signature::SignatureAxis;

/// Wiki ADR-031 standard-library version banner. Each prism standard-
/// library sub-crate exposes this so application authors can introspect
/// the canonical-reference version of the axes it declares.
pub const STANDARD_LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");
