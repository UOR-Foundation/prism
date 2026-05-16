//! Layer-3 substrate-Term verb bodies per [Wiki ADR-024][09-adr-024] +
//! [Wiki ADR-031][09-adr-031] + [Wiki ADR-055][09-adr-055] (universal
//! substrate-Term verb body discipline, supersedes ADR-054 RA2).
//!
//! Per ADR-024 a Layer-3 implementation contributes both axes
//! (substrate-extension vocabularies via `axis!`) AND verbs (named,
//! reusable compositions of prism operators applied to substrate
//! primitives via `verb!`). Per ADR-055 every `AxisExtension` impl
//! (standard-library AND application-author custom) carries a
//! substrate-Term verb body via the foundation-declared
//! `SubstrateTermBody` supertrait.
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
//!   input at W256 per ADR-055 + ADR-055. Each verb body is one
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
// per ADR-055 of the corresponding `BigIntAxis` kernel bodies.
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
// realizations per ADR-055 of `Gf2NumericAxisN<32>::{add, mul}`.
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

// ---- Three-operand verbs (parametric-modulus form).
//
// Note: depth-2 partition-product field access works in
// foundation-sdk 0.4.10's verb! macro when the leaf factor is a
// hand-written ConstrainedTypeShape without explicit
// PartitionProductFields impl (the smoke-test pattern at
// uor-foundation-sdk/tests/smoke.rs `verb_depth2_pos00` over
// PosOuter = (InnerLR, LeafA)). Replicating that pattern with
// BigIntShape<N> as the leaf factor (which is parametric over a
// const generic byte-width parameter) triggers a verb!-macro
// const-eval "index out of bounds" failure not reproduced by the
// hand-written non-generic LeafA pattern; the failure mode
// appears specific to const-generic leaf factors. The
// secp256k1-pinned verbs below (depth-1 access + wide literal
// embedding) realize the same semantics with the modulus baked
// in as a `literal_bytes` const, avoiding the depth-2 path.

// ---- Wide-literal P verbs (closed by foundation-sdk 0.4.10
// Dependency 2 — `literal_bytes(<bytes>, <level>)` admission for
// W128+ literal embedding).

/// Secp256k1 base-field prime as a 32-byte big-endian literal:
/// `p = 2^256 - 2^32 - 977`.
pub const SECP256K1_P_BYTES: &[u8] = &[
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xff, 0xff, 0xfc, 0x2f,
];

/// W256 Witt-level marker for the secp256k1 P_LITERAL embedding.
pub const W256_LEVEL: uor_foundation::WittLevel = uor_foundation::WittLevel::new(256);

// secp256k1-pinned field arithmetic — the parametric `field_*`
// verbs above with the W256 P literal baked into the verb body via
// `literal_bytes`. These are the substrate-Term realizations of
// `PrimeFieldNumericSecp256k1::{add, sub, mul}` per ADR-054 (4) +
// ADR-055.

verb! {
    pub fn secp256k1_field_add(input: BigIntPair32) -> BigInt32 {
        r#mod(add(input.0, input.1), literal_bytes(SECP256K1_P_BYTES, W256_LEVEL))
    }
}

verb! {
    pub fn secp256k1_field_sub(input: BigIntPair32) -> BigInt32 {
        r#mod(sub(input.0, input.1), literal_bytes(SECP256K1_P_BYTES, W256_LEVEL))
    }
}

verb! {
    pub fn secp256k1_field_mul(input: BigIntPair32) -> BigInt32 {
        r#mod(mul(input.0, input.1), literal_bytes(SECP256K1_P_BYTES, W256_LEVEL))
    }
}

// ---- Compound-arithmetic verbs (ADR-056 + 0.4.10 grammar admissions).

// `polyeval_horner_2(x, c0, c1) = c0 + x * c1` — Horner-method
// evaluation of a degree-1 polynomial, the smallest compound form
// the canonical roster's `polyeval` / `horner` family generalizes.
// Composes substrate `add` and `mul` over a single pair input
// (x and c1 concatenated as the partition_product; c0 is a fixed
// literal at the verb-body level). Per ADR-056 verb bodies admit
// the full PrimitiveOp surface unconditionally; this verb's
// composition path is fold-fused into the catamorphism's evaluation
// per ADR-054.
verb! {
    pub fn polyeval_linear(input: BigIntPair32) -> BigInt32 {
        add(input.0, mul(input.1, literal_u64(1, W256_LEVEL)))
    }
}

// Depth-2 partition-product field access experiments deferred —
// reproduced macro quirk with both const-generic `BigIntShape<N>`
// and hand-rolled non-generic 32-byte leaves; smoke-tested LeafA
// pattern works in foundation-sdk 0.4.10 but the trigger for the
// "PartitionProductFields not implemented" trait-bound check at
// the verb! macro site eludes local reproduction. Forward work
// upstream in `emit_term_for_field`'s depth-2 projection chain.
// The `fma(a, b, c)`, `mod_pow(base, exp, m)`, and parametric-prime
// `field_*<P>(a, b, p)` verbs the wiki names per ADR-031 land
// once the parity gap closes.
