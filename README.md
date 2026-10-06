# primed

A token-efficient programming language for LLMs. `primed` source is short,
dense, and machine-oriented — it transpiles to **Rust**, so you get Rust's
performance and memory safety while spending a fraction of the tokens.

```primed
f greet(n s) s
  ret "hi {n}"

f main()
  p greet(arg(1) ? "world")
```

generates Rust, compiles with `rustc -O`, and runs as a native binary.

## Why

LLMs pay per token. Writing Rust directly costs imports, types, semicolons,
`format!`, boilerplate. `primed` strips all of it: one-char keywords, no
punctuation noise, string interpolation, terse types (`i`, `f`, `s`, `b`),
and a builtin stdlib that covers files, args, and I/O in one call.

## Install / usage

```sh
cargo build --release
./target/release/primed run examples/hello.pm
./target/release/primed build examples/notes.pm -o notes
./notes add "buy milk"
./notes list
./target/release/primed transpile examples/notes.pm   # inspect generated Rust
```

## Language (Phase 1)

See [SPEC.md](SPEC.md) for the full spec and roadmap.

| primed            | meaning                          |
|-------------------|----------------------------------|
| `f name(a i, b i) i` | function, typed params/return |
| `v x = 1`         | let binding                      |
| `m x = 1`         | mutable binding                  |
| `x = 2`           | assignment (mut only)            |
| `if c` / `elif` / `e` | branching                   |
| `w c`             | while loop                       |
| `"hi {n}"`        | interpolated string              |
| `p x`             | println                          |
| `ret x`           | return                           |

Builtins: `p` `len` `int` `str` `read` `write` `append` `arg` `args` `home`
`now`. See [SPEC.md](SPEC.md).

## Roadmap

- Phase 2: structs, lists, string ops, `filter`/`map` shorthands
- Phase 3: example macOS apps (notes, timer), packaged binaries
- Phase 4: web/API/MCP shorthands, LLM-oriented tooling (transpile-on-save, size linter)
