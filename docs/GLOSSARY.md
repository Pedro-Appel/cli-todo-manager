# Glossary

Terms get added here the first time they come up in a review, with a short
plain-language explanation and, where useful, a link to the chapter of
*The Rust Book* that covers them properly. This file is a side effect of
doing the work, not something to pre-fill — it starts empty on purpose.

## Cargo.lock
Exact resolved versions of every dependency (and transitive dependency).
Committed for binaries so every build — yours, CI's, a teammate's — uses
identical versions; libraries usually don't commit it. Cargo.toml says what
you *accept* (`serde = "1"`), Cargo.lock says what you *got*.
See: Rust Book ch. 14.3.

## Edition (`edition = "2024"`)
Opt-in language-compatibility epoch, set per crate in Cargo.toml. Crates on
different editions interoperate; editions only change syntax/lint defaults,
never split the ecosystem. Not the same as the compiler version.

## `-D warnings`
A rustc/clippy flag: **D**eny — turn every warning into a hard error. After
the `--` in `cargo clippy ... -- -D warnings` it is passed to the lints
themselves. Makes warnings impossible to ignore; the cost is that a new
toolchain with new lints can break CI until you fix or `#[allow]` them.

## Clippy lint groups
`clippy::correctness` (deny by default: almost certainly a bug), `style`,
`complexity`, `perf`, `suspicious` (warn by default), and opt-in groups like
`pedantic`/`nursery`. Loosen one lint with a reason via `#[allow(...)]`
rather than dropping `-D warnings` globally.

## `cargo fmt` vs `cargo fmt --check`
`cargo fmt` rewrites files; `cargo fmt --all -- --check` only reports
differences and exits non-zero. A CI job must *check*, never rewrite: a
rewrite happens on the throwaway runner, so the step always passes and the
repo never gets fixed.

## Fail-fast CI ordering
Put cheap, fast checks (fmt) before slow ones (build, test) so feedback on
trivial mistakes arrives in seconds, not minutes.
