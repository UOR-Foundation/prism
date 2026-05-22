//! Parametric `ConstrainedTypeShape` carriers per [Wiki ADR-031][09-adr-031]:
//! `Digest<N>`, `PublicKey<N>`, `Signature<N>`.
//!
//! Each shape is a phantom `N`-byte carrier. Per ADR-017's closure rule
//! identity flows through `(SITE_COUNT, CONSTRAINTS)`, so distinct
//! shape types with the same byte count content-address identically —
//! the Rust name is for the developer, the content-address is for the
//! ecosystem.
//!
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

use uor_foundation::enforcement::GroundedShape;
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef, IntoBindingValue, TermValue};

macro_rules! parametric_byte_shape {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy)]
        pub struct $name<const BYTES: usize>;

        impl<const BYTES: usize> Default for $name<BYTES> {
            fn default() -> Self {
                Self
            }
        }

        impl<const BYTES: usize> ConstrainedTypeShape for $name<BYTES> {
            const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
            const SITE_COUNT: usize = BYTES;
            const CONSTRAINTS: &'static [ConstraintRef] = &[];
            #[allow(clippy::cast_possible_truncation)]
            const CYCLE_SIZE: u64 = 256u64.saturating_pow(BYTES as u32);
        }

        impl<const BYTES: usize> uor_foundation::pipeline::__sdk_seal::Sealed
            for $name<BYTES>
        {
        }
        impl<const BYTES: usize> GroundedShape for $name<BYTES> {}
        impl<'a, const BYTES: usize> IntoBindingValue<'a> for $name<BYTES> {
            fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
                TermValue::empty()
            }
        }
    };
}

parametric_byte_shape! {
    /// Hash-digest shape: `N` bytes carrying a hash output.
    ///
    /// Per ADR-031's `Digest<32>` / `Digest<64>` shape commitment.
    /// Common widths: `Digest<32>` (SHA-256, SHA3-256, Keccak-256,
    /// BLAKE3), `Digest<64>` (SHA-512), `Digest<48>` (SHA-384).
    Digest
}

parametric_byte_shape! {
    /// Public-key shape: `N` bytes carrying an elliptic-curve public
    /// key (compressed or raw, application-defined). Per ADR-031's
    /// `PublicKey<32>` shape commitment.
    PublicKey
}

parametric_byte_shape! {
    /// Signature shape: `N` bytes carrying a signature. Per ADR-031's
    /// `Signature<64>` shape commitment.
    ///
    /// Common widths: `Signature<64>` (Ed25519, secp256k1 raw r||s),
    /// `Signature<96>` (BLS12-381 G1 compressed).
    Signature
}
