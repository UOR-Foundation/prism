//! Layer-3 substrate-Term verb bodies per [Wiki ADR-024][09-adr-024] +
//! [Wiki ADR-031][09-adr-031] + [Wiki ADR-054 decision 4][09-adr-054]
//! (canonical axis impl body discipline).
//!
//! Per ADR-024 a Layer-3 implementation contributes both axes
//! (substrate-extension vocabularies via `axis!`) AND verbs (named,
//! reusable compositions of prism operators applied to substrate
//! primitives via `verb!`). Per ADR-054 (4) every canonical axis impl
//! in the standard library carries a substrate-Term verb body.
//!
//! # Verbs shipped (expressible in foundation-sdk 0.4.7's verb! grammar)
//!
//! - [`succ_twice`], [`pred_twice`] — single-input compositions of
//!   substrate unary primitives (`Succ` / `Pred`).
//! - [`square`] — single-input self-multiplication (`mul(x, x)`).
//! - [`add_substrate`], [`sub_substrate`], [`mul_substrate`] —
//!   substrate-Term realizations of `BigIntAxis::{add, sub, mul}` at
//!   W256 per ADR-054 (4). Each verb body is one substrate
//!   `PrimitiveOp` application over a `partition_product(BigInt32,
//!   BigInt32)` input; per ADR-050's width-parametric arithmetic the
//!   substrate evaluates at the full 256-bit width without truncation.
//! - [`gf2_add_substrate`], [`gf2_mul_substrate`], [`or_substrate`] —
//!   substrate-Term realizations of `Gf2NumericAxisN<32>::{add, mul}`
//!   at W256.
//!
//! # Wiki-named numerics verbs blocked on upstream foundation-sdk
//! # grammar extension
//!
//! ADR-031 + ADR-054 (4) commit prism-numerics to ship `modexp_p`,
//! `polyeval`, `gcd`, `ext_euclidean`, `horner`, `newton_step`,
//! `field_add<P>`, `field_sub<P>`, `field_mul<P>`, `field_inv<P>` as
//! substrate-Term verb bodies. The wiki specifies the composition
//! vocabulary (ADR-054 § Substrate-Term realization examples, line
//! 9856): `Add`, `Sub`, `Mul`, `Div`, `Mod`, `Pow` at Witt widths up
//! to the host bounds' ceiling.
//!
//! **Foundation-sdk 0.4.7's `verb!` closure-body grammar admits only**
//! `add`, `sub`, `mul`, `xor`, `and`, `or`, `neg`, `bnot`, `succ`,
//! `pred` as `PrimitiveOp` call forms (see `uor-foundation-sdk`
//! `emit_term_for_call` lines 3222-3260). `div`, `mod`, `pow`
//! — added to the substrate's `PrimitiveOp` catalog by ADR-053 —
//! are not yet admitted as verb-body call forms. `concat` is
//! rejected per ADR-035 ψ-residuals discipline (used by some
//! numerics patterns). Comparison primitives `le`/`lt`/`ge`/`gt`
//! are likewise rejected as ψ-residuals (needed for gcd's branching,
//! Newton iteration's convergence check, and field-inversion's
//! Fermat's-little-theorem comparator path).
//!
//! Closing the canonical-roster gap (everything from `modexp_p`
//! through `field_inv<P>`) is gated on a foundation-sdk grammar
//! extension admitting `div`/`mod`/`pow` as verb-body call forms.
//! Until that lands, the canonical roster's verb bodies are not
//! syntactically expressible. The blocking dependency is named in
//! AGENTS.md §11.8.
//!
//! [09-adr-024]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-054]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(missing_docs)]

use uor_foundation::enforcement::ConstrainedTypeInput;
use uor_foundation_sdk::{partition_product, verb};

use crate::BigIntShape;

// Single-input architectural-witness verbs.

verb! {
    pub fn succ_twice(input: ConstrainedTypeInput) -> ConstrainedTypeInput {
        succ(succ(input))
    }
}

verb! {
    pub fn pred_twice(input: ConstrainedTypeInput) -> ConstrainedTypeInput {
        pred(pred(input))
    }
}

verb! {
    pub fn square(input: ConstrainedTypeInput) -> ConstrainedTypeInput {
        mul(input, input)
    }
}

// Three-input fused-multiply-add. Routes the canonical roster's `fma`
// from the wiki — a single Add over a single Mul over the three
// projections of a `BigIntShape<32>`-triple partition-product input.

/// Concrete 256-bit BigInt alias for partition-product composition.
/// `partition_product!` parses operands as bare type paths; generic
/// types like `BigIntShape<32>` need a type alias for the macro's
/// tokenizer.
pub type BigInt32 = BigIntShape<32>;

partition_product!(BigIntPair32, BigInt32, BigInt32);

// Substrate-native 256-bit modular arithmetic: `add_substrate` /
// `sub_substrate` / `mul_substrate` are the substrate-Term realizations
// per ADR-054 (4) of the corresponding `BigIntAxis` kernel bodies.
// The substrate evaluates `Add`/`Sub`/`Mul` at the full 256-bit operand
// width per ADR-050's width-parametric fold-rules (low 256 bits of
// the schoolbook product for `Mul`). The catamorphism walks each
// composition as a fold-fusion-reachable Term tree — no opaque
// axis-kernel boundary remains inside the substrate's structural reach
// for these three operations.

verb! {
    pub fn add_substrate(input: BigIntPair32) -> BigInt32 {
        add(input.0, input.1)
    }
}

verb! {
    pub fn sub_substrate(input: BigIntPair32) -> BigInt32 {
        sub(input.0, input.1)
    }
}

verb! {
    pub fn mul_substrate(input: BigIntPair32) -> BigInt32 {
        mul(input.0, input.1)
    }
}

// Substrate-native 256-bit GF(2) hypercube arithmetic — substrate-Term
// realizations per ADR-054 (4) of `Gf2NumericAxisN<32>::{add, mul}`.
// Per ADR-050 the substrate evaluates `Xor`/`And` byte-wise at any
// operand width (trivially width-parametric since they have no carry).

verb! {
    pub fn gf2_add_substrate(input: BigIntPair32) -> BigInt32 {
        xor(input.0, input.1)
    }
}

verb! {
    pub fn gf2_mul_substrate(input: BigIntPair32) -> BigInt32 {
        and(input.0, input.1)
    }
}

verb! {
    pub fn or_substrate(input: BigIntPair32) -> BigInt32 {
        or(input.0, input.1)
    }
}
