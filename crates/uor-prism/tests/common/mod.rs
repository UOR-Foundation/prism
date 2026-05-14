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
