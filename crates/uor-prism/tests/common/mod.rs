//! Shared test fuel: FNV-1a `Hasher` impls at the 16- and 24-byte
//! `OUTPUT_BYTES` widths.
//!
//! Per [Wiki ADR-031][09-adr-031], the prism standard library ships
//! canonical cryptographic `HashAxis` impls covering the published
//! digest widths (32-byte SHA-256, SHA-3, BLAKE3, Keccak; 64-byte
//! SHA-512). The narrower 16- and 24-byte widths are reachable through
//! the `Hasher::OUTPUT_BYTES` axis but have no canonical cryptographic
//! primitive at those widths; these FNV-1a stand-ins exist to vary the
//! axis-width parameter across the scaling test suite, not to provide
//! cryptographic security. The 32-byte axis-width row of the scaling
//! matrix uses [`prism::crypto::Sha256Hasher`] directly.
//!
//! `tests/common/mod.rs` (directory form, not `tests/common.rs`) is
//! how cargo lets multiple integration test files share helper code
//! without each being treated as its own test binary.
//!
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(dead_code)]

use prism::vocabulary::Hasher;

const FNV_PRIME: u64 = 0x100_0000_01b3;
const FNV_OFFSET_A: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_OFFSET_B: u64 = 0x8422_2325_cbf2_9ce4;
const FNV_OFFSET_C: u64 = 0x1234_5678_9abc_def0;

/// 16-byte FNV-1a substrate — two 64-bit lanes.
#[derive(Clone, Copy)]
pub(crate) struct Fnv16 {
    a: u64,
    b: u64,
}

impl Hasher for Fnv16 {
    const OUTPUT_BYTES: usize = 16;

    fn initial() -> Self {
        Self {
            a: FNV_OFFSET_A,
            b: FNV_OFFSET_B,
        }
    }

    fn fold_byte(mut self, x: u8) -> Self {
        let xv = u64::from(x);
        self.a = (self.a ^ xv).wrapping_mul(FNV_PRIME);
        self.b = (self.b ^ xv.rotate_left(8)).wrapping_mul(FNV_PRIME);
        self
    }

    fn finalize(self) -> [u8; 32] {
        let mut buf = [0u8; 32];
        buf[..8].copy_from_slice(&self.a.to_be_bytes());
        buf[8..16].copy_from_slice(&self.b.to_be_bytes());
        buf
    }
}

/// 24-byte FNV-1a substrate — three 64-bit lanes.
#[derive(Clone, Copy)]
pub(crate) struct Fnv24 {
    a: u64,
    b: u64,
    c: u64,
}

impl Hasher for Fnv24 {
    const OUTPUT_BYTES: usize = 24;

    fn initial() -> Self {
        Self {
            a: FNV_OFFSET_A,
            b: FNV_OFFSET_B,
            c: FNV_OFFSET_C,
        }
    }

    fn fold_byte(mut self, x: u8) -> Self {
        let xv = u64::from(x);
        self.a = (self.a ^ xv).wrapping_mul(FNV_PRIME);
        self.b = (self.b ^ xv.rotate_left(8)).wrapping_mul(FNV_PRIME);
        self.c = (self.c ^ xv.rotate_left(16)).wrapping_mul(FNV_PRIME);
        self
    }

    fn finalize(self) -> [u8; 32] {
        let mut buf = [0u8; 32];
        buf[..8].copy_from_slice(&self.a.to_be_bytes());
        buf[8..16].copy_from_slice(&self.b.to_be_bytes());
        buf[16..24].copy_from_slice(&self.c.to_be_bytes());
        buf
    }
}

// ---- TestHostBounds: an application-declared `HostBounds` impl ----
//
// Per wiki ADR-060, the foundation no longer ships a `DefaultHostBounds`:
// "there is no 'default' application, so the foundation supplies no
// default policy. Every application declares its own `impl HostBounds`."
// The prism standard library re-exports the `HostBounds` trait but
// provides no concrete impl (a default would re-introduce exactly the
// hidden-choice the ADR removes). prism's test suite is the
// "application" here, so it declares its own bounds. The values match
// the pre-0.5.0 foundation defaults the byte-width-cap ADR-037 carried,
// minus the 12 byte-width caps ADR-060 removed (those now derive from
// these structural-count primitives via the foundation `*_carrier_bytes`
// const fns).
use prism::vocabulary::HostBounds;

/// Test-only `HostBounds` carrying the pre-0.5.0 foundation default
/// capacity values. The 14 retained associated constants per ADR-060;
/// the 12 byte-width caps are gone (carrier widths derive from these).
#[derive(Clone, Copy)]
pub(crate) struct TestHostBounds;

impl HostBounds for TestHostBounds {
    const FINGERPRINT_MIN_BYTES: usize = 16;
    const FINGERPRINT_MAX_BYTES: usize = 32;
    const TRACE_MAX_EVENTS: usize = 256;
    const WITT_LEVEL_MAX_BITS: u32 = 64;
    const FOLD_UNROLL_THRESHOLD: usize = 8;
    const BETTI_DIMENSION_MAX: usize = 8;
    const NERVE_CONSTRAINTS_MAX: usize = 8;
    const NERVE_SITES_MAX: usize = 8;
    const JACOBIAN_SITES_MAX: usize = 8;
    const RECURSION_TRACE_DEPTH_MAX: usize = 16;
    const OP_CHAIN_DEPTH_MAX: usize = 8;
    const AFFINE_COEFFS_MAX: usize = 8;
    const CONJUNCTION_TERMS_MAX: usize = 8;
    const UNFOLD_ITERATIONS_MAX: usize = 256;
}
