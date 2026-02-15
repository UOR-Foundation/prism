#!/usr/bin/env python3
"""
UOR Framework

A computation engine that IS the specification.
The JSON-LD output is proof of verified computation.

Scales from Quantum 0 (8-bit) to arbitrary Quantum N (8×(N+1) bits).

- T = 3 (triadic coordinates: datum, stratum, spectrum)
- O = 8 (basis generators per quantum level)
- Computation binds the three

===============================================================================
UNIVERSAL ALGEBRA FORMALIZATION
===============================================================================

Carrier Set: Z/(2^bits)Z presented as big-endian byte tuples

Signature Σ (primitive operations):
  - neg  : unary  (additive inverse, -x mod 2^bits)
  - bnot : unary  (bitwise complement, ~x)
  - xor  : binary (bitwise exclusive or)
  - and  : binary (bitwise and)
  - or   : binary (bitwise or)

Surface Syntax Extension:
  The binary ops {xor, and, or} are associative and commutative, so we allow
  n-ary syntax as sugar: xor(a,b,c) ≡ xor(xor(a,b),c). Canonicalization
  flattens and sorts these.

The two primitive INVOLUTIONS are neg and bnot.
  - neg(neg(x)) = x
  - bnot(bnot(x)) = x

DERIVED OPERATIONS (not in primitive signature):
  - succ(x) = neg(bnot(x)) = x + 1 mod 2^bits
  - pred(x) = bnot(neg(x)) = x - 1 mod 2^bits

CRITICAL IDENTITY (Theorem):
  neg(bnot(x)) = succ(x)
  
This means closure under both involutions implies closure under succ,
which generates the entire ring. Therefore:

  THEOREM: No nonempty proper subset S ⊂ Z/(2^bits)Z can be graph-closed
           under both neg and bnot.

===============================================================================
VOCABULARY (Universal Algebra)
===============================================================================

  Term:        Syntax tree built from operation symbols (the signature Σ).
               Example: xor(0x55, 0xAA) as a tree structure.
               
  Evaluation:  Interpreting a term in the UOR algebra to yield a datum.
               The term is syntax; evaluation produces semantics.
               
  Derivation:  A certificate/witness binding {term, operands, result, metrics}.
               This is the provenance record for verified computation.
               Multiple derivations can yield the same datum.
               
  Path/Walk:   Traversal in the emitted graph (following succ/pred/not/inverse edges).
               Graph-theoretic, not syntactic.

===============================================================================
CLOSURE SEMANTICS
===============================================================================

  ONE_STEP:     S ∪ f(S) for each f in closure_ops, applied once from seed only.
                Closes S under each individual involution, but NOT under their
                composition. f(g(x)) may escape S. This is NOT the group closure.
                
  FIXED_POINT:  Iterate until no new nodes appear. For {neg, bnot} together,
                this generates the full ring (via the critical identity).
                GUARDED: raises error for large cycles with both involutions
                unless explicitly confirmed.
                
  GRAPH_CLOSED: Every edge in closure_ops points to a node in S.
                This is fixed-point closure under closure_ops with verification.
                Note: Full graph-closure under ALL edges (including succ/pred)
                would require full ring enumeration for any nonempty set.

===============================================================================
CANONICALIZATION POLICY
===============================================================================

Canonicalization normalizes terms for deterministic derivation IDs and
structural comparison. The following equations are normalized:

  1. Involution cancellation: f(f(x)) → x for f ∈ {neg, bnot}
  2. Derived expansion: succ(x) → neg(bnot(x)), pred(x) → bnot(neg(x))
  3. Constant reduction: integers reduced mod 2^bits
  4. AC flatten+sort: xor/and/or flattened to n-ary, operands sorted
  5. Identity elimination: x xor 0 → x, x and mask → x, x or 0 → x
  6. Annihilator reduction: x and 0 → 0, x or mask → mask
  7. Self-cancellation: x xor x → 0
  8. Idempotence: x and x → x, x or x → x

NOT normalized (would require semantic equality testing):
  - Absorption: x and (x or y) → x
  - Distributivity
  - General term equivalence under the full equational theory

This canonicalization is sufficient for syntactic determinism and common
algebraic identities, but is NOT a complete decision procedure for term
equivalence in the bitwise algebra.

The machine pins registers, computes transformations,
verifies coherence, and emits the proven structure.
"""

import json
import hashlib
from enum import Enum
from typing import Dict, List, Tuple, Optional, Union, Set, Any
from dataclasses import dataclass, field
from collections import Counter

import click


class CoherenceError(Exception):
    """Raised when the machine fails self-verification."""
    pass


class ValidationError(Exception):
    """Raised when input fails validation."""
    pass


class ClosureError(Exception):
    """Raised when closure would cause full ring enumeration unexpectedly."""
    pass


class ClosureMode(Enum):
    """
    Defines how closure is computed for sampled graphs.
    
    ONE_STEP:     S ∪ f(S) for each f - single application from seed only.
                  Closes under each involution individually, but f(g(x)) may escape.
                  This is NOT full closure under the group generated by {f, g}.
    
    FIXED_POINT:  Repeatedly apply until no new nodes appear.
                  For {neg, bnot} together, generates full ring via succ.
                  GUARDED for large cycles.
    
    GRAPH_CLOSED: Fixed-point closure under closure_ops, with verification that
                  every closure_ops edge lands in S. For both involutions on
                  large rings, requires full enumeration (guarded).
    """
    ONE_STEP = "oneStep"
    FIXED_POINT = "fixedPoint"
    GRAPH_CLOSED = "graphClosed"


@dataclass(frozen=True)
class Triad:
    """The three coordinates of any datum."""
    datum: Tuple[int, ...]   # x: value (tuple of bytes)
    stratum: Tuple[int, ...] # y: weight per position
    spectrum: Tuple[Tuple[int, ...], ...]  # z: basis elements per position
    
    @property
    def total_stratum(self) -> int:
        return sum(self.stratum)
    
    @property
    def width(self) -> int:
        return len(self.datum)


