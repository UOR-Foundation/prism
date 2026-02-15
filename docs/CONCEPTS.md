# How PRISM Works

This document explains PRISM from first principles. No background in mathematics, computer science, or AI is assumed.

## The Idea in One Sentence

PRISM is the engine of the UOR coordinate system for information. It resolves every possible digital value to a unique point through three independent constraints, the same way GPS resolves a physical location. Uniqueness is not assumed; it is constructed.

## Why a Coordinate System Matters

Before GPS, every map used its own reference system. Sailors had nautical charts, hikers had topographic maps, pilots had aeronautical charts. Translating between them was manual, unreliable, and sometimes impossible. GPS solved this by providing a single, universal reference frame that all systems could project into.

Digital information has the same problem today. Every database, every AI model, every software system stores and references data using its own internal scheme. When systems need to exchange data, they rely on improvised translations (APIs, format converters, schema mappers), each one a potential source of error.

PRISM provides the shared reference frame. If two systems both project their data into PRISM coordinates, they can compare, relate, and verify their data without needing to understand each other's internal representations.

## How GPS Actually Works

GPS does not "give" you a location. It eliminates uncertainty until only one location remains.

A single satellite signal tells you that you are somewhere on the surface of a sphere. That is an infinite number of possible positions. A second signal constrains you to a circle: the intersection of two spheres. Still many possibilities. A third signal narrows it to a point: the single location where all three spheres intersect.

Each signal adds an independent constraint. Fewer than three constraints leave ambiguity. When the third constraint is complete, uncertainty collapses to one unique point. Uniqueness is not assumed; it is constructed by completing the triad.

## The Three Constraints

UOR works the same way in information space. A value is not merely stored. It is resolved through three informational constraints. Collectively, they are called a **triad**.

### 1. Datum: The Identity Constraint

The datum is the value itself, stored as a sequence of bytes. Datum anchors identity, like latitude and longitude fixing you to a point on Earth's surface.

Every possible byte sequence corresponds to exactly one position. The number 42, the letter `*`, and any other data that maps to the same byte pattern all occupy the same position. Different byte patterns always occupy different positions.

But knowing the datum alone is like knowing only your latitude and longitude. You still need more information to fully resolve the point.

### 2. Stratum: The Magnitude Constraint

The stratum counts how many bits are "active" (set to 1) in each byte. Stratum constrains magnitude, like elevation determining how high above sea level you stand.

A value with stratum 0 has no active bits. It is at sea level, carrying no information. A value with stratum 8 (all bits on) is at peak altitude, fully saturated. Everything else falls somewhere in between.

For the value 42 (binary: `00101010`), the stratum is 3. Three bits are active. Elevation 3 out of a possible 8.

### 3. Spectrum: The Structure Constraint

The spectrum identifies *which specific bits* are active. Spectrum resolves internal structure, like satellite triangulation refining timing differences to eliminate the last ambiguity and pinpoint your exact position.

Without the spectrum, two values with the same stratum (same number of active bits) could still be different values, just as two locations at the same elevation could be in completely different places. The spectrum resolves this by identifying exactly which bit positions compose the value.

For the value 42, the spectrum is bits 1, 3, and 5. A different value with stratum 3 (say, 7, which uses bits 0, 1, and 2) has the same magnitude but a completely different structure. The spectrum distinguishes them.

### Together: Uncertainty Collapses to One Point

Datum anchors identity. Stratum constrains magnitude. Spectrum resolves structure.

In both systems, fewer than three constraints leave ambiguity: GPS leaves multiple possible locations; UOR leaves multiple possible configurations. When the triad is complete, uncertainty collapses to one unique point, physical in GPS, informational in UOR. The coordinate does not describe the point; the intersection of constraints creates it.

## The Two Fundamental Operations

PRISM is not just a way to label values. It is an algebra: a system of operations that transform values while staying within the coordinate space.

The system rests on two operations:

### Negation

Negation produces the additive inverse: the value that, when added to the original, gives zero. In an 8 bit space, the negation of 1 is 255, because 1 + 255 = 256, which wraps around to 0 in a space of 256 values.

Negation is an **involution**: applying it twice returns to the original. `negate(negate(x)) = x`, always.

### Complement

Complement flips every bit: each 0 becomes 1, each 1 becomes 0. The complement of 42 (`00101010`) is 213 (`11010101`).

Complement is also an involution: `complement(complement(x)) = x`, always.

### The Key Discovery

When you apply negation *after* complement, you always get the next value:

```
negate(complement(42)) = 43
negate(complement(43)) = 44
...
negate(complement(255)) = 0    (wraps around to the start)
```

These two operations, composed, produce a stepping function that visits every value in the space exactly once. The implication is powerful: **the entire space is connected.** Starting from any value, you can reach every other value by combining these two operations.

This is what guarantees PRISM's universality. It is not a system that works for some values but not others. If the data fits in the bits, PRISM can address it.

## Verified Computation

Every time PRISM performs a computation, it produces a **derivation**: a certificate recording the original expression, the canonical form, the result, and a unique ID addressed by content.

The ID comes from the canonical form, not the original expression. Two people who write the same computation in different ways (`XOR(A, B)` and `XOR(B, A)`) get the same certificate. The system recognizes they are equivalent.

This matters for trust. If someone gives you a result and a PRISM derivation, you can verify independently that the computation is correct. You do not need to trust the person, the machine, or the software. The algebra is the proof.

## Verification

PRISM checks its own correctness before producing any output. It verifies that both involutions work correctly, that the critical identity holds, that the successor function traverses the entire space, and that all algebraic laws are internally consistent.

At the 8 bit level (256 values), this check is **exhaustive**: every single case is verified. At higher levels, PRISM verifies via boundary values, structural patterns, and midpoints.

This is a design principle, not a testing strategy. PRISM treats its own correctness as something to be proven, not assumed.

## Finite and Closed

All PRISM operations stay within a bounded space. If you start with a valid value and apply any operation, the result is always another valid value. Nothing can escape.

Think of the globe. No matter which direction you travel, you stay on the surface. PRISM's information space works the same way: it wraps around, closes on itself, and contains every possible state.

In unbounded number systems, operations can produce arbitrarily large results. In PRISM, every result is bounded. Every computation is predictable. Every state is accounted for. For applications where safety is critical, this is the difference between hoping nothing goes wrong and *proving* nothing can.

## Scaling

PRISM scales through quantum levels:

* **Quantum 0**: 1 byte, 8 bits, 256 values. The base level where everything is exhaustively verified.
* **Quantum 1**: 2 bytes, 16 bits, 65,536 values.
* **Quantum 3**: 4 bytes, 32 bits, over 4 billion values.
* There is no upper limit.

The algebra is the same at every level. Everything that works at Quantum 0 works at Quantum 100. The only difference is the size of the space. PRISM computes only what you need. It never has to enumerate every possible value to operate correctly.

## Next Steps

* [examples/quickstart.py](../examples/quickstart.py): See PRISM in action with runnable code
* [ALGEBRA.md](ALGEBRA.md): The mathematical foundation (for those who want the proofs)
* [API.md](API.md): Complete function reference (for those ready to build)
