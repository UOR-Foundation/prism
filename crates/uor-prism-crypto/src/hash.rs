//! `HashAxis` declaration and FIPS-180-4 / FIPS-202 / BLAKE3 impls.
//!
//! The axis declaration and its impls live in the same module per the
//! Rust constraint that `#[macro_export]` macros emitted from a sub-
//! module by proc-macro expansion are not reachable from sibling
//! modules via either `use crate::<macro>` or bare-name resolution
//! (Rust issue #52234). Consolidating per-axis impls into one file
//! keeps the companion-macro call in scope at every invocation site.
//!
//! # ADR-055 substrate-Term verb body discipline
//!
//! Per [Wiki ADR-055](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
//! every `AxisExtension` impl carries a substrate-Term verb body via
//! the foundation-declared `SubstrateTermBody` supertrait. The
//! `axis!` companion macro in foundation-sdk 0.4.11 emits a default
//! empty `body_arena()` for every impl that doesn't supply an
//! explicit `body = |input| { … };` clause — ADR-055 names this the
//! **primitive-fast-path-equivalent realization**: the kernel-function
//! dispatch path below is byte-output-equivalent to recursive
//! fold-fusion through an empty body arena, so the hand-written
//! kernel bodies satisfy the discipline as-shipped.
//!
//! Explicit substrate-Term canonical body composition for the hash
//! family — compressing 32-/64-byte internal-state blocks via composed
//! `Add` (mod 2^32 or 2^64), `Xor`, `And`, `Or`, `Bnot`, plus a `rotr`
//! sub-verb composing `Or(Div(x, 2^k), Mul(x, 2^(width-k)))` per
//! ADR-054 § Substrate-Term realization examples plus pad-and-finalize
//! via `Concat` per ADR-056 — is **syntactically expressible** in
//! foundation-sdk 0.4.11's verb-body grammar (every PrimitiveOp call
//! form including `div`/`r#mod`/`pow`/`concat`/`le`/`lt`/`ge`/`gt`
//! plus `hash` axis invocation is admitted in verb/axis bodies per
//! ADR-056). The remaining work is **operational composition**: each
//! canonical hash impl's 64- / 80- / 24-round compression unfolded as
//! `fold_n` over the round-constant table is a published-roster
//! follow-on; the hand-written kernel bodies below remain the
//! operational form pending that composition.
//!
//! Byte-output equivalence with the canonical reference vectors
//! (FIPS-180-4, FIPS-202, BLAKE3 spec) is verified by direct vectors
//! in `tests/conformance.rs`. Per ADR-055's byte-output-equivalence-
//! at-every-input clause the kernel-dispatch path and any future
//! explicit substrate-Term `body` clause produce byte-identical
//! outputs.

#![allow(missing_docs)]

use sha2::Digest as Sha2Digest;
use sha3::Digest as Sha3Digest;
use uor_foundation::enforcement::{Hasher, ShapeViolation};
use uor_foundation_sdk::axis;

