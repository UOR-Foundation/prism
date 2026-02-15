#!/usr/bin/env python3
"""
AI Interpretability with PRISM
==============================

Run: python examples/interpretability.py

Demonstrates how PRISM can map, trace, and measure neural network
internals. Uses simulated activations. In practice, extract real
activation patterns from a model and project them into PRISM.
"""

import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from prism import Q0


def activation_fingerprinting():
    """Each activation pattern gets a structural fingerprint."""
    print("ACTIVATION FINGERPRINTING")
    print("=" * 56)
    engine = Q0()
    engine.verify()

    # Simulated 8-bit quantized activations (4 neurons)
    patterns = {
        "Input A": [0x3F, 0xC0, 0x55, 0xAA],
        "Input B": [0x3E, 0xC1, 0x55, 0xAB],  # Similar to A
        "Input C": [0xC0, 0x3F, 0xAA, 0x55],  # Structurally different
    }

    for label, pat in patterns.items():
        strata = [engine.triad(v).total_stratum for v in pat]
        print(f"\n  {label}: {[f'0x{v:02X}' for v in pat]}")
        print(f"    Per-neuron elevation: {strata}")
        print(f"    Total density: {sum(strata)} / {len(pat) * 8} bits")

    print("\n  Pattern similarity (average per-neuron fidelity):")
    for a_label, b_label in [("Input A", "Input B"), ("Input A", "Input C")]:
        fids = [engine.correlate(va, vb)['fidelity']
                for va, vb in zip(patterns[a_label], patterns[b_label])]
        print(f"    {a_label} vs {b_label}: {sum(fids)/len(fids):.3f}")
    print()


def transformation_tracing():
    """Track and certify each transformation step."""
    print("TRANSFORMATION TRACING")
    print("=" * 56)
    engine = Q0()
    engine.verify()

    initial = 0x55
    print(f"\n  Start: 0x{initial:02X}  "
          f"(elevation={engine.triad(initial).stratum[0]}, "
          f"fix=bits{engine.triad(initial).spectrum[0]})\n")

    steps = [
        ("bnot", None,  "Complement (feature inversion)"),
        ("neg",  None,  "Negate (sign flip)"),
        ("xor",  0x0F,  "XOR mask (selective toggle)"),
    ]

    current = initial
    for i, (op, arg, desc) in enumerate(steps):
        term = engine.make_term(op, current, arg) if arg else engine.make_term(op, current)
        d = engine.derive(term)
        result = d.result_datum[0]
        print(f"  Step {i+1}: {desc}")
        print(f"    {op}(0x{current:02X}{f', 0x{arg:02X}' if arg else ''}) → 0x{result:02X}")
        print(f"    Certificate: {d.derivation_id}")
        current = result

    change = engine.correlate(initial, current)
    print(f"\n  Total: 0x{initial:02X} → 0x{current:02X}")
    print(f"  Fidelity to original: {change['fidelity']:.3f}")
    print(f"  Bits changed: {change['totalDifference']}/{change['maxDifference']}")
    print()


def cluster_detection():
    """PRISM's correlation metric reveals cluster structure."""
    print("CLUSTER DETECTION")
    print("=" * 56)
    engine = Q0()
    engine.verify()

    activations = [0x33, 0x35, 0x37, 0x3B, 0x3D,  # Cluster 1
                   0xCC, 0xCE, 0xCF, 0xCD, 0xCB]   # Cluster 2

    print("\n  Within-cluster fidelity:")
    for label, cluster in [("Cluster 1", activations[:5]), ("Cluster 2", activations[5:])]:
        fids = []
        for i in range(len(cluster)):
            for j in range(i+1, len(cluster)):
                fids.append(engine.correlate(cluster[i], cluster[j])['fidelity'])
        print(f"    {label}: avg = {sum(fids)/len(fids):.3f}")

    print("\n  Between-cluster fidelity:")
    cross = []
    for a in activations[:5]:
        for b in activations[5:]:
            cross.append(engine.correlate(a, b)['fidelity'])
    print(f"    Cluster 1 vs 2: avg = {sum(cross)/len(cross):.3f}")
    print("\n  → High within, low between: clusters are structurally separable")


def main():
    activation_fingerprinting()
    transformation_tracing()
    cluster_detection()


if __name__ == "__main__":
    main()
