//! `prism` — the Prism runtime crate.
//!
//! This crate is the Rust realization of the **`prism`** container of the
//! Prism system specified by the [UOR-Framework wiki][wiki]. It hosts the
//! singular principal data path (`pipeline::run`), the three sealed
//! Prism-mechanism types (`Validated`, `Grounded`, `Certified`), the replay
//! machinery (`certify_from_trace`), the operation-declaration vocabulary,
//! and the standard type library. The substrate vocabulary lives in
//! [`uor_foundation`], which this crate re-exports as a convenience.
//!
//! The crate is published to crates.io under the package name
//! [`uor-prism`](https://crates.io/crates/uor-prism); the library name is
//! `prism` so that import paths track wiki nomenclature
//! (`use prism::pipeline::run;`).
//!
//! # See also
//!
//! - [Wiki: 01 Introduction and Goals](https://github.com/UOR-Foundation/UOR-Framework/wiki/01-Introduction-and-Goals)
//! - [Wiki: 04 Solution Strategy](https://github.com/UOR-Foundation/UOR-Framework/wiki/04-Solution-Strategy)
//! - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
//! - [Wiki: 09 Architecture Decisions](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
//! - [Wiki: 12 Glossary](https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary)
//!
//! # Constraints
//!
//! This crate is normatively bound by:
//!
//! - **TC-01** — zero-cost runtime; no Prism interpreter layer at execution
//! - **TC-02** — sealing of `Validated`, `Grounded`, `Certified` via the Rust
//!   type system, enforced through `pub(crate)` constructors
//! - **TC-03** — singular principal data path; exactly one constructor for
//!   `Grounded<T>`, reached only through `pipeline::run`
//! - **TC-04** — bilateral compile-time UORassembly enforcement
//! - **TC-05** — replayability without invoking author deciders or hash
//!   functions
//! - **TC-06** — no application-author infrastructure at runtime
//!
//! Substitution axes are restricted to `HostTypes`, `HostBounds`, and
//! `Hasher`.
//!
//! # C4 placement
//!
//! Container `prism` (Level 2) of the Prism system. As implementation
//! lands, modules in this crate will mirror the Level 2 components named in
//! the wiki's [Building Block View § Whitebox `prism`][05-prism]:
//! `pipeline`, `seal regime`, `replay`, `operation declaration`, the
//! standard type library, and re-exports of the `uor-foundation` vocabulary.
//! No top-level module is permitted that does not correspond to a wiki
//! Level 2 component.
//!
//! # Behavior
//!
//! ```rust
//! // Given: the substrate dependency `uor-foundation` is in scope
//! // When:  the prism crate is loaded
//! // Then:  the foundation namespace resolves and the wiki landing URL is
//! //        exposed for downstream tooling
//! use uor_foundation as _;
//! assert_eq!(prism::WIKI, "https://github.com/UOR-Foundation/UOR-Framework/wiki");
//! ```
//!
//! [wiki]: https://github.com/UOR-Foundation/UOR-Framework/wiki
//! [05-prism]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use uor_foundation;

/// Canonical URL of the UOR-Framework wiki, the normative source for the
/// Prism architecture realized by this crate.
///
/// Every public item in `prism` carries a backlink to a wiki page rooted at
/// this URL. Consumers may reference this constant when surfacing the same
/// origin programmatically — for example, in error messages that direct
/// users to the architectural section that defines a violated invariant.
///
/// # See also
///
/// - [Wiki: Home](https://github.com/UOR-Foundation/UOR-Framework/wiki)
///
/// # Constraints
///
/// - **CV-02** — code identifiers appear in monospace without paraphrase;
///   this constant is the single source of truth for the wiki origin
///
/// # Behavior
///
/// ```rust
/// // Given: prism is loaded
/// // When:  the wiki URL constant is read
/// // Then:  it points at the UOR-Framework wiki landing page
/// assert!(prism::WIKI.starts_with("https://"));
/// assert!(prism::WIKI.ends_with("/UOR-Framework/wiki"));
/// ```
pub const WIKI: &str = "https://github.com/UOR-Foundation/UOR-Framework/wiki";

/// Minimum supported Rust version of this crate.
///
/// Pinned to track `uor-foundation`'s effective MSRV so the dependency
/// graph never imposes a tighter requirement on consumers than the
/// substrate itself. Bumping this constant requires bumping the workspace
/// `rust-version` and the `rust-toolchain.toml` channel in lockstep.
///
/// # See also
///
/// - [Wiki: 02 Architecture Constraints](https://github.com/UOR-Foundation/UOR-Framework/wiki/02-Architecture-Constraints)
///
/// # Constraints
///
/// - **TC-04** — bilateral compile-time enforcement assumes a single,
///   declared toolchain version on both sides of the contract
///
/// # Behavior
///
/// ```rust
/// // Given: the MSRV constant
/// // When:  parsed into its semver components
/// // Then:  it is at least 1.83 and uses the major.minor form
/// let parts: Vec<&str> = prism::MSRV.split('.').collect();
/// assert_eq!(parts.len(), 2);
/// let major: u32 = parts[0].parse().expect("major version is numeric");
/// let minor: u32 = parts[1].parse().expect("minor version is numeric");
/// assert!((major, minor) >= (1, 83));
/// ```
pub const MSRV: &str = "1.83";
