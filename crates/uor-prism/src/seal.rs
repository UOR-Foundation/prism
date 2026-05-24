//! The seal regime: the three sealed Prism-mechanism types.
//!
//! `seal` is the surface for [`Validated`], [`Grounded`], and [`Certified`]
//! — the three types whose construction is gated by the principal data
//! path. They are the structural witness that the wiki's TC-02 sealing
//! property holds: every value carrying these types was produced by
//! `pipeline::run` (or the replay façade, for `Certified`), never by
//! direct construction outside the substrate.
//!
//! In v0.3 of the architecture, the sealing is realized by `pub(crate)`
//! constructors in `uor-foundation::enforcement` (per ADR-011, sealing is
//! enforced by Rust visibility rules, not runtime sentinels). `prism`
//! owns the *namespace* of these types per the wiki's prescription, and
//! re-exports them here. Future evolution of `uor-foundation` may move
//! the constructors into this crate's seal regime; the public API
//! presented to consumers does not change.
//!
//! # See also
//!
//! - [Wiki: 05 Building Block View § Whitebox `prism` seal regime and replay][05-seal]
//! - [Wiki: 08 Concepts § Sealing Discipline](https://github.com/UOR-Foundation/UOR-Framework/wiki/08-Concepts#sealing-discipline)
//! - [Wiki: 09 Architecture Decisions § ADR-011](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
//! - [Wiki: 12 Glossary § Term Definitions](https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary#term-definitions)
//! - [Wiki: Conceptual Model § SD4 UORassembly Enforcement](https://github.com/UOR-Foundation/UOR-Framework/wiki/Conceptual-Model#sd4-uorassembly-enforcement)
//!
//! # Constraints
//!
//! - **TC-02** — sealing of the three Prism-mechanism types via the Rust
//!   type system; no runtime sentinel is involved
//! - **TC-03** — `Grounded<T>` has exactly one constructor: `pipeline::run`
//! - **ADR-011** — sealing is `pub(crate)`, no tokens, no runtime checks
//! - **ADR-019** — the three Prism-mechanism sealed types are
//!   **fixed points** of the typed pipeline endofunctor; sealing is the
//!   architectural statement that the fixed-point inhabitants are
//!   reachable only via the catamorphism's image (`pipeline::run`) or
//!   the anamorphism's witness (replay)
//!
//! # C4 placement
//!
//! Component `seal regime` (Level 3) inside container `prism` (Level 2).
//! Co-located with [`crate::replay`] in the wiki because both are about
//! who is permitted to mint a sealed value: `seal` covers the forward
//! path through `pipeline::run`, `replay` covers the reverse path
//! through `certify_from_trace`.
//!
//! # Behavior
//!
//! ```rust
//! // Given: the three Prism-mechanism types are exposed by `seal`
//! // When:  external code references them by name
//! // Then:  the references type-check at compile time, even though no
//! //        external code can construct them — the constructors are
//! //        `pub(crate)` in the foundation, satisfying TC-02
//! use prism::seal::{Certified, CompileTime, Grounded, Runtime, Validated};
//! use prism::std_types::ConstrainedTypeInput;
//! fn _name<T>() -> &'static str { core::any::type_name::<T>() }
//! let _ = (
//!     _name::<Validated<ConstrainedTypeInput, CompileTime>>(),
//!     _name::<Validated<ConstrainedTypeInput, Runtime>>(),
//!     // Per ADR-060 `Grounded` carries the inline carrier width as its
//!     // 2nd parameter (`Grounded<T, INLINE_BYTES, Tag = T>`); 32 is a
//!     // representative width for this type-name reachability check.
//!     _name::<Grounded<ConstrainedTypeInput, 32>>(),
//!     _name::<Certified<prism::vocabulary::GroundingCertificate>>(),
//! );
//! ```
//!
//! Scenario 3 of the [Runtime View][06-scenario-3] specifies that the
//! Rust toolchain must reject programs that violate sealing or the
//! UORassembly contract. The following compile-fail doctest is the
//! enforcement evidence: it attempts to assign a literal `()` as a
//! validation phase to `Validated`, which fails because `()` does not
//! implement the foundation-sealed `ValidationPhase` trait.
//!
//! ```compile_fail,E0277
//! use prism::seal::Validated;
//! use prism::std_types::ConstrainedTypeInput;
//! // `()` is not a `ValidationPhase` impl — only foundation-supplied
//! // `CompileTime` and `Runtime` markers are. The Rust toolchain
//! // rejects the program at compile time with E0277 (the
//! // `ValidationPhase` bound is unsatisfied) per TC-04 + ADR-011.
//! fn _bad(_: Validated<ConstrainedTypeInput, ()>) {}
//! ```
//!
//! [05-seal]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism-seal-regime-and-replay
//! [06-scenario-3]: https://github.com/UOR-Foundation/UOR-Framework/wiki/06-Runtime-View#scenario-3-compile-time-uorassembly-enforcement

pub use uor_foundation::enforcement::{CompileTime, Runtime, ValidationPhase};
pub use uor_foundation::{Certified, Grounded, Validated};