axis! {
    /// Wiki ADR-031 canonical hash-function family.
    ///
    /// Single kernel `hash(input: &[u8], out: &mut [u8])` emitting the
    /// digest of `input` into the first `Self::MAX_OUTPUT_BYTES` of
    /// `out`. Per ADR-030's signature constraint every axis-kernel
    /// method takes `(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>`;
    /// `HashAxis` exposes only one kernel because per-impl
    /// (Sha256Hasher / Sha512Hasher / Sha3_256Hasher / Keccak256Hasher
    /// / Blake3Hasher) the axis position in the model's `AxisTuple`
    /// already commits to a single digest family.
    pub trait HashAxis: AxisExtension {
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/HashAxis";
        const MAX_OUTPUT_BYTES: usize = 64;
        /// Compute the digest of `input` into `out[..n]`, returning `n`.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` if `out` is too small to hold the digest.
        fn hash(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

fn out_too_small_violation() -> ShapeViolation {
    ShapeViolation {
        shape_iri: "https://uor.foundation/axis/HashAxis",
        constraint_iri: "https://uor.foundation/axis/HashAxis/outputBuffer",
        property_iri: "https://uor.foundation/axis/outputBufferBytes",
        expected_range: "https://uor.foundation/axis/DigestBytesFit",
        min_count: 0,
        max_count: 0,
        kind: uor_foundation::ViolationKind::ValueCheck,
    }
}

// =====================================================================
// SHA-256 — FIPS-180-4 §6.2

const SHA256_BYTES: usize = 32;

/// FIPS-180-4 SHA-256 hasher. 32-byte digest.
#[derive(Debug, Clone)]
pub struct Sha256Hasher {
    inner: sha2::Sha256,
}

impl Default for Sha256Hasher {
    fn default() -> Self {
        Self::initial()
    }
}

impl HashAxis for Sha256Hasher {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/HashAxis/Sha256";
    const MAX_OUTPUT_BYTES: usize = SHA256_BYTES;

    fn hash(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if out.len() < SHA256_BYTES {
            return Err(out_too_small_violation());
        }
        let digest = sha2::Sha256::digest(input);
        out[..SHA256_BYTES].copy_from_slice(&digest);
        Ok(SHA256_BYTES)
    }
}

axis_extension_impl_for_hash_axis!(Sha256Hasher);

impl Hasher for Sha256Hasher {
    const OUTPUT_BYTES: usize = SHA256_BYTES;

    fn initial() -> Self {
        Self {
            inner: sha2::Sha256::new(),
        }
    }

    fn fold_byte(mut self, b: u8) -> Self {
        Sha2Digest::update(&mut self.inner, [b]);
        self
    }

    fn fold_bytes(mut self, bytes: &[u8]) -> Self {
        Sha2Digest::update(&mut self.inner, bytes);
        self
    }

    fn finalize(self) -> [u8; 32] {
        let result = Sha2Digest::finalize(self.inner);
        let mut out = [0u8; 32];
        out.copy_from_slice(&result);
        out
    }
}

// =====================================================================
// SHA-512 — FIPS-180-4 §6.4

const SHA512_BYTES: usize = 64;

/// FIPS-180-4 SHA-512 hasher. 64-byte digest.
#[derive(Debug, Clone)]
pub struct Sha512Hasher {
    inner: sha2::Sha512,
}

impl Default for Sha512Hasher {
    fn default() -> Self {
        <Self as Hasher<64>>::initial()
    }
}

impl HashAxis for Sha512Hasher {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/HashAxis/Sha512";
    const MAX_OUTPUT_BYTES: usize = SHA512_BYTES;

    fn hash(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if out.len() < SHA512_BYTES {
            return Err(out_too_small_violation());
        }
        let digest = sha2::Sha512::digest(input);
        out[..SHA512_BYTES].copy_from_slice(&digest);
        Ok(SHA512_BYTES)
    }
}

axis_extension_impl_for_hash_axis!(Sha512Hasher);

impl Hasher<64> for Sha512Hasher {
    const OUTPUT_BYTES: usize = SHA512_BYTES;

    fn initial() -> Self {
        Self {
            inner: sha2::Sha512::new(),
        }
    }

    fn fold_byte(mut self, b: u8) -> Self {
        Sha2Digest::update(&mut self.inner, [b]);
        self
    }

    fn fold_bytes(mut self, bytes: &[u8]) -> Self {
        Sha2Digest::update(&mut self.inner, bytes);
        self
    }

    fn finalize(self) -> [u8; 64] {
        let result = Sha2Digest::finalize(self.inner);
        let mut out = [0u8; 64];
        out.copy_from_slice(&result);
        out
    }
}

// =====================================================================
// SHA3-256 — FIPS-202

const SHA3_256_BYTES: usize = 32;

/// FIPS-202 SHA3-256 hasher. 32-byte digest.
#[derive(Debug, Clone)]
pub struct Sha3_256Hasher {
    inner: sha3::Sha3_256,
}

impl Default for Sha3_256Hasher {
    fn default() -> Self {
        Self::initial()
    }
}

impl HashAxis for Sha3_256Hasher {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/HashAxis/Sha3_256";
    const MAX_OUTPUT_BYTES: usize = SHA3_256_BYTES;

    fn hash(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if out.len() < SHA3_256_BYTES {
            return Err(out_too_small_violation());
        }
        let digest = sha3::Sha3_256::digest(input);
        out[..SHA3_256_BYTES].copy_from_slice(&digest);
        Ok(SHA3_256_BYTES)
    }
}

axis_extension_impl_for_hash_axis!(Sha3_256Hasher);

impl Hasher for Sha3_256Hasher {
    const OUTPUT_BYTES: usize = SHA3_256_BYTES;

    fn initial() -> Self {
        Self {
            inner: sha3::Sha3_256::new(),
        }
    }

    fn fold_byte(mut self, b: u8) -> Self {
        Sha3Digest::update(&mut self.inner, [b]);
        self
    }

    fn fold_bytes(mut self, bytes: &[u8]) -> Self {
        Sha3Digest::update(&mut self.inner, bytes);
        self
    }

    fn finalize(self) -> [u8; 32] {
        let result = Sha3Digest::finalize(self.inner);
        let mut out = [0u8; 32];
        out.copy_from_slice(&result);
        out
    }
}

// =====================================================================
// Keccak-256 — pre-FIPS-202 sponge (Ethereum-adopted variant)

const KECCAK256_BYTES: usize = 32;

/// Keccak-256 hasher. 32-byte digest. The pre-FIPS-202 sponge (the
/// variant adopted by Ethereum); distinguished from SHA3-256 by the
/// 0x01 vs 0x06 domain-separation byte.
#[derive(Debug, Clone)]
pub struct Keccak256Hasher {
    inner: sha3::Keccak256,
}

impl Default for Keccak256Hasher {
    fn default() -> Self {
        Self::initial()
    }
}

impl HashAxis for Keccak256Hasher {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/HashAxis/Keccak256";
    const MAX_OUTPUT_BYTES: usize = KECCAK256_BYTES;

    fn hash(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if out.len() < KECCAK256_BYTES {
            return Err(out_too_small_violation());
        }
        let digest = sha3::Keccak256::digest(input);
        out[..KECCAK256_BYTES].copy_from_slice(&digest);
        Ok(KECCAK256_BYTES)
    }
}

axis_extension_impl_for_hash_axis!(Keccak256Hasher);

impl Hasher for Keccak256Hasher {
    const OUTPUT_BYTES: usize = KECCAK256_BYTES;

    fn initial() -> Self {
        Self {
            inner: sha3::Keccak256::new(),
        }
    }

    fn fold_byte(mut self, b: u8) -> Self {
        Sha3Digest::update(&mut self.inner, [b]);
        self
    }

    fn fold_bytes(mut self, bytes: &[u8]) -> Self {
        Sha3Digest::update(&mut self.inner, bytes);
        self
    }

    fn finalize(self) -> [u8; 32] {
        let result = Sha3Digest::finalize(self.inner);
        let mut out = [0u8; 32];
        out.copy_from_slice(&result);
        out
    }
}

// =====================================================================
// BLAKE3

const BLAKE3_BYTES: usize = 32;

/// BLAKE3 hasher. 32-byte digest (the standard BLAKE3 output width;
/// XOF mode is not exposed at the axis level).
#[derive(Debug, Clone, Default)]
pub struct Blake3Hasher {
    inner: blake3::Hasher,
}

impl HashAxis for Blake3Hasher {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/HashAxis/Blake3";
    const MAX_OUTPUT_BYTES: usize = BLAKE3_BYTES;

    fn hash(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if out.len() < BLAKE3_BYTES {
            return Err(out_too_small_violation());
        }
        let digest = blake3::hash(input);
        out[..BLAKE3_BYTES].copy_from_slice(digest.as_bytes());
        Ok(BLAKE3_BYTES)
    }
}

axis_extension_impl_for_hash_axis!(Blake3Hasher);

impl Hasher for Blake3Hasher {
    const OUTPUT_BYTES: usize = BLAKE3_BYTES;

    fn initial() -> Self {
        Self::default()
    }

    fn fold_byte(mut self, b: u8) -> Self {
        self.inner.update(&[b]);
        self
    }

    fn fold_bytes(mut self, bytes: &[u8]) -> Self {
        self.inner.update(bytes);
        self
    }

    fn finalize(self) -> [u8; 32] {
        *self.inner.finalize().as_bytes()
    }
}
