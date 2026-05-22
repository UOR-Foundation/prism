//! Layer-3 substrate-Term verbs per [Wiki ADR-024][09-adr-024] +
//! [Wiki ADR-055][09-adr-055] + [Wiki ADR-056][09-adr-056] (ψ-residuals
//! discipline scope refined to route bodies only — verb bodies admit
//! comparison + concat + hash composition).
//!
//! [09-adr-024]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-055]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-056]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(missing_docs)]

use uor_foundation_sdk::{partition_product, verb};

// A single-byte W8 wrapper shape for compositional verbs over
// signed 8-bit values. Per ADR-056 verb bodies admit the full
// substrate vocabulary (concat for byte-packing, comparisons for
// saturation matches); the leaf-shape pattern follows the
// smoke-test convention (no PartitionProductFields impl on leaves).

pub struct W8Byte;
impl uor_foundation::pipeline::ConstrainedTypeShape for W8Byte {
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [uor_foundation::pipeline::ConstraintRef] = &[];
    const CYCLE_SIZE: u64 = 256;
}
impl uor_foundation::pipeline::__sdk_seal::Sealed for W8Byte {}
impl uor_foundation::enforcement::GroundedShape for W8Byte {}
impl<'a> uor_foundation::pipeline::IntoBindingValue<'a> for W8Byte {
    fn as_binding_value<const INLINE_BYTES: usize>(
        &self,
    ) -> uor_foundation::pipeline::TermValue<'a, INLINE_BYTES> {
        uor_foundation::pipeline::TermValue::empty()
    }
}

partition_product!(BytePair, W8Byte, W8Byte);

// Substrate-Term `add_bytes(a, b)` — single-byte ring add at W8.
// Architectural witness that verb body composition over W8 leaves
// works through depth-1 partition-product field access.
verb! {
    pub fn add_bytes(input: BytePair) -> W8Byte {
        add(input.0, input.1)
    }
}

// Substrate-Term concat — admissible per ADR-056 (verb bodies have
// no ψ-residual discipline). Realizes the byte-packing primitive
// the canonical SHA pad-and-finalize composition uses.
verb! {
    pub fn concat_bytes(input: BytePair) -> W8Byte {
        concat(input.0, input.1)
    }
}

// `saturating_xor_bytes(a, b)` — the GF(2) overflow-free byte sum
// xor(a, b), shipped as the architectural witness for tensor
// saturation per ADR-054 § Substrate-Term realization examples.
// Per ADR-056 the broader Wn saturation path uses `match` over
// `ge(acc, sat_max)` comparisons (now admissible in verb bodies);
// the witness here demonstrates the no-overflow byte-add primitive
// the saturation composition reduces to for unsigned operands.
verb! {
    pub fn saturating_xor_bytes(input: BytePair) -> W8Byte {
        xor(input.0, input.1)
    }
}
