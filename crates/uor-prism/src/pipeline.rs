//! The principal data path: admission, validation, and grounding.
//!
//! `pipeline` realizes the wiki's
//! [Building Block View § Whitebox `prism` pipeline][05-pipeline]
//! and the runtime narrative in
//! [Runtime View § Scenario 1: Principal Data Path Execution][06-scenario-1].
//! Its single sanctioned entry point is [`run`]: it consumes a
//! `Validated<CompileUnit, Phase>` produced by the foundation's builders
//! and emits a `(Grounded<T>, Trace)` pair simultaneously, both
//! constructed from the same intermediate values so that replay
//! equivalence (TC-05) holds by construction.
//!
//! Application authors implement [`ConstrainedTypeShape`] to declare a
//! constrained type; the closed set of [`ConstraintRef`] variants is the
//! vocabulary they assemble. `pipeline::run`'s type parameters are
//! exactly the three substitution axes identified in ADR-007: the
//! constrained type `T` (carrying the application's `HostBounds` const
//! generics), the validation phase `P`, and the substrate hasher `H`.
//!
//! # See also
//!
//! - [Wiki: 05 Building Block View § Whitebox `prism` pipeline — staged transitions][05-pipeline]
//! - [Wiki: 06 Runtime View § Scenario 1: Principal Data Path Execution][06-scenario-1]
//! - [Wiki: 07 Deployment View § Quality Properties of the Deployment](https://github.com/UOR-Foundation/UOR-Framework/wiki/07-Deployment-View#quality-properties-of-the-deployment)
//! - [Wiki: 08 Concepts § Hashing Substrate Contract](https://github.com/UOR-Foundation/UOR-Framework/wiki/08-Concepts#hashing-substrate-contract)
//! - [Wiki: 09 Architecture Decisions § ADR-012](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
//!
//! # Constraints
//!
//! - **TC-01** — pipeline execution is a sequence of monomorphized,
//!   non-dispatched calls; no Prism interpreter layer is interposed
//! - **TC-03** — `run` is the singular constructor of `Grounded<T>`;
//!   no alternative path exists
//! - **QS-01** — release builds with `opt-level=3`, `lto=true`,
//!   `codegen-units=1` reduce execution cost to the substrate hasher
//!   invocation plus the primitive operations
//! - **ADR-012** — the pipeline lives in `prism`, not `uor-foundation`,
//!   so alternative runtimes can consume the same foundation vocabulary
//!
//! # C4 placement
//!
//! Component `pipeline` (Level 3) inside container `prism` (Level 2).
//! Its boundary properties are: validated input on the left, sealed
//! `Grounded<T>` plus `Trace` on the right; everything between is
//! foundation-internal staged transitions.
//!
//! # Behavior
//!
//! ```rust
//! // Given: the pipeline module surface
//! // When:  consumers reference the entry-point and admission types
//! // Then:  every name resolves at compile time
//! use prism::pipeline::{run as _, ConstrainedTypeShape as _, ConstraintRef};
//! // ConstraintRef is a closed enum; matching is exhaustive at compile time.
//! fn _is_total_admission_check(c: &ConstraintRef) -> bool {
//!     matches!(
//!         c,
//!         ConstraintRef::Residue { .. }
//!             | ConstraintRef::Hamming { .. }
//!             | ConstraintRef::Depth { .. }
//!             | ConstraintRef::Carry { .. }
//!             | ConstraintRef::Site { .. }
//!             | ConstraintRef::Affine { .. }
//!             | ConstraintRef::SatClauses { .. }
//!             | ConstraintRef::Bound { .. }
//!             | ConstraintRef::Conjunction { .. }
//!     )
//! }
//! ```
//!
//! [05-pipeline]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism-pipeline--staged-transitions
//! [06-scenario-1]: https://github.com/UOR-Foundation/UOR-Framework/wiki/06-Runtime-View#scenario-1-principal-data-path-execution

pub use uor_foundation::pipeline::{
    run, validate_compile_unit_const, validate_constrained_type, validate_constrained_type_const,
    ConstrainedTypeShape, ConstraintRef, FragmentKind, StageOutcome,
};
pub use uor_foundation::ViolationKind;
pub use uor_foundation::{PipelineFailure, ShapeViolation};

// `TimingPolicy` is the foundation-sealed trait the application author
// references to declare timing budgets that participate in preflight
// and runtime timing checks of the principal data path. It is part of
// the admission contract surfaced by [`run`] indirectly, through
// `Validated<CompileUnit, _>`'s thermodynamic-budget plumbing.
pub use uor_foundation::enforcement::TimingPolicy;

// Free functions that drive the per-stage admission machinery. They are
// surfaced because `prism::pipeline` is the wiki-defined home of the
// principal data path (ADR-012); having them here means consumers can
// reach the const-evaluable validators without depending on
// `uor-foundation`'s `pipeline` module path directly.
pub use uor_foundation::pipeline::{
    fragment_classify, preflight_budget_solvency, preflight_dispatch_coverage,
    preflight_feasibility, preflight_package_coherence,
};

// `WITT_MAX_BITS` is the normative upper bound on Witt-level bit width
// honored by `preflight_budget_solvency`. Surfacing it here keeps the
// pipeline contract self-contained.
pub use uor_foundation::pipeline::WITT_MAX_BITS;
