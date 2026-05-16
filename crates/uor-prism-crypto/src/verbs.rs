//! Layer-3 substrate-Term verbs per [Wiki ADR-024][09-adr-024] +
//! [Wiki ADR-055][09-adr-055] + [Wiki ADR-056][09-adr-056].
//!
//! Per ADR-056 the ψ-residuals discipline applies only to the route
//! body's syntactic surface; verb bodies admit the full substrate
//! vocabulary including `hash(...)` axis invocations and `concat(...)`.
//! This unblocks the canonical cryptographic compound verbs the wiki
//! commits to per ADR-031: HMAC, HKDF, ECDSA, Merkle-tree construction.
//!
//! # Verbs shipped
//!
//! - [`merkle_reduce_pair`] — Merkle-tree internal-node reducer
//!   `H(left || right)` over a `partition_product(Digest32, Digest32)`
//!   input. Composes `hash(concat(input.0, input.1))`. This is the
//!   reducer that drives any Merkle-tree `tree_fold` composition.
//! - [`hmac_inner_prep`] — HMAC inner-hash step
//!   `H(K_ipad || message)`. The full HMAC composition
//!   `H(K_opad || H(K_ipad || message))` chains two instances of this
//!   verb plus an outer key prep — the wiki names this as the
//!   canonical prism-crypto verb roster's HMAC realization per
//!   ADR-031.
//!
//! [09-adr-024]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-055]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-056]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(missing_docs)]

use uor_foundation_sdk::{partition_product, verb};

use crate::Digest;

/// 32-byte digest leaf (alias for the parametric `Digest<32>`).
pub type Digest32 = Digest<32>;

/// 64-byte block leaf for HMAC's keyed-prefix composition.
pub type HmacBlock64 = Digest<64>;

partition_product!(DigestPair32, Digest32, Digest32);

verb! {
    pub fn merkle_reduce_pair(input: DigestPair32) -> Digest32 {
        hash(concat(input.0, input.1))
    }
}

partition_product!(HmacInputs, HmacBlock64, HmacBlock64);

verb! {
    pub fn hmac_inner_prep(input: HmacInputs) -> Digest32 {
        hash(concat(input.0, input.1))
    }
}
