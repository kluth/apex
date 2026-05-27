# APEX Compiler Architecture

## Overview
The APEX compiler is responsible for translating human-readable `.apex` source text into a data-oriented **Anatomy Intermediate Representation (AIR)** ready for the XPBD solver.

## Pipeline Stages

### 1. Lexical Analysis (`Lexer`)
- **Input:** Raw string.
- **Output:** Stream of `Token` variants.
- **Responsibility:** Categorizes text into keywords, identifiers, numbers, and symbols. Handles whitespace and pragma/comment skipping.

### 2. Syntactic Analysis (`Parser`)
- **Architecture:** Recursive-Descent.
- **Responsibility:** Consumes tokens according to the EBNF production rules.
- **Output:** **Abstract Syntax Tree (AST)** composed of `Bone`, `Joint`, `Muscle`, and `Synapse` aggregates.

### 3. Biological Validation (`Validator`)
- **Responsibility:** Enforces domain-specific invariants that cannot be captured by the grammar alone (e.g., ID uniqueness).
- **Architecture:** Rule-based validation pass.

### 4. Lowering (`Lowering Pass`)
- **Input:** AST.
- **Output:** **AIR Topology**.
- **Responsibility:** Flattens the hierarchical AST into a memory-contiguous graph (Arena-based layout). Maps IDs to $O(1)$ array offsets.

## Error Handling
The compiler uses the **Result Pattern** extensively. `ParseError` and `ValidationError` types are aggregated into a top-level `CompileError` to provide precise diagnostic information to the user.
