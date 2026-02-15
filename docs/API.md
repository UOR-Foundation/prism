# API Reference

Complete reference for `prism.py`. All operations accept either `int` or `tuple` of bytes. Integers are reduced mod `cycle`.


## Creating an Engine

```python
from prism import Q0, Q1, Q2, Q3, Q

engine = Q0()     # 8-bit   (256 states)
engine = Q1()     # 16-bit  (65,536 states)
engine = Q(n)     # 8×(n+1) bits
```

| Property | Description |
|---|---|
| `engine.quantum` | Quantum level |
| `engine.width` | Bytes per value (quantum + 1) |
| `engine.bits` | Bits per value (8 × width) |
| `engine.cycle` | Total states (2^bits) |


## Verification

```python
engine.verify() → bool
```

Runs the full coherence check. Exhaustive at Q0 (all 256 states). Raises `CoherenceError` on failure. Called automatically by `emit()` if not already verified.


## Primitive Operations

```python
engine.neg(n)  → Tuple[int, ...]     # additive inverse
engine.bnot(n) → Tuple[int, ...]     # bitwise complement
engine.xor(a, b) → Tuple[int, ...]   # bitwise XOR
engine.band(a, b) → Tuple[int, ...]  # bitwise AND
engine.bor(a, b) → Tuple[int, ...]   # bitwise OR
```

## Derived Operations

```python
engine.succ(n) → Tuple[int, ...]     # successor (n + 1)
engine.pred(n) → Tuple[int, ...]     # predecessor (n - 1)
```


## Triadic Coordinates

```python
engine.triad(n) → Triad
```

Returns a `Triad` with:
* `.datum`: byte tuple
* `.stratum`: popcount per byte
* `.spectrum`: active bit positions per byte
* `.total_stratum`: sum of strata
* `.width`: byte count

```python
engine.stratum(n) → Tuple[int, ...]           # just the stratum
engine.spectrum(n) → Tuple[Tuple[int,...],...]  # just the spectrum
```


## Terms and Derivations

### Build a term

```python
engine.make_term(operation, *operands) → Term
```

Valid operations: `neg`, `bnot`, `xor`, `and`, `or`, `succ`, `pred`. Operands can be integers or nested `Term` objects.

### Canonicalize

```python
engine.canonicalize_term(term) → Term
```

Full normalization: involution cancellation, derived expansion, constant reduction, AC flatten/sort, identity elimination, annihilator reduction, self cancellation, idempotence.

### Evaluate

```python
engine.evaluate(term) → Tuple[int, ...]
```

### Derive (canonicalize + evaluate + certify)

```python
engine.derive(term) → Derivation
```

A `Derivation` contains:
* `.original_term`: as written
* `.canonical_term`: normalized form
* `.result_datum`: evaluated byte tuple
* `.result_iri`: web address of the result
* `.derivation_id`: content addressed certificate ID
* `.metrics`: depth, node count, operation counts


## Correlation

```python
engine.correlate(a, b) → Dict
```

Returns: `fidelity` (0.0 to 1.0), `totalDifference` (Hamming distance), `maxDifference` (total bits), `differenceStratum` (per byte distances).


## Emission

```python
engine.emit(**kwargs) → Dict                  # JSON-LD as dict
engine.emit_json(indent=2, **kwargs) → str    # JSON-LD as string
engine.write(path, **kwargs) → None           # write to file
engine.emit_entity(n) → Dict                  # single datum as JSON-LD
```

`emit()` parameters:

| Parameter | Default | Description |
|---|---|---|
| `sample_size` | auto | Initial sample count |
| `closure_ops` | `[]` | `["not"]`, `["inverse"]`, or both |
| `closure_mode` | `ONE_STEP` | `ONE_STEP`, `FIXED_POINT`, or `GRAPH_CLOSED` |
| `include_derivations` | `None` | List of `Derivation` objects to embed |
| `allow_full_closure` | `False` | Allow full ring enumeration |


## CLI

```bash
python prism.py [OPTIONS]

  -q, --quantum INT       Quantum level (default: 0)
  -o, --output PATH       Output file path
  --closure-ops TEXT       "not" and/or "inverse" (repeatable)
  --sample-size INT       Sample count
  --verbose               Detailed output with demos
```


## Exceptions

| Exception | Cause |
|---|---|
| `CoherenceError` | Algebraic verification failed |
| `ValidationError` | Input format mismatch |
| `ClosureError` | Closure would require full enumeration (guarded) |
