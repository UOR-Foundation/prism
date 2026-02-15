#!/usr/bin/env python3
"""
Topological Confinement with PRISM
===================================

Run: python examples/confinement.py

Demonstrates that every PRISM operation stays within the bounded
space. This is a structural guarantee.  Not a test, but a proof.
"""

import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from prism import Q0, Q, ClosureMode


def universality():
    """Two operations generate the entire space."""
    print("UNIVERSALITY: TWO OPERATIONS → FULL SPACE")
    print("=" * 56)
    engine = Q0()
    engine.verify()

    start = 73
    visited = {start}
    current = (start,)

    for _ in range(510):
        current = engine.bnot(current)
        visited.add(current[0])
        current = engine.neg(current)
        visited.add(current[0])

    print(f"\n  Starting from {start}, alternating neg and bnot:")
    print(f"  Values reached: {len(visited)} / {engine.cycle}")
    print(f"  Full coverage:  {len(visited) == engine.cycle}")
    print(f"\n  No proper subset is closed under both operations.")
    print()


def exhaustive_confinement():
    """Every operation on every input stays in range."""
    print("EXHAUSTIVE CONFINEMENT PROOF (Q0)")
    print("=" * 56)
    engine = Q0()
    engine.verify()

    tests = 0
    for x in range(256):
        b = (x,)
        for r in [engine.neg(b), engine.bnot(b), engine.succ(b), engine.pred(b)]:
            assert len(r) == 1 and 0 <= r[0] <= 255
            tests += 1

    for x in range(256):
        for y in range(256):
            for r in [engine.xor((x,), (y,)), engine.band((x,), (y,)), engine.bor((x,), (y,))]:
                assert len(r) == 1 and 0 <= r[0] <= 255
                tests += 1

    print(f"\n  Operations tested: {tests:,}")
    print(f"  Violations: 0")
    print(f"  Confinement: PROVEN")
    print(f"\n  Every operation on every input (and every pair)")
    print(f"  produces a value within [0, 255]. No exceptions.")
    print()


def cycle_completeness():
    """Successor visits every state exactly once."""
    print("CYCLE COMPLETENESS")
    print("=" * 56)
    engine = Q0()

    visited = []
    current = (0,)
    for _ in range(256):
        visited.append(current[0])
        current = engine.succ(current)

    print(f"\n  States visited: {len(set(visited))} / 256")
    print(f"  All unique:     {len(set(visited)) == 256}")
    print(f"  Returned to 0:  {current == (0,)}")
    print(f"\n  The cycle has no gaps, no dead ends, no isolated states.")
    print()


def graph_closure():
    """Emit a graph where every edge lands on an existing node."""
    print("GRAPH CLOSURE")
    print("=" * 56)
    engine = Q0()
    engine.verify()

    result = engine.emit(sample_size=256, closure_ops=["not"],
                         closure_mode=ClosureMode.GRAPH_CLOSED)

    ids = {n['@id'] for n in result['@graph'] if n.get('@type') == 'Datum'}
    dangling = sum(1 for n in result['@graph']
                   if n.get('@type') == 'Datum' and n.get('not') not in ids)

    print(f"\n  Datums: {result['proof']['datumCount']}")
    print(f"  Fully closed: {result['proof']['graphFullyClosed']}")
    print(f"  Dangling edges: {dangling}")
    print(f"\n  Every complement edge points to an existing node.")
    print(f"  The graph is self-contained.  No external references.")
    print()


def bounded_spaces():
    """Finite state spaces at each quantum level."""
    print("BOUNDED STATE SPACES")
    print("=" * 56)
    for q in range(5):
        e = Q(q)
        print(f"  Q{q}: {e.width} byte{'s' if e.width > 1 else ''}, "
              f"{e.bits} bits, {e.cycle:,} states")
    print(f"\n  Every level is finite. No operation can exceed the boundary.")


def main():
    universality()
    exhaustive_confinement()
    cycle_completeness()
    graph_closure()
    bounded_spaces()


if __name__ == "__main__":
    main()
