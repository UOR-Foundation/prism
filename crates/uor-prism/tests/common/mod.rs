//! Shared test fuel: FNV-1a `Hasher` impls at multiple `OUTPUT_BYTES`
//! widths.
//!
//! These impls are **test fuel only** — they exist to vary the
//! `Hasher::OUTPUT_BYTES` axis across the integration test suite and
//! to satisfy the `Hasher` bound on `pipeline::run`. Per AGENTS.md
//! § 11.2 (exclusion criteria), prism does not ship cryptographic
//! `Hasher` implementations; the structural form here follows the
//! foundation `Hasher` trait's normative documentation example.
//!
//! `tests/common/mod.rs` (directory form, not `tests/common.rs`) is
//! how cargo lets multiple integration test files share helper code
//! without each being treated as its own test binary.

#![allow(dead_code)]

use prism::vocabulary::Hasher;

const FNV_PRIME: u64 = 0x100_0000_01b3;
const FNV_OFFSET_A: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_OFFSET_B: u64 = 0x8422_2325_cbf2_9ce4;
const FNV_OFFSET_C: u64 = 0x1234_5678_9abc_def0;
const FNV_OFFSET_D: u64 = 0xfedc_ba98_7654_3210;

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

/// 32-byte FNV-1a substrate — four 64-bit lanes (saturates the default
/// `<DefaultHostBounds as HostBounds>::FINGERPRINT_MAX_BYTES = 32`).
#[derive(Clone, Copy)]
pub(crate) struct Fnv32 {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

impl Hasher for Fnv32 {
    const OUTPUT_BYTES: usize = 32;

    fn initial() -> Self {
        Self {
            a: FNV_OFFSET_A,
            b: FNV_OFFSET_B,
            c: FNV_OFFSET_C,
            d: FNV_OFFSET_D,
        }
    }

    fn fold_byte(mut self, x: u8) -> Self {
        let xv = u64::from(x);
        self.a = (self.a ^ xv).wrapping_mul(FNV_PRIME);
        self.b = (self.b ^ xv.rotate_left(8)).wrapping_mul(FNV_PRIME);
        self.c = (self.c ^ xv.rotate_left(16)).wrapping_mul(FNV_PRIME);
        self.d = (self.d ^ xv.rotate_left(24)).wrapping_mul(FNV_PRIME);
        self
    }

    fn finalize(self) -> [u8; 32] {
        let mut buf = [0u8; 32];
        buf[..8].copy_from_slice(&self.a.to_be_bytes());
        buf[8..16].copy_from_slice(&self.b.to_be_bytes());
        buf[16..24].copy_from_slice(&self.c.to_be_bytes());
        buf[24..32].copy_from_slice(&self.d.to_be_bytes());
        buf
    }
}
