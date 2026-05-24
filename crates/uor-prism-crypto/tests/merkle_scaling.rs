//! Scaling V&V for the streaming Merkle commitment kernel — the
//! falsification suite for the claim that `MerkleRoot<H, LEAF_BYTES>`
//! admits **any** leaf count with no ceiling (AGENTS.md § 11.10
//! category 3).
//!
//! The kernel previously buffered every leaf in a fixed
//! `[[u8; MAX_LEAF_WIDTH]; MAX_MERKLE_LEAVES]` array, capping leaves at
//! 64. It now streams leaves left-to-right, combining equal-height
//! subtrees on an `O(log N)` stack of `usize::BITS` slots — enough for
//! any leaf count a `usize`-indexed slice can hold. This suite drives
//! leaf counts two orders of magnitude past the retired 64-leaf cap and
//! checks the streaming root against an independent reference.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism_crypto::{CommitmentAxis, HashAxis, MerkleRootCommitment, Sha256Hasher};

const LEAF: usize = 32;

/// Independent reference for the Merkle root of `n` **identical** 32-byte
/// leaves `leaf`: a perfect binary tree over identical leaves collapses
/// to `r_0 = leaf`, `r_{k+1} = H(r_k ‖ r_k)`, so after `log2(n)` doubling
/// rounds the root is `r_{log2 n}`. This recomputation shares no code
/// with the streaming kernel under test.
fn reference_root_of_identical_leaves(leaf: [u8; LEAF], n: usize) -> [u8; LEAF] {
    assert!(n.is_power_of_two());
    let mut r = leaf;
    let mut count = n;
    while count > 1 {
        let mut pair = [0u8; 2 * LEAF];
        pair[..LEAF].copy_from_slice(&r);
        pair[LEAF..].copy_from_slice(&r);
        let mut digest = [0u8; LEAF];
        Sha256Hasher::hash(&pair, &mut digest).expect("hash");
        r = digest;
        count /= 2;
    }
    r
}

#[test]
fn merkle_root_matches_reference_far_past_the_retired_cap() {
    // 128, 1024, 4096 leaves — 2×, 16×, 64× the retired 64-leaf cap.
    let leaf = [0xABu8; LEAF];
    for &n in &[128usize, 1024, 4096] {
        let mut leaves = vec![0u8; n * LEAF];
        for i in 0..n {
            leaves[i * LEAF..(i + 1) * LEAF].copy_from_slice(&leaf);
        }
        let mut out = [0u8; LEAF];
        let written = MerkleRootCommitment::commit(&leaves, &mut out).expect("commit must admit");
        assert_eq!(written, LEAF, "{n}-leaf commit writes a full leaf width");
        assert_eq!(
            out,
            reference_root_of_identical_leaves(leaf, n),
            "{n}-leaf streaming root must match the independent reference",
        );
    }
}

#[test]
fn merkle_root_is_deterministic_and_leaf_sensitive_at_scale() {
    // 2048 distinct leaves: leaf i = [i mod 256; 32]. The root is stable
    // across calls and changes when any single leaf changes — the kernel
    // genuinely folds all leaves, it does not silently truncate at a cap.
    let n = 2048usize;
    let mut leaves = vec![0u8; n * LEAF];
    for i in 0..n {
        #[allow(clippy::cast_possible_truncation)]
        let b = (i % 256) as u8;
        leaves[i * LEAF..(i + 1) * LEAF].fill(b);
    }

    let mut root_a = [0u8; LEAF];
    let mut root_b = [0u8; LEAF];
    MerkleRootCommitment::commit(&leaves, &mut root_a).expect("commit a");
    MerkleRootCommitment::commit(&leaves, &mut root_b).expect("commit b");
    assert_eq!(root_a, root_b, "root must be deterministic at 2048 leaves");

    // Flip one byte in the last leaf — the root must change, proving the
    // streaming fold reaches the final leaf (no truncation at 64).
    leaves[n * LEAF - 1] ^= 0x01;
    let mut root_c = [0u8; LEAF];
    MerkleRootCommitment::commit(&leaves, &mut root_c).expect("commit c");
    assert_ne!(
        root_a, root_c,
        "changing the last of 2048 leaves must change the root",
    );
}

#[test]
fn merkle_root_two_leaves_still_matches_direct_hash() {
    // Regression guard: the streaming form must agree with the bottom-up
    // form on the smallest tree — root of [l0, l1] = H(l0 ‖ l1).
    let mut leaves = [0u8; 2 * LEAF];
    leaves[..LEAF].fill(0x11);
    leaves[LEAF..].fill(0x22);
    let mut out = [0u8; LEAF];
    MerkleRootCommitment::commit(&leaves, &mut out).expect("commit");

    let mut expected = [0u8; LEAF];
    Sha256Hasher::hash(&leaves, &mut expected).expect("hash");
    assert_eq!(out, expected);
}
