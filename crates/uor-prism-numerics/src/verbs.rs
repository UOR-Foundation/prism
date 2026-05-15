//! Layer-3 verbs published by prism-numerics per [Wiki ADR-024][09-adr-024] +
//! [Wiki ADR-031][09-adr-031].
//!
//! Per ADR-024 a Layer-3 implementation contributes both axes
//! (substrate-extension vocabularies via `axis!`) AND verbs (named,
//! reusable compositions of prism operators applied to substrate
//! primitives via `verb!`). The verbs in this module are
//! demonstration-grade compositions of `PrimitiveOp` evaluated at the
//! substrate's full Witt-tower width per ADR-050; richer numerics
//! verbs the wiki names (`modexp_p`, `polyeval`, `gcd`,
//! `ext_euclidean`, `horner`, `newton_step`, `fma`, `field_add<P>`,
//! `field_sub<P>`, `field_mul<P>`, `field_inv<P>`) are operational
//! follow-on additions per ADR-031's "specific sub-crates'
//! versioning, methods, and impls are operational policy" carve-out:
//! the architecture admits them; each individual impl is a separate
//! `verb!` declaration over `PrimitiveOp::{Add, Sub, Mul, Div, Mod,
//! Pow}` plus the partition-product composition machinery (ADR-033).
//!
//! Per ADR-024 the verb-closure check is performed at the macro
//! expansion: cycle-freeness through non-`recurse` operators is a
//! compile-time commitment, not a runtime guard.
//!
//! The `succ_twice` verb below is an architectural witness: it
//! exercises the closure-body grammar (G1 application of substrate
//! primitives plus nested composition), the macro's
//! `inline_verb_fragment` emission, the verb-closure check, and the
//! re-export path through the prism façade. Downstream applications
//! adding verbs from the canonical roster follow the same pattern.
//!
//! [09-adr-024]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(missing_docs)]

use uor_foundation::enforcement::ConstrainedTypeInput;
use uor_foundation_sdk::verb;

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
