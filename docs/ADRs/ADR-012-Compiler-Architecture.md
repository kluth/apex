# ADR-012: Compiler Architecture

## Context
APEX requires a robust and maintainable compiler frontend to translate declarative `.apex` files into the anatomical AST. The grammar is designed to be highly readable but contains biological constraints that should ideally be checked as early as possible in the compilation pipeline.

## Decision
APEX will implement a **Recursive-Descent Parser** written from scratch in Rust, accompanied by a hand-written **Lexer**.

### 1. Lexer (Tokenizer)
The lexer will scan the source text and produce a stream of `Token` variants. It will handle:
- Keywords (`organism`, `bone`, `joint`, `muscle`, `substrate`)
- Identifiers and numeric literals
- Units (`kg`, `m`, `N_m`, `rad`)
- Structural delimiters (`{`, `}`, `=`, `;`)
- Comments and whitespace (ignored)

### 2. Parser (Recursive-Descent)
The parser will consume the token stream using recursive functions that map directly to our EBNF production rules.
- **Error Handling:** Use the Result pattern to propagate detailed `ParseError` types, including expected vs. actual tokens.
- **On-the-fly Validation:** Basic structural invariants (like missing mandatory properties) will be checked during parsing.

## Rationale
Hand-written recursive-descent parsers are easier to debug, provide superior error messages compared to parser generators (like Yacc/Bison), and allow for complex biological validation to be interwoven into the parsing logic. This aligns with our "Parse, Don't Validate" mandate by ensuring that the AST produced is already structurally sound.
