# primed language spec (v0.1 — Phase 1)

Goal: minimum tokens for an LLM to express a program; maximum leverage via
transpilation to Rust. Human readability is explicitly NOT a priority.

## Source form

- Files: `.pm`
- One statement per line. Indentation (2 spaces per level) = block nesting.
- No semicolons. No parentheses around conditions. Comments: `#` to EOL.

## Types (inferred at transpile, declared in signatures)

| char | type    |
|------|---------|
| `i`  | i64     |
| `f`  | f64     |
| `s`  | String  |
| `b`  | bool    |

## Declarations

```
f name(a i, b s) s      # fn; params typed; optional return type after )
v x = expr              # let (immutable)
m x = expr              # let mut
x = expr                # assign to a mut declared earlier (checked)
```

## Control flow

```
if cond
  ...
elif cond
  ...
e
  ...
w cond
  ...
```

Blocks by indentation only. `e` = else. No `else if` keyword — `elif` is one.

## Expressions

- Literals: `123`, `1.5`, `true`, `false`, `"text"`
- Interpolation: `"hi {name}, you are {age}"` — any expression inside `{}`.
- Operators: `+ - * / %`, comparisons `== != < <= > >=`, `and or not`
- `+` on strings concatenates.
- Calls: `name(args)`; `?` is default-if-missing for `arg`: `arg(1) ? "x"`
- Indexing: `s[0]` (char as String), slices: `s[0:3]`, `s[1:]`

## Builtin stdlib (global, no imports)

| call                | Rust equivalent                              |
|---------------------|----------------------------------------------|
| `p(x)`              | `println!("{x}")`                             |
| `len(x)`            | `.len()` (i64)                               |
| `int(s)` `int(f)`   | parse / truncate                             |
| `f2i(x)` `i2f(x)`   | casts                                        |
| `str(x)`            | `x.to_string()`                              |
| `char(s, i)`        | `s.chars().nth(i)` (String)                  |
| `read(path)`        | read file to String, abort on error          |
| `write(path, s)`    | write file, abort on error                   |
| `append(path, s)`   | append line                                  |
| `arg(i)`            | `env::args().nth(i)` (may be missing)         |
| `args()`            | all args joined? No: count is `argn()`; use `arg(i)` |
| `home()`            | `$HOME` path                                 |
| `now()`             | epoch millis                                 |
| `quit(i)`           | exit with code                               |

## CLI

```
primed run file.pm [args...]      # transpile, compile, run
primed build file.pm -o out       # produce native binary
primed transpile file.pm          # print generated .rs
```

## Semantics / errors

- Transpiler validates: assignment to immutable, unknown identifiers,
  arity of builtins, unmatched indentation. Errors point at line numbers.
- Runtime panics abort with file:line context (Rust default).
- All generated code is safe Rust; `unsafe` never emitted.

## Phase 2 proposals (not yet implemented)

- `t Name { a s, b i }` struct decl; `n Name(a:"x")` constructor
- `l i` list type, `push pop each filter map`
- string ops: `split join trim rep upper lower contains has`
- `match x` with `c pat` arms

## Phase 3 proposals

- `notes` example app: add/list/del via file-backed store, built as a
  macOS binary. GUI bindings out of scope for the transpiler; apps ship as CLI
  plus optional SwiftUI wrapper that reads the same note file.
