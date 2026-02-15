#!/usr/bin/env python3
"""
PRISM Verification Suite

Run:  python examples/test_prism.py
      python -m pytest examples/test_prism.py -v
"""

import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from prism import UOR, Q0, Q1, Q, CoherenceError, ClosureMode


# ── Coherence ─────────────────────────────────────────────────

def test_coherence_q0():
    assert Q0().verify() is True

def test_coherence_q1():
    assert Q1().verify() is True

def test_coherence_q2():
    assert Q(2).verify() is True


# ── Involutions ───────────────────────────────────────────────

def test_involution_neg():
    e = Q0()
    for x in range(256):
        assert e.neg(e.neg((x,))) == (x,)

def test_involution_bnot():
    e = Q0()
    for x in range(256):
        assert e.bnot(e.bnot((x,))) == (x,)


# ── Critical Identity ────────────────────────────────────────

def test_critical_identity_succ():
    e = Q0()
    for x in range(256):
        assert e.neg(e.bnot((x,))) == ((x + 1) % 256,)

def test_critical_identity_pred():
    e = Q0()
    for x in range(256):
        assert e.bnot(e.neg((x,))) == ((x - 1) % 256,)

def test_succ_pred_inverse():
    e = Q0()
    for x in range(256):
        b = (x,)
        assert e.succ(e.pred(b)) == b
        assert e.pred(e.succ(b)) == b


# ── Cycle ─────────────────────────────────────────────────────

def test_full_cycle():
    e = Q0()
    visited = set()
    current = (0,)
    for _ in range(256):
        assert current not in visited
        visited.add(current)
        current = e.succ(current)
    assert current == (0,) and len(visited) == 256


# ── Algebraic Laws ───────────────────────────────────────────

def test_xor_self_cancellation():
    e = Q0()
    for x in range(256):
        assert e.xor((x,), (x,)) == (0,)

def test_xor_complement():
    e = Q0()
    for x in range(256):
        assert e.xor((x,), e.bnot((x,))) == (0xFF,)

def test_additive_inverse():
    e = Q0()
    for x in range(256):
        assert (x + e.neg(x)[0]) % 256 == 0

def test_stratum_symmetry():
    e = Q0()
    for x in range(256):
        assert e.stratum((x,))[0] + e.stratum(e.bnot((x,)))[0] == 8

def test_basis_recomposition():
    e = Q0()
    for x in range(256):
        recomposed = sum(1 << bit for bit in e.spectrum(x)[0])
        assert recomposed == x


# ── Triad Uniqueness ─────────────────────────────────────────

def test_triad_uniqueness():
    e = Q0()
    triads = set()
    for x in range(256):
        t = e.triad(x)
        key = (t.datum, t.stratum, t.spectrum)
        assert key not in triads
        triads.add(key)


# ── Canonicalization ─────────────────────────────────────────

def test_canonicalization_commutativity():
    e = Q0(); e.verify()
    d1 = e.derive(e.make_term("xor", 0x55, 0xAA))
    d2 = e.derive(e.make_term("xor", 0xAA, 0x55))
    assert d1.derivation_id == d2.derivation_id

def test_canonicalization_identity():
    e = Q0(); e.verify()
    d1 = e.derive(e.make_term("xor", 0x55, 0x00))
    assert d1.result_datum == (0x55,)

def test_canonicalization_self_cancel():
    e = Q0(); e.verify()
    d = e.derive(e.make_term("xor", 0x55, 0x55))
    assert d.result_datum == (0,)

def test_canonicalization_zero_elimination():
    e = Q0(); e.verify()
    d1 = e.derive(e.make_term("xor", 0x55, 0xAA))
    d2 = e.derive(e.make_term("xor", 0x55, 0x00, 0xAA))
    assert d1.derivation_id == d2.derivation_id


# ── Correlation ──────────────────────────────────────────────

def test_correlation_identical():
    r = Q0().correlate(42, 42)
    assert r['fidelity'] == 1.0 and r['totalDifference'] == 0

def test_correlation_opposite():
    r = Q0().correlate(0x00, 0xFF)
    assert r['fidelity'] == 0.0 and r['totalDifference'] == 8

def test_correlation_symmetry():
    e = Q0()
    assert e.correlate(42, 100)['fidelity'] == e.correlate(100, 42)['fidelity']


# ── Confinement ──────────────────────────────────────────────

def test_confinement_unary():
    e = Q0()
    for x in range(256):
        b = (x,)
        for r in [e.neg(b), e.bnot(b), e.succ(b), e.pred(b)]:
            assert len(r) == 1 and 0 <= r[0] <= 255

def test_confinement_binary():
    e = Q0()
    for x in range(256):
        for y in range(256):
            for r in [e.xor((x,), (y,)), e.band((x,), (y,)), e.bor((x,), (y,))]:
                assert len(r) == 1 and 0 <= r[0] <= 255


# ── Emission ─────────────────────────────────────────────────

def test_emit_structure():
    e = Q0(); e.verify()
    r = e.emit(sample_size=10, closure_ops=[])
    assert "@context" in r and "proof" in r and "@graph" in r
    assert r["proof"]["verified"] is True

def test_emit_with_derivations():
    e = Q0(); e.verify()
    d = e.derive(e.make_term("xor", 0x55, 0xAA))
    r = e.emit(sample_size=256, closure_ops=[], include_derivations=[d])
    nodes = [n for n in r["@graph"] if n.get("@id") == d.result_iri]
    assert len(nodes) == 1 and "derivations" in nodes[0]


# ── Runner ───────────────────────────────────────────────────

def run_all():
    tests = [(n, f) for n, f in sorted(globals().items())
             if n.startswith("test_") and callable(f)]
    passed = failed = 0
    for name, fn in tests:
        try:
            fn(); print(f"  ✓ {name}"); passed += 1
        except Exception as e:
            print(f"  ✗ {name}: {e}"); failed += 1
    print(f"\n  {passed} passed, {failed} failed, {passed + failed} total")
    return failed == 0


if __name__ == "__main__":
    print("PRISM Verification Suite\n")
    sys.exit(0 if run_all() else 1)
