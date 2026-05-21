//! Surface checks for the Hopf convergence tower re-exported through
//! [`prism::convergence`] per wiki ADR-031 + ADR-058 + ADR-059.
//!
//! Two layers of coverage:
//!
//! 1. **Compile-time path resolution.** The `use` statements plus the
//!    `accepts_*` bound helpers fail to compile if the re-exports in
//!    `prism::convergence` regress against the foundation
//!    `kernel::convergence` trait surface.
//! 2. **Tower-structure constants.** The four convergence levels
//!    (R / C / H / O) and four Hopf fibers (S⁰ / S¹ / S³ / S⁷) carry the
//!    division-algebra dimensions, characteristic identities, and fibration
//!    structure the wiki's *Hopf convergence tower* glossary entry fixes.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism::convergence::{
    hopf_s0, hopf_s1, hopf_s3, hopf_s7, l0_state, l1_memory, l2_agency, l3_self,
    AssociativeSubalgebra, CommutativeSubspace, ConvergenceLevel, ConvergenceResidual, HopfFiber,
    NullAssociativeSubalgebra, NullCommutativeSubspace, NullConvergenceLevel,
    NullConvergenceResidual, NullHopfFiber,
};
use prism::vocabulary::DefaultHostTypes;

// ---- Compile-time bound resolution ----
//
// Each `Null*` baseline must implement its tower trait at
// `DefaultHostTypes`. Declaring the helpers with these bounds resolves the
// impls at definition time; a re-export regression fails the build.

#[allow(dead_code)]
fn accepts_convergence_level<L: ConvergenceLevel<DefaultHostTypes>>() {}
#[allow(dead_code)]
fn accepts_hopf_fiber<F: HopfFiber<DefaultHostTypes>>() {}
#[allow(dead_code)]
fn accepts_convergence_residual<R: ConvergenceResidual<DefaultHostTypes>>() {}
#[allow(dead_code)]
fn accepts_commutative_subspace<S: CommutativeSubspace<DefaultHostTypes>>() {}
#[allow(dead_code)]
fn accepts_associative_subalgebra<A: AssociativeSubalgebra<DefaultHostTypes>>() {}

#[allow(dead_code)]
const NULL_CONVERGENCE_LEVEL_IS_LEVEL: fn() =
    accepts_convergence_level::<NullConvergenceLevel<DefaultHostTypes>>;
#[allow(dead_code)]
const NULL_HOPF_FIBER_IS_FIBER: fn() = accepts_hopf_fiber::<NullHopfFiber<DefaultHostTypes>>;
#[allow(dead_code)]
const NULL_CONVERGENCE_RESIDUAL_IS_RESIDUAL: fn() =
    accepts_convergence_residual::<NullConvergenceResidual<DefaultHostTypes>>;
#[allow(dead_code)]
const NULL_COMMUTATIVE_SUBSPACE_IS_SUBSPACE: fn() =
    accepts_commutative_subspace::<NullCommutativeSubspace<DefaultHostTypes>>;
#[allow(dead_code)]
const NULL_ASSOCIATIVE_SUBALGEBRA_IS_SUBALGEBRA: fn() =
    accepts_associative_subalgebra::<NullAssociativeSubalgebra<DefaultHostTypes>>;

// ---- Tower-structure constants ----

#[test]
fn convergence_levels_carry_division_algebra_dimensions() {
    // R / C / H / O at the four normed-division-algebra dimensions
    // {1, 2, 4, 8} per the wiki *Hopf convergence tower* glossary entry.
    assert_eq!(l0_state::ALGEBRA_DIMENSION, 1);
    assert_eq!(l1_memory::ALGEBRA_DIMENSION, 2);
    assert_eq!(l2_agency::ALGEBRA_DIMENSION, 4);
    assert_eq!(l3_self::ALGEBRA_DIMENSION, 8);

    assert_eq!(l0_state::LEVEL_NAME, "R");
    assert_eq!(l1_memory::LEVEL_NAME, "C");
    assert_eq!(l2_agency::LEVEL_NAME, "H");
    assert_eq!(l3_self::LEVEL_NAME, "O");
}

#[test]
fn convergence_levels_carry_characteristic_identities() {
    // existence → feedback → choice → self-reference, acquired R → O.
    assert_eq!(l0_state::CHARACTERISTIC_IDENTITY, "existence");
    assert_eq!(l1_memory::CHARACTERISTIC_IDENTITY, "feedback");
    assert_eq!(l2_agency::CHARACTERISTIC_IDENTITY, "choice");
    assert_eq!(l3_self::CHARACTERISTIC_IDENTITY, "self-reference");
}

#[test]
fn hopf_fibers_carry_fibration_structure() {
    // The four Hopf fibrations S⁰ / S¹ / S³ / S⁷ of R / C / H / O.
    assert_eq!(hopf_s0::FIBER_SPHERE, "S⁰");
    assert_eq!(hopf_s1::FIBER_SPHERE, "S¹");
    assert_eq!(hopf_s3::FIBER_SPHERE, "S³");
    assert_eq!(hopf_s7::FIBER_SPHERE, "S⁷");

    // Fiber dimensions 0 / 1 / 3 / 7.
    assert_eq!(hopf_s0::FIBER_DIMENSION, 0);
    assert_eq!(hopf_s1::FIBER_DIMENSION, 1);
    assert_eq!(hopf_s3::FIBER_DIMENSION, 3);
    assert_eq!(hopf_s7::FIBER_DIMENSION, 7);
}

#[test]
fn convergence_levels_bind_to_their_hopf_fibers() {
    // Each level's FIBER_TYPE IRI names the matching Hopf fiber.
    assert!(l0_state::FIBER_TYPE.ends_with("hopf_S0"));
    assert!(l1_memory::FIBER_TYPE.ends_with("hopf_S1"));
    assert!(l2_agency::FIBER_TYPE.ends_with("hopf_S3"));
    assert!(l3_self::FIBER_TYPE.ends_with("hopf_S7"));
}
