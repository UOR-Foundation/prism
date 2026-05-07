//! Trace-replay verification surface.
//!
//! `replay` exposes [`certify_from_trace`]: the function that re-derives a
//! `Certified<GroundingCertificate>` from a `Trace` alone, without
//! invoking the application author's deciders and without invoking any
//! hash function. It is the user-facing surface of TC-05 (replayability)
//! and the only path to a `Certified` value other than `pipeline::run`.
//!
//! The function performs structural validation of the trace (monotonic
//! step indices, contiguous from zero, no zero targets, no empty trace)
//! and re-packages the trace's stored `ContentFingerprint` and
//! `witt_level_bits` into a fresh certificate. The fingerprint is *data
//! carried by the trace*, computed at mint time by the consumer-supplied
//! `Hasher` and passed through unchanged. The verifier never folds bytes.
//!
//! # Round-trip property
//!
//! For every `Grounded<T>` produced by [`crate::pipeline::run`], the
//! certificate emitted by `pipeline::run` is bit-identical to the
//! certificate that [`certify_from_trace`] produces from the
//! corresponding trace.
//!
//! # See also
//!
//! - [Wiki: 05 Building Block View § Whitebox `prism` seal regime and replay][05-seal]
//! - [Wiki: 06 Runtime View § Scenario 2: Trace-Replay Verification][06-scenario-2]
//! - [Wiki: 08 Concepts § Trace Wire Format](https://github.com/UOR-Foundation/UOR-Framework/wiki/08-Concepts#trace-wire-format)
//! - [Wiki: 09 Architecture Decisions § ADR-003](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
//! - [Wiki: Conceptual Model § SD3 Verification](https://github.com/UOR-Foundation/UOR-Framework/wiki/Conceptual-Model#sd3-verification)
//! - [Wiki: Conceptual Model § SD5 Distribute And Run](https://github.com/UOR-Foundation/UOR-Framework/wiki/Conceptual-Model#sd5-distribute-and-run) — same `Verification` process viewed from the user's distribute-and-run perspective; `certify_from_trace` is what runs to consume a `Trace` per the SD5 OPL `Verification yields Certified Output`
//!
//! # Constraints
//!
//! - **TC-05** — verification re-derives the certificate using only the
//!   trace and a chosen hasher selection; never invokes deciders
//! - **TC-06** — verification reaches no application-author service
//! - **QS-03** — local verification: the verifier runs entirely on the
//!   user's hardware, with no network access
//! - **QS-05** — replay equivalence: the round-trip property is
//!   bit-identical
//! - **ADR-003** — verification is local-by-construction
//! - **ADR-008**, **ADR-009** — the trace and certificate wire formats
//!   are normative; this façade does not introduce any wire-format
//!   variation
//! - **ADR-019** — the replay surface is the **anamorphism** the trace
//!   witnesses; replay is the categorical dual of `pipeline::run`'s
//!   catamorphism, and `certify_from_trace` is the unique map back from
//!   the trace into the certificate carrier
//! - **TR-06** — trace format evolution requires version coordination
//!   across producers and verifiers; the foundation's
//!   `TRACE_REPLAY_FORMAT_VERSION` constant is the version-coordination
//!   marker re-exported from [`crate::vocabulary`]
//!
//! # C4 placement
//!
//! Component `replay` (Level 3) inside container `prism` (Level 2). The
//! corresponding user-facing crate is [`prism-verify`][verify], a thin
//! façade that re-exports this function and its certificate output type
//! together with the wire-format types from `uor-foundation`.
//!
//! # Behavior
//!
//! ```rust
//! // Given: an empty trace (the simplest deterministic input) at the
//! // foundation's default capacity
//! // When:  certify_from_trace is invoked on it
//! // Then:  the structural validator rejects it with EmptyTrace
//! use prism::replay::{certify_from_trace, ReplayError, Trace};
//! let trace: Trace = Trace::empty();
//! let result = certify_from_trace(&trace);
//! assert!(matches!(result, Err(ReplayError::EmptyTrace)));
//! ```
//!
//! [05-seal]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism-seal-regime-and-replay
//! [06-scenario-2]: https://github.com/UOR-Foundation/UOR-Framework/wiki/06-Runtime-View#scenario-2-trace-replay-verification
//! [verify]: https://crates.io/crates/uor-prism-verify

pub use uor_foundation::enforcement::replay::certify_from_trace;

// The trace and certificate wire-format types this module operates on.
// They live in [`crate::vocabulary`] as well — vocabulary is the broad
// single-import surface — but they are re-anchored here so consumers
// who write `use prism::replay::{certify_from_trace, Trace};` reach a
// coherent module-local API.
pub use uor_foundation::{ReplayError, Trace, TraceEvent};
