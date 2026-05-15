//! Conformance vectors for prism-crypto's `HashAxis` and
//! `CommitmentAxis` impls, per the ADR-031 conformance-test commitment.
//!
//! Each impl is checked against canonical input-output pairs from the
//! authoritative specification of the primitive it realizes:
//!
//! - **SHA-256** — FIPS-180-4 §6.2, Appendix B.1 vectors
//! - **SHA-512** — FIPS-180-4 §6.4, Appendix C.1 vectors
//! - **SHA3-256** — FIPS-202 §A test vectors (NIST CSRC)
//! - **Keccak-256** — original (pre-FIPS) Keccak Test Vectors
//! - **BLAKE3** — the canonical BLAKE3 specification test vectors

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_possible_truncation
)]

use prism_crypto::CommitmentAxis;
use prism_crypto::{
    Blake3Hasher, Digest, HashAxis, Keccak256Hasher, MerkleProofShape, MerkleRoot,
    MerkleRootCommitment, PublicKey, Sha256Hasher, Sha3_256Hasher, Sha512Hasher, Signature,
};
use uor_foundation::pipeline::ConstrainedTypeShape;

fn hex_decode(s: &str) -> Vec<u8> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    let mut i = 0;
    while i + 1 < bytes.len() {
        let hi = hex_digit(bytes[i]);
        let lo = hex_digit(bytes[i + 1]);
        out.push((hi << 4) | lo);
        i += 2;
    }
    out
}

fn hex_digit(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => panic!("invalid hex digit"),
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        let hi = b >> 4;
        let lo = b & 0xf;
        out.push(hex_char(hi));
        out.push(hex_char(lo));
    }
    out
}

fn hex_char(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + n - 10) as char,
        _ => unreachable!(),
    }
}

// ---- SHA-256: FIPS-180-4 vectors ----

