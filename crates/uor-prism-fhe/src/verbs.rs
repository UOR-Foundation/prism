//! Layer-3 substrate-Term verb bodies per [Wiki ADR-054][09-adr-054]
//! decision 4 (canonical axis impl body discipline).
//!
//! Per ADR-054 (4) every canonical axis impl in the standard library
//! carries a substrate-Term `verb!` body composing prism operators
//! over substrate `PrimitiveOp`s; the verb body is the structural
//! witness the catamorphism's fold-fusion reach extends into.
//!
//! [`add_ciphertexts_verb`] is the substrate-Term realization of
//! `OneTimePadFhe<32>::add_ciphertexts` at the canonical
//! 32-byte block width: a single `Term::Application { operator:
//! PrimitiveOp::Xor }` at W256 over the partition-product of two
//! ciphertext blocks. Per ADR-050's width-parametric arithmetic
//! fold-rules the substrate evaluates this Xor at the full block
//! width without truncation. Byte-output equivalence with the
//! hand-written kernel is locked by `tests/conformance.rs`.
//!
//! The one-line substrate-Term form is what ADR-054 commits to: no
//! opaque axis-kernel boundary inside the substrate's structural
//! reach. The hand-written kernel in `fhe.rs` is the
//! algorithm-strategy surface — inlining-friendly LLVM codegen per the
//! three-way responsibility split in ADR-024 (structural correctness
//! foundation-owned via the substrate-Term form; algorithm-strategy
//! implementation-owned).
//!
//! [09-adr-054]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(missing_docs)]

use uor_foundation_sdk::{partition_product, verb};

use crate::CiphertextShape;

/// Concrete 32-byte ciphertext alias for partition-product
/// composition. `partition_product!` parses operands as bare type
/// paths; generic types like `CiphertextShape<32>` need a type
/// alias for the macro's tokenizer to accept them.
pub type Ciphertext32 = CiphertextShape<32>;

partition_product!(CiphertextPair32, Ciphertext32, Ciphertext32);

verb! {
    pub fn add_ciphertexts_verb(input: CiphertextPair32) -> Ciphertext32 {
        xor(input.0, input.1)
    }
}
