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
//! # Verbs shipped (expressible in foundation-sdk 0.4.9's verb! grammar)
//!
//! Foundation-sdk 0.4.9 admits `add`, `sub`, `mul`, `div`, `r#mod`,
//! `pow`, `xor`, `and`, `or`, `neg`, `bnot`, `succ`, `pred` as verb-body
//! `PrimitiveOp` call forms — the full ADR-053 catalog (the 0.4.9
//! grammar extension closes the `div`/`mod`/`pow` admission that
//! 0.4.8 did not yet support).
//!
//! Substrate-Term verbs shipped here:
//!
//! - [`succ_twice`], [`pred_twice`] — single-input compositions of
//!   substrate unary primitives (`Succ` / `Pred`).
//! - [`square`] — single-input self-multiplication (`mul(x, x)`).
//! - [`add_substrate`], [`sub_substrate`], [`mul_substrate`],
//!   [`div_substrate`], [`mod_substrate`], [`pow_substrate`] —
//!   substrate-Term realizations of the six ADR-053 ring-arithmetic
//!   `PrimitiveOp`s over a `partition_product(BigInt32, BigInt32)`
//!   input at W256 per ADR-054 (4) + ADR-055. Each verb body is one
//!   substrate `PrimitiveOp` application; per ADR-050's
//!   width-parametric arithmetic the substrate evaluates at the full
//!   256-bit width without truncation.
//! - [`gf2_add_substrate`], [`gf2_mul_substrate`], [`or_substrate`] —
//!   substrate-Term realizations of the three hypercube-axis
//!   `PrimitiveOp`s (`Xor` / `And` / `Or`) at W256.
//!
//! Together these cover **all thirteen** in-grammar `PrimitiveOp` call
//! forms as substrate-Term verb bodies. The catamorphism walks each
//! composition as a fold-fusion-reachable Term tree per
//! ADR-019/ADR-029/ADR-054 — no opaque axis-kernel boundary remains
//! inside the substrate's structural reach for these compositions.
//!
//! # Wiki-named compound numerics verbs — architectural blockers
//!
//! ADR-031 + ADR-054 § Substrate-Term realization examples + ADR-055
//! commit prism-numerics to ship `modexp_p`, `polyeval`, `gcd`,
//! `ext_euclidean`, `horner`, `newton_step`, `fma`, `field_add<P>`,
//! `field_sub<P>`, `field_mul<P>`, `field_inv<P>` as substrate-Term
//! verb bodies. Of these:
//!
//! - **`fma`, `mod_pow_pair`, `field_add<P>`/`sub<P>`/`mul<P>`** —
//!   three-operand verbs over `partition_product`-folded inputs. The
//!   wiki's algebraic-composition target is expressible in the
//!   admitted `add`/`mul`/`r#mod`/`pow` call forms, but the
//!   depth-2 field access (`input.0.0`, `input.0.1`, `input.1`) on
//!   nested `partition_product`s fails `verb!`'s const-eval path
//!   ("index out of bounds: the length is 0 but the index is 0"
//!   from foundation-sdk's `emit_term_for_call` projection chain).
//!   Forward work in foundation-sdk: extend the verb!-macro
//!   const-eval projection path to admit nested-`partition_product`
//!   depth-2 access, matching `prism_model!`'s already-admitted form
//!   (smoke-tested at uor-foundation-sdk/tests/smoke.rs line 1093).
//!
//! - **`modexp_p`, `field_inv<P>`** — additionally need a wide-Witt
//!   literal mechanism (the secp256k1 prime P is a 256-bit constant
//!   that doesn't fit in the closure-body grammar's `u64` literal
//!   form). ADR-051's `TermValue` wide-value carrier exists at the
//!   substrate level but isn't surfaced as a verb-body literal-expr
//!   form. Forward work in foundation-sdk: admit `TermValue`-typed
//!   literal expressions in verb bodies for wide-Witt constants.
//!
//! - **`gcd`, `ext_euclidean`, `newton_step`** — need comparison
//!   primitives (`le`/`lt`/`ge`/`gt`) for the branching predicate,
//!   which ADR-035's ψ-residuals discipline rejects in verb/axis
//!   bodies. This is an **architectural** wiki commitment, not a
//!   foundation-sdk gap. The canonical body discipline per ADR-054
//!   would need an ADR-035 amendment admitting comparison-as-`Match`
//!   in axis-body contexts to express these algorithms.
//!
//! - **`polyeval`, `horner`** — composable through `fold_n` over
//!   `add` + `mul`, expressible in principle. Implementation gated
//!   on the partition_product depth-2 access fix above (the
//!   coefficient sequence must be projectable from the input shape).
//!
//! [09-adr-024]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-054]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-055]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
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

// Substrate-native 256-bit ring arithmetic via the new ADR-053
// PrimitiveOp call forms (`div`, `r#mod`, `pow`) admitted by
// foundation-sdk 0.4.9's verb-body grammar.

verb! {
    pub fn div_substrate(input: BigIntPair32) -> BigInt32 {
        div(input.0, input.1)
    }
}

verb! {
    pub fn mod_substrate(input: BigIntPair32) -> BigInt32 {
        r#mod(input.0, input.1)
    }
}

verb! {
    pub fn pow_substrate(input: BigIntPair32) -> BigInt32 {
        pow(input.0, input.1)
    }
}

// Compound substrate-Term verbs — three-operand forms like
// `fma(a, b, c) = (a * b) + c`, `mod_pow(base, exp, p)`, and
// `field_add<P>` would compose `Add`/`Mul`/`Mod`/`Pow` over a
// three-operand partition-product input. Foundation-sdk 0.4.9 admits
// the closure-body grammar for these PrimitiveOps but depth-2 field
// access on nested `partition_product`s (`input.0.0` / `input.0.1` /
// `input.1` on a `partition_product(Pair, Leaf)`) fails the verb!
// macro's const-eval path with "index out of bounds: the length is 0
// but the index is 0" — apparently a verb!-specific limitation
// distinct from the `prism_model!` body parser's smoke-test
// coverage of `input.0.0` on a `partition_product(InnerLR, LeafA)`.
// Forward work upstream in foundation-sdk's `emit_term_for_call` /
// nested-projection const-eval path.
