<p align="center">
<img src="docs/cover.png" alt="PRISM: A Universal Coordinate System for Information" width="100%">
</p>

# PRISM

**A Universal Coordinate System for Information**

GPS gave every point on Earth a unique address. Before it existed, navigation depended on local maps that could not talk to each other. After it existed, any system anywhere could describe exactly the same location with exactly the same coordinates.

PRISM does this for data. It assigns every possible digital value a unique, canonical coordinate derived from the value's own structure, not from where it is stored, who created it, or what system holds it. Two independent databases, two AI models, two research teams on opposite sides of the world encoding the same value will always arrive at the same coordinate. No negotiation. No translation layer. No information lost.

The result is a shared reference frame for all digital information: universal addressing, structural comparison, verified computation, and lossless encoding on a closed algebraic space. One coordinate system. Every value. Every scale.

## The Problem

Information is the foundational substrate of the digital age, yet it lacks what geography secured centuries ago: a universal coordinate system.

Current identifiers (file paths, database keys, URLs) indicate *storage location*, not structural position. Identity is entangled with infrastructure. Meaning depends on external schemas rather than intrinsic form.

This produces systemic fragmentation. Scientific domains evolve in isolation because their data models are locally defined and globally incompatible. Insights remain confined within disciplinary boundaries, limiting cross-domain synthesis and delaying the recognition of deeper unifying patterns. The consequence is not inefficiency but constraint: discoveries that could compound across fields are slowed, diluted, or never connected at all.

Artificial intelligence faces the same discontinuity. Models trained on heterogeneous data rely on translation and statistical approximation rather than shared invariants, weakening both interoperability and interpretability.

Without a common framework in which informational objects occupy determinate, invariant positions, research progress itself is structurally incoherent.

## The Core Insight

What if every piece of data could be addressed not by where it lives, but by what it is?

Every digital value has intrinsic attributes: its raw bytes, how many bits are active, and which specific bits are active. These are not interpretations. They are structural facts, present in the value itself, observable by anyone, independent of any system. An object addressed by its own attributes needs no registry, no authority, and no schema. The address is the structure. The structure is the address.

PRISM is the universal lossless encoder that makes this work. It maps arbitrary data onto a finite, closed topological space (a torus) where every value is reachable, every value is unique, and nothing can fall off. One algebraic rule holds the entire system together:

```
negate(complement(x)) = x + 1
```

Apply negation after complement and you always get the next value. Start anywhere, keep going, and you visit every value in the space exactly once before arriving back where you started. The path wraps around and closes on itself, the way the surface of a donut has no edge. This is what guarantees that every object can be addressed by its attributes: the space is closed, the encoding is lossless, and the coordinate is canonical.

GPS works because Earth is a closed surface where three measurements pin you to one spot. PRISM works because information space is a closed surface where three attributes pin you to one value. Same geometry. Same result.

## How PRISM Works

Every value is resolved through three independent constraints, collectively called a **triad**:

| Constraint | What It Resolves | GPS Parallel |
|---|---|---|
| **Datum** | Identity: the value itself, as bytes | Lat/Long fixes you to a point on the surface |
| **Stratum** | Magnitude: how many bits are active per byte | Elevation determines height above sea level |
| **Spectrum** | Structure: which specific bits are active | Satellite triangulation eliminates the last ambiguity |

Datum anchors identity. Stratum constrains magnitude. Spectrum resolves structure.

Fewer than three constraints leave ambiguity. When the triad is complete, uncertainty collapses to one unique point. The coordinate does not describe the value; the intersection of constraints creates it.

Every operation produces a **derivation**: a content-addressed certificate recording the inputs, operation, and result. The same computation always yields the same certificate regardless of when, where, or how many times it runs.

## What This Enables

**Cross-system data unification.** Two databases with incompatible schemas can both project their records into PRISM coordinates. Identical values resolve to the same point. Distance between any two values is measurable without schema alignment.

**AI interpretability.** Neural network activations and embeddings are bit patterns. PRISM resolves each one to a triadic coordinate, tracks transformations as algebraic derivations, and measures similarity between any two internal states. Opaque model internals become a navigable, auditable map.

**Computational confinement.** Every operation maps values inside the space to values inside the space. Nothing can escape. For safety-critical systems, this is a structural guarantee, provable, not just testable.

**Reproducible provenance.** Every computation carries a content-addressed certificate. Independent verification without re-running the experiment.

## Minimal Example

```python
from prism import Q0

engine = Q0()
engine.verify()        # confirms all algebraic laws before any output

t = engine.triad(42)
t.datum                # (42,)           identity
t.stratum              # (3,)            magnitude
t.spectrum             # ((1, 3, 5),)    structure

# Navigate
engine.succ((42,))     # (43,)   next value
engine.neg((42,))      # (214,)  additive inverse
engine.bnot((42,))     # (213,)  bitwise complement

# Measure distance
engine.correlate(42, 43)['fidelity']   # 0.875

# Verified computation
term = engine.make_term("xor", 0x55, 0xAA)
d = engine.derive(term)
d.derivation_id        # content-addressed certificate

# Scale to any width
from prism import Q
engine32 = Q(3)        # 32-bit: 4.29 billion states
```

The algebra is identical at every scale.

## Getting Started

```bash
pip install click
git clone https://github.com/UOR-Foundation/PRISM.git
cd PRISM
```

**1. Verify the algebra.** Run `python prism.py --verbose`. This checks every algebraic law against every value in the 8-bit space and emits a complete JSON-LD graph. If it passes, the system is proven coherent.

**2. Explore the coordinates.** Run `python examples/quickstart.py`. Assigns triadic coordinates, navigates the space, verifies the critical identity, measures distance between values.

**3. Map real data.** Run `python examples/mapping.py`. Projects ASCII, RGB, and database status codes into PRISM coordinates. Shows automatic cross-system correspondence.

**4. Trace AI internals.** Run `python examples/interpretability.py`. Fingerprints activations, traces transformations with certificates, detects structural clusters.

**5. Prove confinement.** Run `python examples/confinement.py`. 197,632 operation checks, zero violations. Full cycle. Closed graph. The structural safety guarantee.

**Run tests.** `python examples/test_prism.py` runs all 26 verification tests.

## Repository

```
prism.py                The engine
README.md               This document
LICENSE                 MIT
docs/
  CONCEPTS.md           How it works, from first principles
  ALGEBRA.md            The proofs and the universality theorem
  API.md                Every function, class, and parameter
examples/
  quickstart.py         Coordinates, navigation, derivations
  mapping.py            External data into PRISM coordinates
  interpretability.py   Activations, tracing, clustering
  confinement.py        Bounded computation proofs
  test_prism.py         26 verification tests
```

## License

MIT

<p align="center">
<a href="https://www.uor.foundation/">The UOR Foundation</a>
</p>
