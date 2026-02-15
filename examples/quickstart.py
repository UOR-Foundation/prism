#!/usr/bin/env python3
"""
PRISM Quick Start
=================

Run: python examples/quickstart.py

Demonstrates the core operations: triadic coordinates, navigation,
the critical identity, verified computation, and correlation.
"""

import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from prism import Q0


def main():
    engine = Q0()
    engine.verify()
    print("✓ Engine verified: all algebraic laws hold.\n")

    # ── Triadic Coordinates ───────────────────────────────────
    print("TRIADIC COORDINATES")
    print("=" * 56)
    for value in [0, 42, 127, 255]:
        t = engine.triad(value)
        print(f"\n  {value:3d} (0x{value:02X}):")
        print(f"    Identity (datum):         {t.datum}")
        print(f"    Magnitude (stratum):      {t.stratum[0]} of 8 bits active")
        print(f"    Structure (spectrum):      bits {t.spectrum[0]}")

    # ── Navigation ────────────────────────────────────────────
    print(f"\n\nNAVIGATION")
    print("=" * 56)
    x = 42
    print(f"\n  Starting at {x}:")
    print(f"    succ({x})  = {engine.succ((x,))[0]:3d}  (next)")
    print(f"    pred({x})  = {engine.pred((x,))[0]:3d}  (previous)")
    print(f"    neg({x})   = {engine.neg(x)[0]:3d}  (additive inverse: {x} + {engine.neg(x)[0]} = {(x + engine.neg(x)[0]) % 256})")
    print(f"    bnot({x})  = {engine.bnot(x)[0]:3d}  (complement: every bit flipped)")

    # ── Critical Identity ─────────────────────────────────────
    print(f"\n\nCRITICAL IDENTITY: neg(bnot(x)) = x + 1")
    print("=" * 56)
    for x in [0, 42, 127, 254, 255]:
        result = engine.neg(engine.bnot((x,)))[0]
        expected = (x + 1) % 256
        print(f"  neg(bnot({x:3d})) = {result:3d}  ✓" if result == expected else f"  FAILED at {x}")

    # ── Verified Computation ──────────────────────────────────
    print(f"\n\nVERIFIED COMPUTATION")
    print("=" * 56)
    t1 = engine.make_term("xor", 0x55, 0xAA)
    t2 = engine.make_term("xor", 0xAA, 0x55)  # same op, different order
    d1, d2 = engine.derive(t1), engine.derive(t2)

    print(f"\n  xor(0x55, 0xAA)  →  result: 0x{d1.result_datum[0]:02X}")
    print(f"  xor(0xAA, 0x55)  →  result: 0x{d2.result_datum[0]:02X}")
    print(f"  Same derivation ID: {d1.derivation_id == d2.derivation_id}")
    print(f"  ID: {d1.derivation_id}")

    # ── Correlation ───────────────────────────────────────────
    print(f"\n\nCORRELATION")
    print("=" * 56)
    for a, b in [(42, 43), (0x55, 0xAA), (0, 255), (100, 100)]:
        r = engine.correlate(a, b)
        print(f"  {a:3d} vs {b:3d}:  fidelity = {r['fidelity']:.3f}  "
              f"({r['totalDifference']} of {r['maxDifference']} bits differ)")

    # ── Full Cycle ────────────────────────────────────────────
    print(f"\n\nFULL CYCLE (first 10 steps)")
    print("=" * 56)
    current = (0,)
    steps = []
    for _ in range(10):
        steps.append(current[0])
        current = engine.succ(current)
    print(f"  {' → '.join(str(s) for s in steps)} → ...")
    print(f"  (continues through all 256 values, returns to 0)")


if __name__ == "__main__":
    main()