@dataclass(frozen=True)
class TermMetrics:
    """
    Structural metrics for a term (syntax tree).
    
    These are properties of the term itself, independent of
    which datum it evaluates to.
    """
    depth: int
    node_count: int
    op_counts: Tuple[Tuple[str, int], ...]
    
    @classmethod
    def from_op_dict(cls, depth: int, node_count: int, op_counts: Dict[str, int]) -> 'TermMetrics':
        """Create from a mutable op_counts dict."""
        return cls(
            depth=depth,
            node_count=node_count,
            op_counts=tuple(sorted(op_counts.items()))
        )
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to JSON-serializable dict."""
        return {
            "depth": self.depth,
            "nodeCount": self.node_count,
            "opCounts": dict(self.op_counts)
        }


@dataclass
class Term:
    """
    A term in the UOR algebra - a syntax tree of operations.
    
    In universal algebra, a term is the syntactic object built from operation
    symbols in the signature. A term is NOT a value until evaluated.
    
    Note: Full canonicalization (including ring-aware constant reduction and
    proper width-dependent sorting) requires UOR.canonicalize_term().
    Term._structural_rewrite() only performs ring-independent rewrites.
    """
    operation: Optional[str]  # None for leaf (constant)
    operands: Tuple[Union['Term', int], ...]  # child terms or constant values
    
    def is_leaf(self) -> bool:
        return self.operation is None
    
    def metrics(self) -> TermMetrics:
        """Compute structural metrics for this term."""
        if self.is_leaf():
            return TermMetrics.from_op_dict(depth=0, node_count=1, op_counts={})
        
        op_counts: Dict[str, int] = {self.operation: 1}
        max_child_depth = 0
        total_nodes = 1
        
        for operand in self.operands:
            if isinstance(operand, Term):
                child_metrics = operand.metrics()
                max_child_depth = max(max_child_depth, child_metrics.depth)
                total_nodes += child_metrics.node_count
                for op, count in child_metrics.op_counts:
                    op_counts[op] = op_counts.get(op, 0) + count
            else:
                total_nodes += 1
        
        return TermMetrics.from_op_dict(
            depth=1 + max_child_depth,
            node_count=total_nodes,
            op_counts=op_counts
        )
    
    def _structural_rewrite(self) -> 'Term':
        """
        Ring-independent structural rewrites only.
        
        Rules applied:
        1. Involution reduction: f(f(x)) → x for f ∈ {neg, bnot}
        2. Derived expansion: succ(x) → neg(bnot(x)), pred(x) → bnot(neg(x))
        3. Associative flattening: nested same-op → n-ary form (NO SORTING)
        
        Sorting and identity elimination require ring context (width/mask)
        and are handled by UOR.canonicalize_term().
        """
        if self.is_leaf():
            return self
        
        # First, rewrite children
        rewritten_operands: List[Union[Term, int]] = []
        for op in self.operands:
            if isinstance(op, Term):
                rewritten_operands.append(op._structural_rewrite())
            else:
                rewritten_operands.append(op)
        
        operation = self.operation
        
        # Rule 2: Expand derived operations
        if operation == "succ":
            inner = rewritten_operands[0]
            return Term("neg", (Term("bnot", (inner,)),))._structural_rewrite()
        elif operation == "pred":
            inner = rewritten_operands[0]
            return Term("bnot", (Term("neg", (inner,)),))._structural_rewrite()
        
        # Rule 1: Involution reduction - f(f(x)) → x
        if operation in ("neg", "bnot") and len(rewritten_operands) == 1:
            inner = rewritten_operands[0]
            if isinstance(inner, Term) and inner.operation == operation:
                inner_operand = inner.operands[0]
                if isinstance(inner_operand, Term):
                    return inner_operand
                else:
                    return Term(None, (inner_operand,))
        
        # Rule 3: Associative flattening (no sorting yet)
        if operation in ("xor", "and", "or"):
            flattened: List[Union[Term, int]] = []
            
            def flatten(t: Union[Term, int]) -> None:
                if isinstance(t, Term) and t.operation == operation:
                    for child in t.operands:
                        flatten(child)
                else:
                    flattened.append(t)
            
            for op in rewritten_operands:
                flatten(op)
            
            rewritten_operands = flattened
        
        return Term(operation, tuple(rewritten_operands))
    
    def canonical_serialize(self, width: int) -> str:
        """
        Deterministic canonical serialization for hashing.
        
        Format: op(arg1,arg2,...) with constants as fixed-width hex.
        Width is the quantum width (number of bytes).
        """
        if self.is_leaf():
            val = self.operands[0]
            if isinstance(val, int):
                hex_digits = width * 2
                mask = (1 << (width * 8)) - 1
                return f"0x{val & mask:0{hex_digits}x}"
            elif isinstance(val, Term):
                return val.canonical_serialize(width)
            else:
                return str(val)
        
        args = ",".join(
            op.canonical_serialize(width) if isinstance(op, Term) 
            else f"0x{op & ((1 << (width * 8)) - 1):0{width * 2}x}"
            for op in self.operands
        )
        return f"{self.operation}({args})"
    
    def __str__(self) -> str:
        if self.is_leaf():
            val = self.operands[0]
            if isinstance(val, int):
                return f"0x{val:x}"
            return str(val)
        args = ", ".join(
            str(op) if isinstance(op, Term) else f"0x{op:x}"
            for op in self.operands
        )
        return f"{self.operation}({args})"


@dataclass
class Derivation:
    """
    A derivation (certificate/witness) binding a term to its evaluation.
    
    This is the provenance record: what was computed, from what, by which rule.
    Multiple derivations can yield the same datum (many terms → one value).
    
    The derivation_id is content-addressed from the CANONICAL term representation
    (fully normalized via UOR.canonicalize_term, not just structural rewrite).
    
    Create via UOR.derive() to ensure proper canonicalization.
    """
    original_term: Term           # The term as originally written
    canonical_term: Term          # Fully canonicalized form (used for ID)
    result_datum: Tuple[int, ...]
    result_iri: str
    metrics: TermMetrics          # Metrics of ORIGINAL term
    quantum: int
    derivation_id: str = field(default="")
    
    def __post_init__(self):
        if not self.derivation_id:
            width = self.quantum + 1
            content = f"{self.canonical_term.canonical_serialize(width)}={self.result_iri}"
            hash_hex = hashlib.sha256(content.encode()).hexdigest()[:16]
            object.__setattr__(self, 'derivation_id', 
                             f"urn:uor:derivation:sha256:{hash_hex}")
    
    def to_jsonld(self, base_iri: str) -> Dict:
        """Convert to JSON-LD representation."""
        width = self.quantum + 1
        
        def term_to_structure(t: Term) -> Dict:
            if t.is_leaf():
                val = t.operands[0]
                if isinstance(val, int):
                    mask = (1 << (width * 8)) - 1
                    return {"@type": "Constant", "value": val & mask}
                return term_to_structure(val)
            return {
                "@type": "TermNode",
                "operation": {"@id": f"{base_iri}op/{t.operation}"},
                "operands": [
                    term_to_structure(op) if isinstance(op, Term) 
                    else {"@type": "Constant", "value": op & ((1 << (width * 8)) - 1)}
                    for op in t.operands
                ]
            }
        
        return {
            "@id": self.derivation_id,
            "@type": "Derivation",
            "originalTerm": term_to_structure(self.original_term),
            "canonicalTerm": term_to_structure(self.canonical_term),
            "result": {"@id": self.result_iri},
            "termMetrics": self.metrics.to_dict()
        }


class UOR:
    """
    Computation engine for arbitrary quantum level.
    
    Quantum 0: 8-bit   (256 states)
    Quantum 1: 16-bit  (65,536 states)
    Quantum 2: 24-bit  (16,777,216 states)
    Quantum 3: 32-bit  (4,294,967,296 states)
    ...
    
    Semantics: Modular ring Z/(2^bits)Z
    All operations reduce mod cycle. This is the canonical interpretation.
    
    SIGNATURE Σ = {neg, bnot, xor, and, or}
    Binary ops are formally binary but accept n-ary syntax (flattened).
    succ/pred are derived: succ = neg∘bnot, pred = bnot∘neg
    
    Does not enumerate. Computes compositionally.
    """
    
    BASE = "https://uor.foundation/u/"
    BYTE_BITS = 8
    BYTE_CYCLE = 256
    
    # Sorted for deterministic output
    SIGNATURE = ("and", "bnot", "neg", "or", "xor")
    UNARY_OPS = frozenset({"neg", "bnot"})
    BINARY_OPS = frozenset({"xor", "and", "or"})
    COMMUTATIVE_OPS = frozenset({"xor", "and", "or"})
    ASSOCIATIVE_OPS = frozenset({"xor", "and", "or"})
    INVOLUTIONS = ("bnot", "neg")
    DERIVED_OPS = ("pred", "succ")
    
    FIXED_POINT_GUARD_THRESHOLD = 65536
    
    def __init__(self, quantum: int = 0):
        if quantum < 0:
            raise ValueError("Quantum must be non-negative")
        self.quantum = quantum
        self.width = quantum + 1
        self.bits = self.BYTE_BITS * self.width
        self.cycle = self.BYTE_CYCLE ** self.width
        self._mask = self.cycle - 1
        self._coherent = False
        self._q0_verified = False
    
    # ═══════════════════════════════════════════════════════════════════════════
    # REPRESENTATION
    # ═══════════════════════════════════════════════════════════════════════════
    
    def _validate_bytes(self, b: Tuple[int, ...]) -> Tuple[int, ...]:
        """Validate byte tuple: correct width, each byte in [0, 255]."""
        if len(b) != self.width:
            raise ValidationError(f"Expected {self.width} bytes, got {len(b)}")
        for i, x in enumerate(b):
            if not isinstance(x, int) or not (0 <= x <= 0xFF):
                raise ValidationError(f"Byte {i} out of range: {x}")
        return b
    
    def _to_bytes(self, n: int) -> Tuple[int, ...]:
        """Convert integer to tuple of bytes (big-endian). Reduces to canonical representative."""
        n &= self._mask
        return tuple(n.to_bytes(self.width, byteorder="big", signed=False))
    
    def _from_bytes(self, b: Tuple[int, ...]) -> int:
        """Convert tuple of bytes to integer (big-endian)."""
        return int.from_bytes(b, byteorder="big")
    
    def _normalize(self, n: Union[int, Tuple[int, ...]]) -> Tuple[int, ...]:
        """Normalize input to validated byte tuple. Integers reduced mod cycle."""
        if isinstance(n, int):
            return self._to_bytes(n)
        return self._validate_bytes(tuple(n))
    
    # ═══════════════════════════════════════════════════════════════════════════
    # PRIMITIVE OPERATIONS (per byte)
    # ═══════════════════════════════════════════════════════════════════════════
    
    @staticmethod
    def _byte_neg(n: int) -> int:
        return (-n) & 0xFF
    
    @staticmethod
    def _byte_not(n: int) -> int:
        return n ^ 0xFF
    
    @staticmethod
    def _byte_popcnt(n: int) -> int:
        return n.bit_count()
    
    @staticmethod
    def _byte_basis(n: int) -> Tuple[int, ...]:
        return tuple(i for i in range(8) if n & (1 << i))
    
    @staticmethod
    def _byte_dots(n: int) -> List[int]:
        return [i + 1 for i in range(8) if n & (1 << i)]
    
    # ═══════════════════════════════════════════════════════════════════════════
    # PRIMITIVE SIGNATURE OPERATIONS
    # ═══════════════════════════════════════════════════════════════════════════
    
    def neg(self, n: Union[int, Tuple[int, ...]]) -> Tuple[int, ...]:
        """Additive inverse (two's complement). PRIMITIVE INVOLUTION."""
        if isinstance(n, tuple):
            val = self._from_bytes(self._validate_bytes(n))
        else:
            val = n
        return self._to_bytes((-val) & self._mask)
    
    def bnot(self, n: Union[int, Tuple[int, ...]]) -> Tuple[int, ...]:
        """Bitwise complement (per byte). PRIMITIVE INVOLUTION."""
        b = self._normalize(n)
        return tuple(self._byte_not(byte) for byte in b)
    
    def xor(self, a: Union[int, Tuple[int, ...]], b: Union[int, Tuple[int, ...]]) -> Tuple[int, ...]:
        """XOR (per byte). PRIMITIVE BINARY: commutative, associative."""
        ba = self._normalize(a)
        bb = self._normalize(b)
        return tuple(x ^ y for x, y in zip(ba, bb))
    
    def band(self, a: Union[int, Tuple[int, ...]], b: Union[int, Tuple[int, ...]]) -> Tuple[int, ...]:
        """AND (per byte). PRIMITIVE BINARY: commutative, associative."""
        ba = self._normalize(a)
        bb = self._normalize(b)
        return tuple(x & y for x, y in zip(ba, bb))
    
    def bor(self, a: Union[int, Tuple[int, ...]], b: Union[int, Tuple[int, ...]]) -> Tuple[int, ...]:
        """OR (per byte). PRIMITIVE BINARY: commutative, associative."""
        ba = self._normalize(a)
        bb = self._normalize(b)
        return tuple(x | y for x, y in zip(ba, bb))
    
    # ═══════════════════════════════════════════════════════════════════════════
    # DERIVED OPERATIONS
    # ═══════════════════════════════════════════════════════════════════════════
    
    def succ(self, n: Union[int, Tuple[int, ...]]) -> Tuple[int, ...]:
        """Increment (DERIVED: succ = neg ∘ bnot). CRITICAL IDENTITY."""
        return self.neg(self.bnot(n))
    
    def pred(self, n: Union[int, Tuple[int, ...]]) -> Tuple[int, ...]:
        """Decrement (DERIVED: pred = bnot ∘ neg)."""
        return self.bnot(self.neg(n))
    
    # ═══════════════════════════════════════════════════════════════════════════
    # TRIADIC COORDINATES
    # ═══════════════════════════════════════════════════════════════════════════
    
    def stratum(self, n: Union[int, Tuple[int, ...]]) -> Tuple[int, ...]:
        """Stratum vector (popcount per byte)."""
        b = self._normalize(n)
        return tuple(self._byte_popcnt(byte) for byte in b)
    
    def spectrum(self, n: Union[int, Tuple[int, ...]]) -> Tuple[Tuple[int, ...], ...]:
        """Spectrum (basis elements per byte)."""
        b = self._normalize(n)
        return tuple(self._byte_basis(byte) for byte in b)
    
    def triad(self, n: Union[int, Tuple[int, ...]]) -> Triad:
        """Complete triadic coordinates."""
        b = self._normalize(n)
        return Triad(datum=b, stratum=self.stratum(b), spectrum=self.spectrum(b))
    
    # ═══════════════════════════════════════════════════════════════════════════
    # TERM CANONICALIZATION (ring-aware)
    # ═══════════════════════════════════════════════════════════════════════════
    
    def _term_sort_key(self, t: Union[Term, int]) -> Tuple[int, str]:
        """Sort key for canonical operand ordering, using correct width."""
        if isinstance(t, int):
            # Constants first, sorted by canonical value
            return (0, f"{t & self._mask:0{self.width * 2}x}")
        else:
            # Terms second, sorted by canonical serialization at correct width
            return (1, t.canonical_serialize(self.width))
    
    def make_term(self, operation: Optional[str], *operands: Union[Term, int]) -> Term:
        """Construct a term from operation and operands."""
        valid_ops = set(self.SIGNATURE) | {"succ", "pred"}
        if operation is not None and operation not in valid_ops:
            raise ValueError(f"Unknown operation: {operation}")
        return Term(operation, tuple(operands))
    
    def canonicalize_term(self, term: Term) -> Term:
        """
        Full ring-aware canonicalization.
        
        Applies all normalization rules documented in the module docstring:
        1. Structural rewrites (involution cancellation, derived expansion, flattening)
        2. Constant reduction mod 2^bits
        3. AC sorting with correct width
        4. Identity elimination (xor 0, and mask, or 0)
        5. Annihilator reduction (and 0, or mask)
        6. Self-cancellation (x xor x → 0)
        7. Idempotence (x and x → x, x or x → x)
        """
        # First: structural rewrites
        rewritten = term._structural_rewrite()
        
        # Then: ring-aware canonicalization
        return self._ring_canonicalize(rewritten)
    
    def _atomize(self, t: Union[Term, int]) -> Union[Term, int]:
        """
        Collapse leaf-constant Terms to raw ints.
        
        This ensures representation-invariant AC results:
        xor(Term(None,(5,)), 5) will cancel correctly because both
        become the same int representation before deduplication.
        """
        if isinstance(t, Term) and t.is_leaf() and len(t.operands) == 1:
            v = t.operands[0]
            if isinstance(v, int):
                return v & self._mask
        return t
    
    def _ring_canonicalize(self, term: Term) -> Term:
        """Ring-aware canonicalization with correct width."""
        if term.is_leaf():
            val = term.operands[0]
            if isinstance(val, int):
                return Term(None, (val & self._mask,))
            elif isinstance(val, Term):
                return self._ring_canonicalize(val)
            return term
        
        # Recursively canonicalize operands
        canonical_operands: List[Union[Term, int]] = []
        for op in term.operands:
            if isinstance(op, Term):
                canonical_operands.append(self._ring_canonicalize(op))
            else:
                canonical_operands.append(op & self._mask)
        
        # Atomize: collapse leaf-constant Terms to ints for representation-invariant AC
        canonical_operands = [self._atomize(op) for op in canonical_operands]
        
        operation = term.operation
        zero = 0
        mask = self._mask
        
        # Apply algebraic reductions based on operation
        if operation == "xor":
            canonical_operands = self._canonicalize_xor(canonical_operands, zero)
        elif operation == "and":
            canonical_operands = self._canonicalize_and(canonical_operands, zero, mask)
        elif operation == "or":
            canonical_operands = self._canonicalize_or(canonical_operands, zero, mask)
        
        # Handle results after reduction
        if isinstance(canonical_operands, Term):
            return canonical_operands  # Reduced to a single term
        if isinstance(canonical_operands, int):
            return Term(None, (canonical_operands,))  # Reduced to constant
        
        # Sort operands for AC operations using correct width
        if operation in ("xor", "and", "or") and isinstance(canonical_operands, list):
            canonical_operands.sort(key=self._term_sort_key)
        
        return Term(operation, tuple(canonical_operands))
    
    def _canonicalize_xor(self, operands: List[Union[Term, int]], zero: int) -> Union[List[Union[Term, int]], Term, int]:
        """
        Canonicalize XOR operands.
        - Remove zeros (identity)
        - Cancel pairs (x xor x = 0)
        """
        # Remove zeros
        filtered = [op for op in operands if not self._is_constant(op, zero)]
        
        # Cancel pairs: count occurrences, keep only odd counts
        term_counts: Counter = Counter()
        for op in filtered:
            key = self._canonical_key(op)
            term_counts[key] += 1
        
        result: List[Union[Term, int]] = []
        seen_keys: Set[str] = set()
        for op in filtered:
            key = self._canonical_key(op)
            if key not in seen_keys:
                seen_keys.add(key)
                if term_counts[key] % 2 == 1:  # Odd count: keep one
                    result.append(op)
        
        if not result:
            return zero
        if len(result) == 1:
            return result[0] if isinstance(result[0], Term) else result[0]
        return result
    
    def _canonicalize_and(self, operands: List[Union[Term, int]], zero: int, mask: int) -> Union[List[Union[Term, int]], Term, int]:
        """
        Canonicalize AND operands.
        - x and 0 = 0 (annihilator)
        - Remove masks (identity)
        - Idempotence: x and x = x
        """
        # Check for annihilator
        if any(self._is_constant(op, zero) for op in operands):
            return zero
        
        # Remove masks (identity)
        filtered = [op for op in operands if not self._is_constant(op, mask)]
        
        # Idempotence: keep only unique operands
        result: List[Union[Term, int]] = []
        seen_keys: Set[str] = set()
        for op in filtered:
            key = self._canonical_key(op)
            if key not in seen_keys:
                seen_keys.add(key)
                result.append(op)
        
        if not result:
            return mask
        if len(result) == 1:
            return result[0] if isinstance(result[0], Term) else result[0]
        return result
    
    def _canonicalize_or(self, operands: List[Union[Term, int]], zero: int, mask: int) -> Union[List[Union[Term, int]], Term, int]:
        """
        Canonicalize OR operands.
        - x or mask = mask (annihilator)
        - Remove zeros (identity)
        - Idempotence: x or x = x
        """
        # Check for annihilator
        if any(self._is_constant(op, mask) for op in operands):
            return mask
        
        # Remove zeros (identity)
        filtered = [op for op in operands if not self._is_constant(op, zero)]
        
        # Idempotence: keep only unique operands
        result: List[Union[Term, int]] = []
        seen_keys: Set[str] = set()
        for op in filtered:
            key = self._canonical_key(op)
            if key not in seen_keys:
                seen_keys.add(key)
                result.append(op)
        
        if not result:
            return zero
        if len(result) == 1:
            return result[0] if isinstance(result[0], Term) else result[0]
        return result
    
    def _is_constant(self, t: Union[Term, int], value: int) -> bool:
        """Check if a term/int equals a constant value."""
        if isinstance(t, int):
            return (t & self._mask) == value
        if t.is_leaf() and len(t.operands) == 1:
            v = t.operands[0]
            return isinstance(v, int) and (v & self._mask) == value
        return False
    
    def _canonical_key(self, t: Union[Term, int]) -> str:
        """
        Get canonical string key for a term/int (for deduplication).
        
        Leaf-constant terms are treated as constants to ensure proper
        cancellation: xor(0x05, Term(None,(0x05,))) → 0
        """
        if isinstance(t, int):
            return f"int:{t & self._mask:0{self.width * 2}x}"
        # Treat leaf-constant terms as plain constants
        if t.is_leaf() and len(t.operands) == 1 and isinstance(t.operands[0], int):
            v = t.operands[0] & self._mask
            return f"int:{v:0{self.width * 2}x}"
        return f"term:{t.canonical_serialize(self.width)}"
    
    # ═══════════════════════════════════════════════════════════════════════════
    # TERM EVALUATION AND DERIVATION
    # ═══════════════════════════════════════════════════════════════════════════
    
    def evaluate(self, term: Term) -> Tuple[int, ...]:
        """Evaluate a term to produce a datum."""
        if term.is_leaf():
            val = term.operands[0]
            if isinstance(val, int):
                return self._normalize(val)
            elif isinstance(val, Term):
                return self.evaluate(val)
            else:
                return self._normalize(val)
        
        evaluated = []
        for op in term.operands:
            if isinstance(op, Term):
                evaluated.append(self.evaluate(op))
            else:
                evaluated.append(self._normalize(op))
        
        op = term.operation
        if op == "neg":
            return self.neg(evaluated[0])
        elif op == "bnot":
            return self.bnot(evaluated[0])
        elif op == "xor":
            result = evaluated[0]
            for other in evaluated[1:]:
                result = self.xor(result, other)
            return result
        elif op == "and":
            result = evaluated[0]
            for other in evaluated[1:]:
                result = self.band(result, other)
            return result
        elif op == "or":
            result = evaluated[0]
            for other in evaluated[1:]:
                result = self.bor(result, other)
            return result
        elif op == "succ":
            return self.succ(evaluated[0])
        elif op == "pred":
            return self.pred(evaluated[0])
        else:
            raise ValueError(f"Unknown operation: {op}")
    
    def derive(self, term: Term) -> Derivation:
        """
        Create a derivation (certificate) for a term.
        
        The derivation ID is computed from the CANONICAL form of the term,
        so semantically equivalent terms produce the same derivation ID.
        """
        canonical = self.canonicalize_term(term)
        result = self.evaluate(canonical)
        return Derivation(
            original_term=term,
            canonical_term=canonical,
            result_datum=result,
            result_iri=self._iri(result),
            metrics=term.metrics(),
            quantum=self.quantum
        )
    
    # ═══════════════════════════════════════════════════════════════════════════
    # COHERENCE VERIFICATION
    # ═══════════════════════════════════════════════════════════════════════════
    
    def _verify_q0_exhaustive(self) -> bool:
        """Exhaustively verify Quantum 0 (256 states)."""
        from math import comb
        
        q0 = UOR(quantum=0)
        
        for n in range(256):
            b = (n,)
            
            if q0.bnot(q0.bnot(b)) != b:
                raise CoherenceError(f"NOT not involution at {n}")
            if q0.neg(q0.neg(b)) != b:
                raise CoherenceError(f"NEG not involution at {n}")
            
            expected_succ = q0._to_bytes((n + 1) & 0xFF)
            if q0.succ(b) != expected_succ:
                raise CoherenceError(f"succ derivation failed at {n}")
            if q0.neg(q0.bnot(b)) != expected_succ:
                raise CoherenceError(f"Critical identity failed at {n}")
            
            expected_pred = q0._to_bytes((n - 1) & 0xFF)
            if q0.pred(b) != expected_pred:
                raise CoherenceError(f"pred derivation failed at {n}")
            if q0.bnot(q0.neg(b)) != expected_pred:
                raise CoherenceError(f"Critical identity (pred) failed at {n}")
            
            if q0.succ(q0.pred(b)) != b:
                raise CoherenceError(f"succ(pred({n})) != {n}")
            if q0.pred(q0.succ(b)) != b:
                raise CoherenceError(f"pred(succ({n})) != {n}")
            
            if q0.xor(b, q0.bnot(b)) != (0xFF,):
                raise CoherenceError(f"XOR complement failed at {n}")
            if q0.xor(b, b) != (0,):
                raise CoherenceError(f"XOR self-annihilation failed at {n}")
            
            neg_n = q0.neg(b)
            total = (n + q0._from_bytes(neg_n)) & 0xFF
            if total != 0:
                raise CoherenceError(f"Additive inverse failed at {n}")
            
            if q0._byte_popcnt(n) + q0._byte_popcnt(q0._byte_not(n)) != 8:
                raise CoherenceError(f"Stratum symmetry failed at {n}")
            
            recomposed = 0
            for bit in q0._byte_basis(n):
                recomposed |= (1 << bit)
            if recomposed != n:
                raise CoherenceError(f"Basis recomposition failed at {n}")
        
        visited = set()
        current = (0,)
        for _ in range(256):
            if current in visited:
                raise CoherenceError("Q0 cycle collapsed")
            visited.add(current)
            current = q0.succ(current)
        if current != (0,):
            raise CoherenceError("Q0 cycle did not return to origin")
        
        counts = [0] * 9
        for n in range(256):
            counts[q0._byte_popcnt(n)] += 1
        for k in range(9):
            if counts[k] != comb(8, k):
                raise CoherenceError("Q0 stratum distribution failed")
        
        return True
    
    def _verify_homomorphism(self, test_values: List[Tuple[int, ...]]) -> bool:
        """Verify bytewise ops match modular integer arithmetic."""
        for b in test_values:
            n = self._from_bytes(b)
            
            if self.succ(b) != self._to_bytes((n + 1) % self.cycle):
                raise CoherenceError(f"succ homomorphism failed at {b}")
            if self.pred(b) != self._to_bytes((n - 1) % self.cycle):
                raise CoherenceError(f"pred homomorphism failed at {b}")
            if self.neg(b) != self._to_bytes((-n) % self.cycle):
                raise CoherenceError(f"neg homomorphism failed at {b}")
            if self.bnot(b) != self._to_bytes((~n) & self._mask):
                raise CoherenceError(f"bnot homomorphism failed at {b}")
        
        return True
    
    def _verify_binary_homomorphism(self, test_values: List[Tuple[int, ...]]) -> bool:
        """Verify binary bitwise ops match integer bitwise ops."""
        k = min(32, len(test_values))
        subset = test_values[:k] if k == len(test_values) else \
                 [test_values[(i * len(test_values)) // k] for i in range(k)]
        
        for a in subset:
            na = self._from_bytes(a)
            for b in subset:
                nb = self._from_bytes(b)
                
                if self.xor(a, b) != self._to_bytes((na ^ nb) & self._mask):
                    raise CoherenceError(f"xor homomorphism failed")
                if self.band(a, b) != self._to_bytes((na & nb) & self._mask):
                    raise CoherenceError(f"and homomorphism failed")
                if self.bor(a, b) != self._to_bytes((na | nb) & self._mask):
                    raise CoherenceError(f"or homomorphism failed")
        
        return True
    
    def _verify_critical_identity(self, test_values: List[Tuple[int, ...]]) -> bool:
        """Verify neg(bnot(x)) = succ(x), bnot(neg(x)) = pred(x)."""
        for b in test_values:
            n = self._from_bytes(b)
            
            if self.neg(self.bnot(b)) != self._to_bytes((n + 1) & self._mask):
                raise CoherenceError(f"Critical identity (succ) failed at {b}")
            if self.bnot(self.neg(b)) != self._to_bytes((n - 1) & self._mask):
                raise CoherenceError(f"Critical identity (pred) failed at {b}")
        
        return True
    
    def _verify_composition_laws(self) -> bool:
        """Verify that composition preserves coherence."""
        zero = tuple([0] * self.width)
        ones = tuple([0xFF] * self.width)
        mid = tuple([0x55] * self.width)
        alt = tuple([0xAA] * self.width)
        
        test_values = [zero, ones, mid, alt]
        
        for i in range(min(16, self.cycle)):
            test_values.append(self._to_bytes(i))
            test_values.append(self._to_bytes(self.cycle - 1 - i))
        
        if self.cycle > 256:
            for i in range(16):
                test_values.append(self._to_bytes((self.cycle // 2) + i))
                test_values.append(self._to_bytes((self.cycle // 4) + i))
        
        for b in test_values:
            if self.bnot(self.bnot(b)) != b:
                raise CoherenceError(f"NOT not involution at {b}")
            if self.neg(self.neg(b)) != b:
                raise CoherenceError(f"NEG not involution at {b}")
            if self.succ(self.pred(b)) != b:
                raise CoherenceError(f"succ(pred({b})) != {b}")
            if self.pred(self.succ(b)) != b:
                raise CoherenceError(f"pred(succ({b})) != {b}")
            if self.xor(b, self.bnot(b)) != ones:
                raise CoherenceError(f"XOR complement failed at {b}")
            if self.xor(b, b) != zero:
                raise CoherenceError(f"XOR self-annihilation failed at {b}")
            
            s1 = sum(self.stratum(b))
            s2 = sum(self.stratum(self.bnot(b)))
            if s1 + s2 != 8 * self.width:
                raise CoherenceError(f"Stratum symmetry failed at {b}")
        
        max_val = tuple([0xFF] * self.width)
        zero_val = tuple([0] * self.width)
        if self.succ(max_val) != zero_val:
            raise CoherenceError("Carry propagation failed at max")
        if self.pred(zero_val) != max_val:
            raise CoherenceError("Borrow propagation failed at zero")
        
        for b in test_values:
            neg_b = self.neg(b)
            if (self._from_bytes(b) + self._from_bytes(neg_b)) & self._mask != 0:
                raise CoherenceError(f"Additive inverse failed at {b}")
        
        self._verify_homomorphism(test_values)
        self._verify_binary_homomorphism(test_values)
        self._verify_critical_identity(test_values)
        
        return True
    
    def verify(self) -> bool:
        """Verify coherence at this quantum level."""
        if not self._q0_verified:
            self._verify_q0_exhaustive()
            self._q0_verified = True
        
        if self.quantum == 0:
            self._coherent = True
            return True
        
        self._verify_composition_laws()
        self._coherent = True
        return True
    
    # ═══════════════════════════════════════════════════════════════════════════
    # EMISSION
    # ═══════════════════════════════════════════════════════════════════════════
    
    def _codepoint(self, byte: int) -> int:
        return 0x2800 + byte
    
    def _glyph(self, b: Tuple[int, ...]) -> str:
        return ''.join(chr(self._codepoint(byte)) for byte in b)
    
    def _iri(self, b: Tuple[int, ...]) -> str:
        codes = ''.join(f"U{self._codepoint(byte):04X}" for byte in b)
        return f"{self.BASE}{codes}"
    
    def _uplus(self, b: Tuple[int, ...]) -> str:
        return ' '.join(f"U+{self._codepoint(byte):04X}" for byte in b)
    
    def _entity(self, n: Union[int, Tuple[int, ...]], 
                derivation_ids: Optional[List[str]] = None) -> Dict:
        """Compute complete entity for datum."""
        b = self._normalize(n)
        t = self.triad(b)
        
        basis_by_position = []
        for pos, byte_spectrum in enumerate(t.spectrum):
            for bit in byte_spectrum:
                basis_byte = tuple(
                    (1 << bit) if i == pos else 0 
                    for i in range(self.width)
                )
                basis_by_position.append(self._iri(basis_byte))
        
        entity: Dict[str, Any] = {
            "@id": self._iri(b),
            "@type": "Datum",
            "quantum": self.quantum,
            "value": self._from_bytes(b),
            "bytes": list(b),
            "stratum": list(t.stratum),
            "totalStratum": t.total_stratum,
            "spectrum": [list(s) for s in t.spectrum],
            "glyph": self._glyph(b),
            "codepoints": self._uplus(b),
            "dots": [self._byte_dots(byte) for byte in b],
            "inverse": self._iri(self.neg(b)),
            "not": self._iri(self.bnot(b)),
            "succ": self._iri(self.succ(b)),
            "pred": self._iri(self.pred(b)),
            "basis": basis_by_position,
        }
        
        if derivation_ids:
            entity["derivations"] = [{"@id": did} for did in derivation_ids]
        
        return entity
    
    def _context(self) -> Dict:
        return {
            "@base": self.BASE,
            "@vocab": self.BASE,
            "xsd": "http://www.w3.org/2001/XMLSchema#",
            "uor": self.BASE,
            "value": {"@type": "xsd:nonNegativeInteger"},
            "quantum": {"@type": "xsd:nonNegativeInteger"},
            "totalStratum": {"@type": "xsd:nonNegativeInteger"},
            "basis": {"@type": "@id", "@container": "@list"},
            "succ": {"@type": "@id"},
            "pred": {"@type": "@id"},
            "inverse": {"@type": "@id"},
            "not": {"@type": "@id"},
            "result": {"@type": "@id"},
            "operation": {"@type": "@id"},
            "derivations": {"@type": "@id", "@container": "@set"},
        }
    
    def _metadata(
        self,
        initial_size: int,
        closure_added: int,
        datum_count: int,
        closure_ops: List[str],
        closure_mode: ClosureMode,
        full_closed: bool,
        not_closed_under: List[str],
        derivation_count: int = 0,
        graph_node_count: int = 0
    ) -> Dict:
        result: Dict[str, Any] = {
            "@id": f"{self.BASE}proof/Q{self.quantum}",
            "@type": "CoherenceProof",
            "quantum": self.quantum,
            "width": self.width,
            "bits": self.bits,
            "cycle": self.cycle if self.cycle <= 2**32 else f"2^{self.bits}",
            "signature": list(self.SIGNATURE),
            "primitiveInvolutions": list(self.INVOLUTIONS),
            "derivedOperations": list(self.DERIVED_OPS),
            "basisPerByte": 8,
            "strataPerByte": 9,
            "totalStrata": 8 * self.width + 1,
            "verified": self._coherent,
            "criticalIdentity": "neg(bnot(x)) = succ(x)",
            "closureMode": closure_mode.value,
            "closureOps": sorted(closure_ops),
            "graphFullyClosed": full_closed,
            "notClosedUnder": sorted(not_closed_under),
            "semantics": "Z/(2^bits)Z modular ring",
            "datumCount": datum_count,
            "graphNodeCount": graph_node_count,
        }
        
        if not full_closed:
            result["initialSampleSize"] = initial_size
            result["closureAdded"] = closure_added
        
        if derivation_count > 0:
            result["derivationCount"] = derivation_count
        
        return result
    
    def emit_entity(self, n: Union[int, Tuple[int, ...]]) -> Dict:
        if not self._coherent:
            self.verify()
        return {"@context": self._context(), **self._entity(n)}
    
    def emit(
        self, 
        sample_size: Optional[int] = None, 
        closure_ops: Optional[List[str]] = None,
        closure_mode: ClosureMode = ClosureMode.ONE_STEP,
        include_derivations: Optional[List[Derivation]] = None,
        allow_full_closure: bool = False
    ) -> Dict:
        """
        Emit complete JSON-LD.
        
        Args:
            sample_size: Initial sample size before closure. None = auto.
            closure_ops: Operations to close under: [], ["not"], ["inverse"], 
                         or ["not", "inverse"].
            closure_mode: How to compute closure.
            include_derivations: Optional derivations to include in @graph.
            allow_full_closure: If False, raises ClosureError when closure
                               would require full ring enumeration.
        """
        if closure_ops is None:
            closure_ops = []
        
        valid_closure_ops = {"not", "inverse"}
        for op in closure_ops:
            if op not in valid_closure_ops:
                raise ValueError(f"closure_ops must be subset of {valid_closure_ops}")
        
        both_involutions = set(closure_ops) == {"not", "inverse"}
        
        # Guard against accidental full enumeration
        if both_involutions and closure_mode in (ClosureMode.FIXED_POINT, ClosureMode.GRAPH_CLOSED):
            if self.cycle > self.FIXED_POINT_GUARD_THRESHOLD and not allow_full_closure:
                raise ClosureError(
                    f"Closure under both involutions requires full ring enumeration "
                    f"({self.cycle:,} elements). Set allow_full_closure=True to proceed."
                )
        
        if not self._coherent:
            self.verify()
        
        # Determine initial sample size
        if sample_size is None:
            sample_size = self.cycle if self.cycle <= 65536 else 1024
        
        initial_target = min(sample_size, self.cycle)
        
        # Force full enumeration for graph-closed with both involutions
        if both_involutions and closure_mode == ClosureMode.GRAPH_CLOSED:
            initial_target = self.cycle
        
        if initial_target == self.cycle:
            samples = set(range(self.cycle))
            full_closed = True
            not_closed_under: List[str] = []
            initial_size = self.cycle
            closure_added = 0
        else:
            full_closed = False
            samples: Set[int] = set()
            
            # Seed with boundaries and patterns
            samples.add(0)
            samples.add(self.cycle - 1)
            
            for i in range(self.width):
                for v in [0x01, 0x80, 0xFF]:
                    b = [0] * self.width
                    b[i] = v
                    samples.add(self._from_bytes(tuple(b)))
            
            for v in [0x00, 0x55, 0xAA, 0xFF]:
                samples.add(self._from_bytes(tuple([v] * self.width)))
            
            if len(samples) >= initial_target:
                samples = set(sorted(samples)[:initial_target])
            else:
                remaining = initial_target - len(samples)
                if remaining > 0:
                    step = max(1, self.cycle // remaining)
                    for i in range(0, self.cycle, step):
                        samples.add(i)
                        if len(samples) >= initial_target:
                            break
            
            # Seed derivation result datums BEFORE closure (proof soundness)
            # This ensures GRAPH_CLOSED claims are accurate
            if include_derivations:
                for d in include_derivations:
                    samples.add(self._from_bytes(d.result_datum))
            
            # NOW set initial_size (truthful: includes derivation results)
            initial_size = len(samples)
            closure_added = 0
            
            # Apply closure based on mode
            if closure_ops:
                def apply_ops(s: Set[int]) -> Set[int]:
                    new = set()
                    for n in s:
                        for op in closure_ops:
                            target = (~n if op == "not" else -n) & self._mask
                            if target not in s:
                                new.add(target)
                    return new
                
                if closure_mode == ClosureMode.ONE_STEP:
                    new_targets = apply_ops(samples)
                    samples.update(new_targets)
                    closure_added = len(new_targets)
                
                elif closure_mode in (ClosureMode.FIXED_POINT, ClosureMode.GRAPH_CLOSED):
                    # Fixed-point iteration
                    changed = True
                    max_iters = self.cycle
                    iters = 0
                    while changed and iters < max_iters:
                        new_targets = apply_ops(samples)
                        if not new_targets:
                            changed = False
                        else:
                            samples.update(new_targets)
                            closure_added += len(new_targets)
                        iters += 1
                    
                    # For GRAPH_CLOSED, verify referential integrity
                    if closure_mode == ClosureMode.GRAPH_CLOSED:
                        for n in samples:
                            for op in closure_ops:
                                target = (~n if op == "not" else -n) & self._mask
                                if target not in samples:
                                    raise CoherenceError(
                                        f"Graph not closed: {op}({n}) = {target} not in S"
                                    )
            
            # Check if closure expanded to full ring
            if len(samples) == self.cycle:
                full_closed = True
                not_closed_under = []
            else:
                # Determine what we're not closed under
                not_closed_under = []
                
                # Check involutions
                for op_name in ["inverse", "not"]:
                    if op_name not in closure_ops:
                        not_closed_under.append(op_name)
                    else:
                        # Verify closure (should be closed if we did fixed-point/graph-closed)
                        for n in samples:
                            target = (-n if op_name == "inverse" else ~n) & self._mask
                            if target not in samples:
                                not_closed_under.append(op_name)
                                break
                
                # succ/pred can never be closed for proper subsets
                not_closed_under.extend(["pred", "succ"])
        
        # Build derivation index: datum IRI → list of derivation IDs
        derivation_index: Dict[str, List[str]] = {}
        if include_derivations:
            for d in include_derivations:
                iri = d.result_iri
                if iri not in derivation_index:
                    derivation_index[iri] = []
                derivation_index[iri].append(d.derivation_id)
        
        # Build graph with datum entities
        graph: List[Dict] = []
        for n in sorted(samples):
            b = self._to_bytes(n)
            iri = self._iri(b)
            derivation_ids = derivation_index.get(iri)
            graph.append(self._entity(n, derivation_ids))
        
        # Add derivation nodes to graph
        if include_derivations:
            for d in include_derivations:
                graph.append(d.to_jsonld(self.BASE))
        
        return {
            "@context": self._context(),
            "proof": self._metadata(
                initial_size=initial_size,
                closure_added=closure_added,
                datum_count=len(samples),
                closure_ops=closure_ops,
                closure_mode=closure_mode,
                full_closed=full_closed,
                not_closed_under=not_closed_under,
                derivation_count=len(include_derivations) if include_derivations else 0,
                graph_node_count=len(graph)
            ),
            "@graph": graph,
        }
    
    def emit_json(self, indent: int = 2, **kwargs) -> str:
        return json.dumps(self.emit(**kwargs), indent=indent, ensure_ascii=False)
    
    def write(self, path: str, **kwargs) -> None:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(self.emit_json(**kwargs))
    
    # ═══════════════════════════════════════════════════════════════════════════
    # CORRELATION
    # ═══════════════════════════════════════════════════════════════════════════
    
    def correlate(self, a: Union[int, Tuple[int, ...]], b: Union[int, Tuple[int, ...]]) -> Dict:
        """Measure correlation via XOR-stratum (Hamming distance)."""
        ba = self._normalize(a)
        bb = self._normalize(b)
        na = self._from_bytes(ba)
        nb = self._from_bytes(bb)
        
        diff_int = (na ^ nb) & self._mask
        total_diff = diff_int.bit_count()
        max_stratum = self.bits
        fidelity = 1.0 - (total_diff / max_stratum)
        
        diff_bytes = self._to_bytes(diff_int)
        diff_stratum = tuple(byte.bit_count() for byte in diff_bytes)
        
        return {
            "a": self._glyph(ba),
            "b": self._glyph(bb),
            "difference": self._glyph(diff_bytes),
            "differenceStratum": list(diff_stratum),
            "totalDifference": total_diff,
            "maxDifference": max_stratum,
            "fidelity": fidelity,
        }


# ═══════════════════════════════════════════════════════════════════════════════
# FACTORY
# ═══════════════════════════════════════════════════════════════════════════════

def Q0() -> UOR: return UOR(quantum=0)
def Q1() -> UOR: return UOR(quantum=1)
def Q2() -> UOR: return UOR(quantum=2)
def Q3() -> UOR: return UOR(quantum=3)
def Q(n: int) -> UOR: return UOR(quantum=n)


# ═══════════════════════════════════════════════════════════════════════════════
# ENTRY POINT
# ═══════════════════════════════════════════════════════════════════════════════

@click.command()
@click.option('-q', '--quantum', default=0, type=int,
              help='Quantum level (0=8-bit, 1=16-bit, etc.)')
@click.option('-o', '--output', type=click.Path(),
              help='Output file path (default: uor_q{quantum}.jsonld)')
@click.option('--closure-ops', multiple=True, type=click.Choice(['not', 'inverse']),
              help='Closure operations (can be specified multiple times)')
@click.option('--sample-size', type=int, default=None,
              help='Sample size (auto if not specified)')
@click.option('--verbose', is_flag=True,
              help='Show detailed output including demos')
def main(quantum: int, output: Optional[str], closure_ops: tuple,
         sample_size: Optional[int], verbose: bool):
    """
    UOR Framework - Generate JSON-LD output for verified computation.

    Scales from Quantum 0 (8-bit) to arbitrary Quantum N (8×(N+1) bits).
    """
    uor = UOR(quantum=quantum)

    # Determine output path
    output_path = output or f"uor_q{quantum}.jsonld"

    # Convert closure_ops tuple to list, default to ["not"]
    closure_list = list(closure_ops) if closure_ops else ["not"]

    if verbose:
        click.echo("UOR Framework")
        click.echo("=" * 60)
        click.echo(f"Quantum:            {uor.quantum}")
        click.echo(f"Width:              {uor.width} byte(s)")
        click.echo(f"Bits:               {uor.bits}")
        click.echo(f"Cycle:              {uor.cycle:,}" if uor.cycle <= 2**32 else f"Cycle:              2^{uor.bits}")
        click.echo()
        click.echo("Algebraic Structure:")
        click.echo(f"  Signature Σ:      {{{', '.join(uor.SIGNATURE)}}}")
        click.echo(f"  Involutions:      {{{', '.join(uor.INVOLUTIONS)}}}")
        click.echo(f"  Derived ops:      succ = neg∘bnot, pred = bnot∘neg")
        click.echo()

    if verbose:
        click.echo("Verifying coherence...")

    try:
        uor.verify()
        if verbose:
            click.echo("  ✓ All coherence checks passed")
            click.echo("  ✓ Critical identity verified: neg(bnot(x)) = succ(x)")
    except CoherenceError as e:
        click.echo(f"  ✗ Coherence failed: {e}", err=True)
        raise SystemExit(1)

    derivations = None
    if verbose:
        click.echo()
        click.echo("Canonicalization demo:")

        # Test that semantically equivalent terms get same derivation ID
        t1 = uor.make_term("xor", 0x55, 0, 0xAA)      # with zero
        t2 = uor.make_term("xor", 0x55, 0xAA)          # without zero
        t3 = uor.make_term("xor", 0xAA, 0x55)          # different order
        t4 = uor.make_term("xor", 0x55, 0x55, 0xAA)    # with self-cancellation

        d1, d2, d3, d4 = uor.derive(t1), uor.derive(t2), uor.derive(t3), uor.derive(t4)

        click.echo(f"  t1 = xor(0x55, 0, 0xAA)      → {d1.canonical_term}")
        click.echo(f"  t2 = xor(0x55, 0xAA)         → {d2.canonical_term}")
        click.echo(f"  t3 = xor(0xAA, 0x55)         → {d3.canonical_term}")
        click.echo(f"  t4 = xor(0x55, 0x55, 0xAA)   → {d4.canonical_term}")
        click.echo()
        click.echo(f"  IDs equal (t1==t2==t3): {d1.derivation_id == d2.derivation_id == d3.derivation_id}")
        click.echo(f"  t4 different (0x55 xor 0x55 = 0): {d4.derivation_id != d1.derivation_id}")
        click.echo(f"  t4 result: {uor._glyph(d4.result_datum)}")

        # Idempotence demo
        click.echo()
        click.echo("Idempotence demo:")
        t_and = uor.make_term("and", 0x55, 0x55, 0xFF)
        t_or = uor.make_term("or", 0xAA, 0xAA, 0x00)
        d_and, d_or = uor.derive(t_and), uor.derive(t_or)
        click.echo(f"  and(0x55, 0x55, 0xFF) → {d_and.canonical_term} = {uor._glyph(d_and.result_datum)}")
        click.echo(f"  or(0xAA, 0xAA, 0x00)  → {d_or.canonical_term} = {uor._glyph(d_or.result_datum)}")

        derivations = [d1, d2, d3, d4, d_and, d_or]

        click.echo()
        click.echo("Emitting with derivations...")

        if quantum <= 1:
            result = uor.emit(closure_ops=closure_list, include_derivations=derivations,
                            sample_size=sample_size)
        else:
            result = uor.emit(sample_size=sample_size or 100, closure_ops=closure_list,
                            include_derivations=derivations)

        click.echo(f"  Graph nodes:      {len(result['@graph'])}")
        click.echo(f"  Derivations:      {result['proof']['derivationCount']}")

        # Check that derivations are linked to datums
        datum_with_derivs = [n for n in result['@graph'] if n.get('@type') == 'Datum' and 'derivations' in n]
        click.echo(f"  Datums with derivs: {len(datum_with_derivs)}")

    # Write output
    include_derivs = derivations if (verbose and quantum <= 1) else None
    uor.write(output_path, closure_ops=closure_list, sample_size=sample_size,
              include_derivations=include_derivs)
    click.echo(f"Written to {output_path}")

    if verbose:
        click.echo()
        click.echo("Critical identity:")
        x = 0x55
        x_bytes = uor._to_bytes(x)
        click.echo(f"  x = 0x{x:02X}")
        click.echo(f"  neg(bnot(x)) = {uor.neg(uor.bnot(x_bytes))}")
        click.echo(f"  succ(x)      = {uor.succ(x_bytes)}")
        click.echo(f"  Equal:       {uor.neg(uor.bnot(x_bytes)) == uor.succ(x_bytes)}")


if __name__ == "__main__":
    main()
