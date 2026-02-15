#!/usr/bin/env python3
"""
Mapping External Data into PRISM Coordinates
=============================================

Run: python examples/mapping.py

Any data that fits in bytes gets a unique triadic address. This
demo shows how to project characters, colors, sensor readings,
and cross-system records into PRISM's coordinate space.
"""

import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from prism import Q0, Q1, Q


def ascii_characters():
    """Every ASCII character has a triadic coordinate."""
    print("ASCII CHARACTERS")
    print("=" * 56)
    engine = Q0()
    engine.verify()

    for char in "PRISM":
        code = ord(char)
        t = engine.triad(code)
        print(f"  '{char}' (ASCII {code:3d}): "
              f"elevation={t.stratum[0]}, fix=bits{t.spectrum[0]}")

    print("\n  Character similarity:")
    for a, b in [('A', 'B'), ('A', 'a'), ('0', '9')]:
        r = engine.correlate(ord(a), ord(b))
        print(f"    '{a}' vs '{b}': fidelity = {r['fidelity']:.3f}")
    print()


def rgb_pixels():
    """A 24-bit RGB pixel maps to Quantum 2 (3 bytes, one per channel)."""
    print("RGB PIXELS (Quantum 2)")
    print("=" * 56)
    engine = Q(2)
    engine.verify()

    colors = {
        "Red":     (255, 0, 0),     "Green":   (0, 255, 0),
        "Blue":    (0, 0, 255),     "White":   (255, 255, 255),
        "Black":   (0, 0, 0),       "Yellow":  (255, 255, 0),
    }

    for name, rgb in colors.items():
        t = engine.triad(rgb)
        print(f"  {name:7s} {str(rgb):16s}  "
              f"elevation=R:{t.stratum[0]} G:{t.stratum[1]} B:{t.stratum[2]}  "
              f"total={t.total_stratum}/24")

    print("\n  Color similarity:")
    for a, b in [("Red", "Green"), ("Red", "Yellow"), ("White", "Black")]:
        r = engine.correlate(
            engine._from_bytes(colors[a]), engine._from_bytes(colors[b]))
        print(f"    {a:6s} vs {b:6s}: fidelity = {r['fidelity']:.3f}")
    print()


def cross_system_mapping():
    """Two systems with different schemas project into the same space."""
    print("CROSS-SYSTEM DATA MAPPING (Quantum 3)")
    print("=" * 56)
    engine = Q(3)
    engine.verify()

    sys_a = {"active": 0x01, "inactive": 0x02, "suspended": 0x04, "deleted": 0x08}
    sys_b = {"enabled": 0x01, "disabled": 0x02, "blocked": 0x10, "removed": 0x08}

    print("\n  Automatic correspondence via PRISM coordinates:\n")
    for a_name, a_code in sys_a.items():
        best, best_f = None, -1
        for b_name, b_code in sys_b.items():
            f = engine.correlate(a_code, b_code)['fidelity']
            if f > best_f:
                best_f, best = f, b_name
        status = "exact match" if best_f == 1.0 else f"fidelity {best_f:.3f}"
        print(f"    A.{a_name:10s} ↔ B.{best:10s}  ({status})")


def main():
    ascii_characters()
    rgb_pixels()
    cross_system_mapping()


if __name__ == "__main__":
    main()
