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

## Phase 2 features

Structs, lists, loops, and string operations:

```primed
t note { id i, text s }

f main()
  v ns = [{note id: 1, text: "buy milk"}]
  each n ns
    p "#{n.id}: {n.text}"

  m tags = ["a", "b"]
  push(tags, "c")
  p join(tags, ",")

  m nums = [5, 3, 9]
  sorti(nums)
  p "{sum(nums)}"
```

| primed                  | meaning                       |
|-------------------------|-------------------------------|
| `t name { a i, b s }`   | struct decl                    |
| `{note id: 1}`          | struct literal                 |
| `n.id`                  | field access                   |
| `[1, 2, 3]` / `["a"]`   | list literal                   |
| `each x list`           | for-in loop                    |
| `push pop count get`    | list ops (`pushi popi geti` for i64 lists) |
| `sort sorti sum seti`   | more list ops                  |
| `split join lines trim` | string ops                     |
| `upper lower rep rev`   | string ops                     |
| `has starts ends idx`   | search ops                     |
| `cut(s, a, b)`          | substring                      |

Examples: `hello.pm`, `notes.pm`, `phase2.pm`, `todo.pm` (full task manager in
~1.6KB).

## Phase 3 features

Time, terminal interaction, and app installation:

| primed            | meaning                                  |
|-------------------|------------------------------------------|
| `clock()`         | `"2026-10-06 14:32"` timestamp           |
| `today()`         | `"2026-10-06"` date                      |
| `sleep(ms)`       | pause the program                        |
| `ask("prompt: ")` | read a line from stdin                   |
| `cls()`           | clear the terminal                        |
| `primed install file.pm` | compile + install to `~/.local/bin` |

New apps: `timer.pm` (pomodoro countdown), `journal.pm` (searchable daily
journal). Install any of them:

```sh
./target/release/primed install examples/journal.pm
journal write "shipped phase 3"
journal find "phase"
```

## Phase 4 features: LLM tooling

**Token report** — measure the token cost of any program:

```sh
./target/release/primed tokens examples/todo.pm
# primed source: 1718 bytes, ~516 tokens
# generated rust: 10385 bytes, ~3244 tokens
```

**Cheat sheet** — the entire language spec, sized to paste into an LLM system
prompt (~150 tokens):

```sh
./target/release/primed cheat
```

The intended workflow for LLM-driven development:

1. `primed cheat` output goes into the system prompt (one-time, tiny).
2. The model writes dense `.pm` source — a full task manager costs ~500 tokens
   instead of ~3000+ for hand-written Rust.
3. `primed run file.pm` verifies; `primed tokens file.pm` reports cost;
   `primed install file.pm` ships it.

## Phase 7 features: JSON output mode

`primed doc file.pm [--mcp]` emits a single machine-readable JSON object so
agents can self-verify without parsing prose:

```json
{
  "file": "examples/todo.pm",
  "source": {"bytes": 1718, "tokens_est": 516},
  "rust": {"bytes": 10385, "tokens_est": 3244},
  "mode": "bin",
  "functions": ["db() s", "load() s", "main()"],
  "tools": [],
  "structs": [],
  "rust_source": {"content": "... full transpiled Rust ..."}
}
```

An agent can transpile, inspect signatures, check token cost, and read the
generated Rust — one command, one parse.

## Roadmap

- **Phase 5 — Web/API shorthands**: `srv(port)` starts an HTTP server; `route(path, method)` handlers; JSON helpers (`jget jstr jnum jbool`). Target: a working API in 3 lines of primed.
- **Phase 6 — MCP server support**: `primed mcp file.pm` speaks the MCP protocol over stdio so LLM hosts can call your app as a tool.
- **Phase 7 — JSON output mode**: `primed doc file.pm --json` emits machine-readable output (transpiled Rust, AST metadata, token counts) so agents can self-verify without parsing prose.
- **Phase 8 — Watch mode**: `primed watch file.pm` re-transpiles and re-runs on save.
- **Phase 9 — Rust FFI / external crates**: `use crate_name` pulls in external Rust libraries; primed calls map to their public APIs with typed wrappers, so the whole Rust ecosystem stays available while source stays token-dense.
