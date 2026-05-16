//! Conformance tests for prism-crypto's substrate-Term verb bodies
//! per [Wiki ADR-024][09-adr-024] + [Wiki ADR-055][09-adr-055] +
//! [Wiki ADR-056][09-adr-056].
//!
//! Each verb is checked structurally — the verb's emitted Term arena
//! terminates in the expected substrate composition (typically an
//! `Application` carrying `PrimitiveOp::Concat` or `AxisInvocation`).
//! Byte-output conformance against canonical reference vectors
//! (FIPS-198 HMAC, etc.) lands once the full HMAC composition assembles.
//!
//! [09-adr-024]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-055]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-056]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(clippy::unwrap_used, clippy::expect_used)]

use uor_foundation::Term;

#[test]
fn merkle_reduce_pair_arena_witness() {
    // Per ADR-056 verb bodies admit `hash(...)` axis invocation and
    // `concat(...)` byte-packing composition. The Merkle reducer's
    // arena terminates in an AxisInvocation node (the outer `hash(...)`).
    let arena = prism_crypto::verbs::merkle_reduce_pair_term_arena();
    assert!(!arena.is_empty(), "verb emits a non-empty Term arena");
    assert!(
        matches!(arena.last(), Some(Term::AxisInvocation { .. })),
        "merkle_reduce_pair terminates in hash(...) → AxisInvocation"
    );
}

#[test]
fn hmac_inner_prep_arena_witness() {
    // Per ADR-056 + ADR-031 HMAC's inner-hash step composes
    // `hash(concat(K_ipad, message))`. The arena terminates in an
    // AxisInvocation node (the outer `hash(...)`).
    let arena = prism_crypto::verbs::hmac_inner_prep_term_arena();
    assert!(!arena.is_empty());
    assert!(matches!(arena.last(), Some(Term::AxisInvocation { .. })));
}

#[test]
fn merkle_and_hmac_verbs_contain_concat_application() {
    // Both verbs internally compose `concat(input.0, input.1)` as a
    // sub-expression. The concat node is a `Term::Application` carrying
    // `PrimitiveOp::Concat` — present in both arenas.
    for arena in [
        prism_crypto::verbs::merkle_reduce_pair_term_arena(),
        prism_crypto::verbs::hmac_inner_prep_term_arena(),
    ] {
        let has_concat = arena.iter().any(|t| {
            matches!(
                t,
                Term::Application {
                    operator: uor_foundation::PrimitiveOp::Concat,
                    ..
                }
            )
        });
        assert!(has_concat, "verb arena includes a Concat Application");
    }
}