#[test]
fn sha256_empty_string() {
    // Empty-string vector from FIPS-180-4 §B.1 / RFC 6234 §8.5.
    let mut out = [0u8; 32];
    let n = Sha256Hasher::hash(b"", &mut out).expect("hash succeeds");
    assert_eq!(n, 32);
    assert_eq!(
        hex_encode(&out),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn sha256_abc() {
    // Canonical "abc" vector from FIPS-180-4 §B.1.
    let mut out = [0u8; 32];
    Sha256Hasher::hash(b"abc", &mut out).expect("hash succeeds");
    assert_eq!(
        hex_encode(&out),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn sha256_two_block() {
    // FIPS-180-4 §B.2 — "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".
    let mut out = [0u8; 32];
    Sha256Hasher::hash(
        b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        &mut out,
    )
    .expect("hash succeeds");
    assert_eq!(
        hex_encode(&out),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
}

// ---- SHA-512: FIPS-180-4 vectors ----

#[test]
fn sha512_empty_string() {
    let mut out = [0u8; 64];
    Sha512Hasher::hash(b"", &mut out).expect("hash succeeds");
    assert_eq!(
        hex_encode(&out),
        "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
    );
}

#[test]
fn sha512_abc() {
    // FIPS-180-4 §C.1 — canonical "abc" vector.
    let mut out = [0u8; 64];
    Sha512Hasher::hash(b"abc", &mut out).expect("hash succeeds");
    assert_eq!(
        hex_encode(&out),
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
    );
}

// ---- SHA3-256: FIPS-202 vectors ----

#[test]
fn sha3_256_empty_string() {
    let mut out = [0u8; 32];
    Sha3_256Hasher::hash(b"", &mut out).expect("hash succeeds");
    assert_eq!(
        hex_encode(&out),
        "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
    );
}

#[test]
fn sha3_256_abc() {
    let mut out = [0u8; 32];
    Sha3_256Hasher::hash(b"abc", &mut out).expect("hash succeeds");
    assert_eq!(
        hex_encode(&out),
        "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
    );
}

// ---- Keccak-256: pre-FIPS Keccak Team test vectors ----

#[test]
fn keccak256_empty_string() {
    // The Ethereum keccak256("") canonical vector.
    let mut out = [0u8; 32];
    Keccak256Hasher::hash(b"", &mut out).expect("hash succeeds");
    assert_eq!(
        hex_encode(&out),
        "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
    );
}

#[test]
fn keccak256_abc() {
    let mut out = [0u8; 32];
    Keccak256Hasher::hash(b"abc", &mut out).expect("hash succeeds");
    assert_eq!(
        hex_encode(&out),
        "4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45"
    );
}

// ---- BLAKE3 ----

#[test]
fn blake3_empty_string() {
    let mut out = [0u8; 32];
    Blake3Hasher::hash(b"", &mut out).expect("hash succeeds");
    assert_eq!(
        hex_encode(&out),
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
    );
}

#[test]
fn blake3_abc() {
    let mut out = [0u8; 32];
    Blake3Hasher::hash(b"abc", &mut out).expect("hash succeeds");
    assert_eq!(
        hex_encode(&out),
        "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85"
    );
}

// ---- MerkleRootCommitment: structural conformance ----

#[test]
fn merkle_root_two_leaves() {
    // Two leaves of all-zeros: root should be SHA256(0^32 || 0^32).
    let leaves = [0u8; 64];
    let mut out = [0u8; 32];
    let n = MerkleRootCommitment::commit(&leaves, &mut out).expect("commit succeeds");
    assert_eq!(n, 32);
    // Compute the expected: sha256(64 zero bytes).
    let mut expected = [0u8; 32];
    Sha256Hasher::hash(&leaves, &mut expected).expect("sha256 succeeds");
    assert_eq!(out, expected);
}

#[test]
fn merkle_root_rejects_non_power_of_two() {
    // 3 leaves — not a power of two — must fail.
    let leaves = [0u8; 96];
    let mut out = [0u8; 32];
    let err = MerkleRootCommitment::commit(&leaves, &mut out).unwrap_err();
    assert_eq!(
        err.constraint_iri,
        "https://uor.foundation/axis/CommitmentAxis/MerkleRoot/powerOfTwoLeaves"
    );
}

#[test]
fn merkle_root_rejects_misaligned_leaves() {
    // 33 bytes — not a multiple of 32 — must fail.
    let leaves = [0u8; 33];
    let mut out = [0u8; 32];
    let err = MerkleRootCommitment::commit(&leaves, &mut out).unwrap_err();
    assert_eq!(
        err.constraint_iri,
        "https://uor.foundation/axis/CommitmentAxis/MerkleRoot/leafAlignment"
    );
}

// ---- Helper smoke test: hex roundtrip ----

#[test]
fn hex_roundtrip() {
    let bytes = [0x12, 0x34, 0xab, 0xcd];
    assert_eq!(hex_decode(&hex_encode(&bytes)), bytes);
}

// ---- Parametricity: Merkle over alternate hashers ----

#[test]
fn merkle_root_with_blake3_hasher() {
    // Same input as merkle_root_two_leaves but using BLAKE3 as the
    // hasher. Verifies the parametric MerkleRoot<H, LEAF_BYTES>
    // composition: switching H switches the digest at every layer.
    type MerkleBlake3 = MerkleRoot<Blake3Hasher, 32>;
    let leaves = [0u8; 64];
    let mut out = [0u8; 32];
    let n = MerkleBlake3::commit(&leaves, &mut out).expect("commit ok");
    assert_eq!(n, 32);
    let mut expected = [0u8; 32];
    Blake3Hasher::hash(&leaves, &mut expected).expect("blake3 ok");
    assert_eq!(out, expected);
}

#[test]
fn merkle_root_with_keccak_hasher() {
    type MerkleKeccak = MerkleRoot<Keccak256Hasher, 32>;
    let leaves = [0u8; 64];
    let mut out = [0u8; 32];
    let n = MerkleKeccak::commit(&leaves, &mut out).expect("commit ok");
    assert_eq!(n, 32);
    let mut expected = [0u8; 32];
    Keccak256Hasher::hash(&leaves, &mut expected).expect("keccak ok");
    assert_eq!(out, expected);
}

#[test]
fn merkle_root_with_sha512_hasher() {
    type MerkleSha512 = MerkleRoot<Sha512Hasher, 64>;
    let leaves = [0u8; 128];
    let mut out = [0u8; 64];
    let n = MerkleSha512::commit(&leaves, &mut out).expect("commit ok");
    assert_eq!(n, 64);
    let mut expected = [0u8; 64];
    Sha512Hasher::hash(&leaves, &mut expected).expect("sha512 ok");
    assert_eq!(out, expected);
}

#[test]
fn merkle_root_four_leaves() {
    // Two-layer tree: 4 leaves → 2 pairs → 1 root.
    // Each leaf is byte i replicated 32 times.
    let mut leaves = [0u8; 128];
    for layer in 0..4 {
        for j in 0..32 {
            leaves[layer * 32 + j] = layer as u8;
        }
    }
    let mut out = [0u8; 32];
    MerkleRootCommitment::commit(&leaves, &mut out).expect("commit ok");

    // Manually compute the expected root.
    let mut pair01 = [0u8; 64];
    pair01[..32].copy_from_slice(&leaves[..32]);
    pair01[32..].copy_from_slice(&leaves[32..64]);
    let mut hash01 = [0u8; 32];
    Sha256Hasher::hash(&pair01, &mut hash01).expect("sha256 ok");

    let mut pair23 = [0u8; 64];
    pair23[..32].copy_from_slice(&leaves[64..96]);
    pair23[32..].copy_from_slice(&leaves[96..128]);
    let mut hash23 = [0u8; 32];
    Sha256Hasher::hash(&pair23, &mut hash23).expect("sha256 ok");

    let mut top = [0u8; 64];
    top[..32].copy_from_slice(&hash01);
    top[32..].copy_from_slice(&hash23);
    let mut expected = [0u8; 32];
    Sha256Hasher::hash(&top, &mut expected).expect("sha256 ok");

    assert_eq!(out, expected);
}

// ---- Parametric shape introspection ----

#[test]
fn digest_shape_site_counts() {
    assert_eq!(<Digest<32> as ConstrainedTypeShape>::SITE_COUNT, 32);
    assert_eq!(<Digest<48> as ConstrainedTypeShape>::SITE_COUNT, 48);
    assert_eq!(<Digest<64> as ConstrainedTypeShape>::SITE_COUNT, 64);
}

#[test]
fn pubkey_signature_shapes() {
    assert_eq!(<PublicKey<32> as ConstrainedTypeShape>::SITE_COUNT, 32);
    assert_eq!(<PublicKey<48> as ConstrainedTypeShape>::SITE_COUNT, 48);
    assert_eq!(<Signature<64> as ConstrainedTypeShape>::SITE_COUNT, 64);
    assert_eq!(<Signature<96> as ConstrainedTypeShape>::SITE_COUNT, 96);
}

#[test]
fn merkle_proof_shape_size() {
    // Depth-6 SHA-256 Merkle proof: 6 sibling-digests + 8-byte leaf-index
    // = 6 * 32 + 8 = 200 bytes.
    type Proof6 = MerkleProofShape<6, 32>;
    assert_eq!(<Proof6 as ConstrainedTypeShape>::SITE_COUNT, 200);
}

#[test]
fn shapes_share_constrained_type_iri() {
    // ADR-017 closure rule: empty-CONSTRAINTS shapes content-address
    // through (SITE_COUNT, CONSTRAINTS) regardless of Rust name.
    assert_eq!(
        <Digest<32> as ConstrainedTypeShape>::IRI,
        <PublicKey<32> as ConstrainedTypeShape>::IRI,
    );
    assert_eq!(
        <Digest<32> as ConstrainedTypeShape>::IRI,
        "https://uor.foundation/type/ConstrainedType"
    );
}

// ---- Compile-time bound resolution: shapes are GroundedShape-bound ----

#[allow(dead_code)]
fn _shapes_are_grounded_shape() {
    fn check<S: uor_foundation::enforcement::GroundedShape>() {}
    check::<Digest<32>>();
    check::<Digest<48>>();
    check::<Digest<64>>();
    check::<PublicKey<32>>();
    check::<Signature<64>>();
    check::<Signature<96>>();
    check::<MerkleProofShape<6, 32>>();
}
