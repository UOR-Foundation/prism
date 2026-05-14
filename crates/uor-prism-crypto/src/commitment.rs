//! `CommitmentAxis` declaration and Merkle reference impl.

#![allow(missing_docs)]

use sha2::Digest;
use uor_foundation::enforcement::ShapeViolation;
use uor_foundation::pipeline::AxisExtension;
use uor_foundation_sdk::axis;

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

const LEAF_BYTES: usize = 32;

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

fn fold_pair(left: &[u8], right: &[u8]) -> [u8; LEAF_BYTES] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(left);
    hasher.update(right);
    let result = hasher.finalize();
    let mut out = [0u8; LEAF_BYTES];
    out.copy_from_slice(&result);
    out
}

// Cap of 64 leaves = depth-6 tree. Deeper trees compose at the verb level.
const MAX_LEAVES: usize = 64;

/// Merkle commitment over a SHA-256 binary tree.
///
/// Input layout: a sequence of fixed-width 32-byte leaves. Length must
/// be a non-zero power of two; the commit kernel folds adjacent pairs
/// through SHA-256 until a single 32-byte root remains.
#[derive(Debug, Clone, Copy, Default)]
pub struct MerkleRootCommitment;

impl CommitmentAxis for MerkleRootCommitment {
    const AXIS_ADDRESS: &'static str =
        "https://uor.foundation/axis/CommitmentAxis/MerkleRootSha256";
    const MAX_OUTPUT_BYTES: usize = LEAF_BYTES;

    fn commit(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
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
        if leaf_count > MAX_LEAVES {
            return Err(shape_violation(
                "https://uor.foundation/axis/CommitmentAxis/MerkleRoot/maxLeaves",
            ));
        }
        let mut layer = [[0u8; LEAF_BYTES]; MAX_LEAVES];
        for i in 0..leaf_count {
            layer[i].copy_from_slice(&input[i * LEAF_BYTES..(i + 1) * LEAF_BYTES]);
        }
        let mut len = leaf_count;
        while len > 1 {
            let half = len / 2;
            for i in 0..half {
                let l = layer[2 * i];
                let r = layer[2 * i + 1];
                layer[i] = fold_pair(&l, &r);
            }
            len = half;
        }
        out[..LEAF_BYTES].copy_from_slice(&layer[0]);
        Ok(LEAF_BYTES)
    }
}

axis_extension_impl_for_commitment_axis!(MerkleRootCommitment);
