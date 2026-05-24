//! `CommitmentAxis` declaration, parametric Merkle reference impl, and
//! shape carriers.

#![allow(missing_docs)]

use core::marker::PhantomData;

use uor_foundation::enforcement::{GroundedShape, ShapeViolation};
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef, IntoBindingValue, TermValue};
use uor_foundation_sdk::axis;

use crate::hash::{HashAxis, Sha256Hasher};

axis! {
    /// Wiki ADR-031 commitment schemes (Merkle, Pedersen, KZG).
    pub trait CommitmentAxis: AxisExtension {
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/CommitmentAxis";
        const MAX_OUTPUT_BYTES: usize = 96;
        /// Commit to `input` — emits the commitment bytes into `out`.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on malformed input.
        fn commit(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

const SHA256_BYTES: usize = 32;

fn shape_violation(constraint: &'static str) -> ShapeViolation {
    ShapeViolation {
        shape_iri: "https://uor.foundation/axis/CommitmentAxis/MerkleRoot",
        constraint_iri: constraint,
        property_iri: "https://uor.foundation/axis/inputBytes",
        expected_range: "https://uor.foundation/axis/MerkleLeafSequence",
        min_count: 0,
        max_count: 0,
        kind: uor_foundation::ViolationKind::ValueCheck,
    }
}

/// Depth of the streaming-Merkle subtree stack: `usize::BITS` slots.
/// A binary tree with `N` leaves has height `⌈log2 N⌉`, and any leaf
/// sequence a `usize`-indexed slice can address has `N ≤ usize::MAX`, so
/// `usize::BITS` stack slots accommodate the **maximum possible** tree —
/// there is no arbitrary leaf-count ceiling (§ 11.10). The commit kernel
/// reads leaves directly from `input` and combines equal-height subtrees
/// on this `O(log N)` stack, so leaf count scales arbitrarily.
const MERKLE_STACK_DEPTH: usize = usize::BITS as usize;

/// Parametric Merkle-root commitment over **any** `HashAxis` impl
/// `H` with `H::MAX_OUTPUT_BYTES = LEAF_BYTES`.
///
/// `LEAF_BYTES` is the leaf width (and the root width — Merkle's input
/// and output share the digest's output size). The default,
/// [`MerkleRootCommitment`], uses SHA-256 (32-byte leaves and root).
///
/// Per ADR-031 a standard-library commitment composes other
/// standard-library axes (`HashAxis` here); this is the
/// canonical-reference example of axis composition the wiki commits
/// to. Two `MerkleRoot<H>` instantiations with structurally-identical
/// `H` content-address identically per ADR-017.
#[derive(Debug, Clone, Copy)]
pub struct MerkleRoot<H: HashAxis, const LEAF_BYTES: usize = SHA256_BYTES>(PhantomData<H>);

impl<H: HashAxis, const LEAF_BYTES: usize> Default for MerkleRoot<H, LEAF_BYTES> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<H: HashAxis, const LEAF_BYTES: usize> CommitmentAxis for MerkleRoot<H, LEAF_BYTES> {
    const AXIS_ADDRESS: &'static str =
        "https://uor.foundation/axis/CommitmentAxis/MerkleRootParametric";
    const MAX_OUTPUT_BYTES: usize = LEAF_BYTES;

    fn commit(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if LEAF_BYTES == 0 {
            return Err(shape_violation(
                "https://uor.foundation/axis/CommitmentAxis/MerkleRoot/leafBytesNonZero",
            ));
        }
        if input.is_empty() || input.len() % LEAF_BYTES != 0 {
            return Err(shape_violation(
                "https://uor.foundation/axis/CommitmentAxis/MerkleRoot/leafAlignment",
            ));
        }
        let leaf_count = input.len() / LEAF_BYTES;
        if !leaf_count.is_power_of_two() {
            return Err(shape_violation(
                "https://uor.foundation/axis/CommitmentAxis/MerkleRoot/powerOfTwoLeaves",
            ));
        }
        if out.len() < LEAF_BYTES {
            return Err(shape_violation(
                "https://uor.foundation/axis/CommitmentAxis/MerkleRoot/outputBuffer",
            ));
        }
        // Streaming Merkle: read leaves left-to-right directly from
        // `input` and combine equal-height subtrees on an `O(log N)`
        // stack. All scratch is sized by the const-generic `LEAF_BYTES`
        // (no fixed leaf-width cap) and the stack has `usize::BITS` slots
        // (no fixed leaf-count cap — see `MERKLE_STACK_DEPTH`). This is
        // identical, leaf-for-leaf, to the bottom-up pairing
        // `hash(node[2i] || node[2i+1])` for the power-of-two leaf counts
        // this kernel admits.
        let mut roots = [[0u8; LEAF_BYTES]; MERKLE_STACK_DEPTH];
        let mut levels = [0u32; MERKLE_STACK_DEPTH];
        let mut top = 0usize;
        for i in 0..leaf_count {
            let mut cur = [0u8; LEAF_BYTES];
            cur.copy_from_slice(&input[i * LEAF_BYTES..(i + 1) * LEAF_BYTES]);
            let mut cur_level = 0u32;
            // Fold the current subtree with any same-height neighbour on
            // top of the stack; the popped entry is the left sibling.
            while top > 0 && levels[top - 1] == cur_level {
                top -= 1;
                let mut pair = [[0u8; LEAF_BYTES]; 2];
                pair[0] = roots[top];
                pair[1] = cur;
                let mut digest = [0u8; LEAF_BYTES];
                H::hash(pair.as_flattened(), &mut digest[..])?;
                cur = digest;
                cur_level += 1;
            }
            roots[top] = cur;
            levels[top] = cur_level;
            top += 1;
        }
        // Power-of-two leaf count ⇒ the stack collapses to a single root.
        out[..LEAF_BYTES].copy_from_slice(&roots[0][..LEAF_BYTES]);
        Ok(LEAF_BYTES)
    }
}

// ADR-052 generic-form companion.
axis_extension_impl_for_commitment_axis!(
    @generic MerkleRoot<H, LEAF_BYTES>,
    [H: HashAxis, const LEAF_BYTES: usize]
);

/// SHA-256 Merkle root — the canonical default per ADR-031.
pub type MerkleRootCommitment = MerkleRoot<Sha256Hasher, SHA256_BYTES>;

// ---- MerkleProofShape: ConstrainedTypeShape carrier ----

/// Parametric ConstrainedTypeShape for a Merkle-inclusion proof.
///
/// Carries `MAX_DEPTH` sibling-digests of `LEAF_BYTES` each plus a
/// leaf-index — `(MAX_DEPTH * LEAF_BYTES + 8)` bytes total (the +8
/// for a u64 leaf-index). Per ADR-031's `MerkleProof<MaxDepth>` shape
/// commitment.
#[derive(Debug, Clone, Copy)]
pub struct MerkleProofShape<const MAX_DEPTH: usize, const LEAF_BYTES: usize = SHA256_BYTES>;

impl<const MAX_DEPTH: usize, const LEAF_BYTES: usize> Default
    for MerkleProofShape<MAX_DEPTH, LEAF_BYTES>
{
    fn default() -> Self {
        Self
    }
}

impl<const MAX_DEPTH: usize, const LEAF_BYTES: usize> ConstrainedTypeShape
    for MerkleProofShape<MAX_DEPTH, LEAF_BYTES>
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = MAX_DEPTH * LEAF_BYTES + 8;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow((MAX_DEPTH * LEAF_BYTES + 8) as u32);
}

impl<const MAX_DEPTH: usize, const LEAF_BYTES: usize> uor_foundation::pipeline::__sdk_seal::Sealed
    for MerkleProofShape<MAX_DEPTH, LEAF_BYTES>
{
}
impl<const MAX_DEPTH: usize, const LEAF_BYTES: usize> GroundedShape
    for MerkleProofShape<MAX_DEPTH, LEAF_BYTES>
{
}
impl<'a, const MAX_DEPTH: usize, const LEAF_BYTES: usize> IntoBindingValue<'a>
    for MerkleProofShape<MAX_DEPTH, LEAF_BYTES>
{
    fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
        TermValue::empty()
    }
}
