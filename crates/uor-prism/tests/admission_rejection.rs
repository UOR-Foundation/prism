//! Failure-mode coverage for the principal data path's admission stage.
//!
//! Per the wiki's
//! [Runtime View § Scenario 1: Principal Data Path Execution][06-scenario-1],
//! a `CompileUnitBuilder` whose required fields are missing must produce
//! a typed `ShapeViolation` with `ViolationKind::Missing` and the exact
//! ontology property IRI that names the absent field. These tests pin
//! the property IRIs so any drift between `uor-foundation`'s ontology
//! and the rejection contract is caught here.
//!
//! [06-scenario-1]: https://github.com/UOR-Foundation/UOR-Framework/wiki/06-Runtime-View#scenario-1-principal-data-path-execution

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use prism::operation::Term;
use prism::pipeline::{validate_compile_unit_const, ViolationKind};
use prism::std_types::ConstrainedTypeInput;
use prism::vocabulary::{CompileUnitBuilder, VerificationDomain, WittLevel};

const CARRIER: usize = uor_foundation::pipeline::carrier_inline_bytes::<common::TestHostBounds>();

// ADR-060: `TermValue` now carries a `Stream(&dyn ChunkSource)` variant
// that is not `Sync`, so a `&[Term]` can no longer live in a `static`
// (which requires `Sync`). These literal arenas only ever construct the
// `Inline` variant; promoting them to `const` keeps the same `'static`
// slice semantics without the `Sync` obligation.
const SENTINEL_TERMS: &[Term<'static, CARRIER>] = &[Term::Literal {
    value: prism::operation::TermValue::from_u64_be(1, 1),
    level: WittLevel::W8,
}];
static SENTINEL_DOMAINS: &[VerificationDomain] = &[VerificationDomain::Enumerative];

#[test]
fn missing_root_term_is_typed_missing() {
    // Given: a CompileUnitBuilder with every required field except root_term.
    // No `.root_term(..)` call means the width can't be inferred from the
    // arena slice, so it's pinned explicitly via the carrier const.
    let builder = CompileUnitBuilder::<'_, CARRIER>::new()
        .witt_level_ceiling(WittLevel::W8)
        .thermodynamic_budget(100)
        .target_domains(SENTINEL_DOMAINS)
        .result_type::<ConstrainedTypeInput>();

    // When: the const validator runs.
    let err: prism::pipeline::ShapeViolation =
        validate_compile_unit_const(&builder).expect_err("missing root_term must be rejected");

    // Then: the typed error names the missing property by its ontology IRI.
    assert_eq!(err.kind, ViolationKind::Missing);
    assert_eq!(
        err.property_iri,
        "https://uor.foundation/reduction/rootTerm"
    );
}

#[test]
fn missing_thermodynamic_budget_is_typed_missing() {
    let builder = CompileUnitBuilder::new()
        .root_term(SENTINEL_TERMS)
        .witt_level_ceiling(WittLevel::W8)
        .target_domains(SENTINEL_DOMAINS)
        .result_type::<ConstrainedTypeInput>();
    let err = validate_compile_unit_const(&builder)
        .expect_err("missing thermodynamic_budget must be rejected");
    assert_eq!(err.kind, ViolationKind::Missing);
    assert_eq!(
        err.property_iri,
        "https://uor.foundation/reduction/thermodynamicBudget"
    );
}

#[test]
fn missing_result_type_is_typed_missing() {
    let builder = CompileUnitBuilder::new()
        .root_term(SENTINEL_TERMS)
        .witt_level_ceiling(WittLevel::W8)
        .thermodynamic_budget(100)
        .target_domains(SENTINEL_DOMAINS);
    let err =
        validate_compile_unit_const(&builder).expect_err("missing result_type must be rejected");
    assert_eq!(err.kind, ViolationKind::Missing);
    assert_eq!(
        err.property_iri,
        "https://uor.foundation/reduction/resultType"
    );
}
