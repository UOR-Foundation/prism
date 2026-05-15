//! `CommitmentAxis` declaration, parametric Merkle reference impl, and
//! shape carriers.

#![allow(missing_docs)]

use core::marker::PhantomData;

use uor_foundation::enforcement::{GroundedShape, ShapeViolation};
use uor_foundation::pipeline::{
    AxisExtension, ConstrainedTypeShape, ConstraintRef, IntoBindingValue,
};
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

/// Maximum leaf count any [`MerkleRoot`] instantiation supports.
/// Depth-6 binary tree; deeper Merkle commitments compose at the verb
/// level (per ADR-024) rather than baking into the axis kernel.
pub const MAX_MERKLE_LEAVES: usize = 64;

/// Maximum leaf byte-width — the largest `HashAxis::MAX_OUTPUT_BYTES`
/// among standard-library hash impls (64 bytes for SHA-512).
const MAX_LEAF_WIDTH: usize = 64;

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
        if leaf_count > MAX_MERKLE_LEAVES {
            return Err(shape_violation(
                "https://uor.foundation/axis/CommitmentAxis/MerkleRoot/maxLeaves",
            ));
        }
        // Use a fixed-size working buffer sized for the maximum
        // supported leaf width across HashAxis impls (64 bytes for
        // SHA-512). Per-instantiation LEAF_BYTES bounds the actually
        // used slice.
        if LEAF_BYTES > MAX_LEAF_WIDTH {
            return Err(shape_violation(
                "https://uor.foundation/axis/CommitmentAxis/MerkleRoot/leafBytesInRange",
            ));
        }
        let mut layer = [[0u8; MAX_LEAF_WIDTH]; MAX_MERKLE_LEAVES];
        for i in 0..leaf_count {
            layer[i][..LEAF_BYTES].copy_from_slice(&input[i * LEAF_BYTES..(i + 1) * LEAF_BYTES]);
        }
        let mut pair_buf = [0u8; 2 * MAX_LEAF_WIDTH];
        let mut digest_buf = [0u8; MAX_LEAF_WIDTH];
        let mut len = leaf_count;
        while len > 1 {
            let half = len / 2;
            for i in 0..half {
                pair_buf[..LEAF_BYTES].copy_from_slice(&layer[2 * i][..LEAF_BYTES]);
                pair_buf[LEAF_BYTES..2 * LEAF_BYTES]
                    .copy_from_slice(&layer[2 * i + 1][..LEAF_BYTES]);
                H::hash(&pair_buf[..2 * LEAF_BYTES], &mut digest_buf[..LEAF_BYTES])?;
                layer[i][..LEAF_BYTES].copy_from_slice(&digest_buf[..LEAF_BYTES]);
            }
            len = half;
        }
        out[..LEAF_BYTES].copy_from_slice(&layer[0][..LEAF_BYTES]);
        Ok(LEAF_BYTES)
    }
}

impl<H: HashAxis, const LEAF_BYTES: usize> AxisExtension for MerkleRoot<H, LEAF_BYTES> {
    const AXIS_ADDRESS: &'static str = <Self as CommitmentAxis>::AXIS_ADDRESS;
    const MAX_OUTPUT_BYTES: usize = <Self as CommitmentAxis>::MAX_OUTPUT_BYTES;

    fn dispatch_kernel(
        kernel_id: u32,
        input: &[u8],
        out: &mut [u8],
    ) -> Result<usize, ShapeViolation> {
        match kernel_id {
            KERNEL_COMMIT => <Self as CommitmentAxis>::commit(input, out),
            _ => Err(ShapeViolation {
                shape_iri: "https://uor.foundation/axis/AxisExtensionShape",
                constraint_iri: "https://uor.foundation/axis/AxisExtensionShape/kernelId",
                property_iri: "https://uor.foundation/axis/kernelId",
                expected_range: "https://uor.foundation/axis/RecognisedKernelId",
                min_count: 0,
                max_count: 0,
                kind: uor_foundation::ViolationKind::ValueCheck,
            }),
        }
    }
}

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
impl<const MAX_DEPTH: usize, const LEAF_BYTES: usize> IntoBindingValue
    for MerkleProofShape<MAX_DEPTH, LEAF_BYTES>
{
    const MAX_BYTES: usize = MAX_DEPTH * LEAF_BYTES + 8;

    fn into_binding_bytes(&self, _out: &mut [u8]) -> Result<usize, ShapeViolation> {
        Ok(0)
    }
}
