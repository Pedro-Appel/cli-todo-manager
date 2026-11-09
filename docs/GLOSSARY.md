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

## Newtype pattern
A tuple struct wrapping one value (`struct TaskId(u32);`) so the compiler
treats it as its own type. You can't pass a `TaskId` where a `UserId` or a
bare `u32` is expected. Zero runtime cost. Java analogue: a value class.
See: Rust Book ch. 19.3.

## `Clone` vs `Copy`
`Clone` = explicit duplication, you write `.clone()` and it may allocate.
`Copy` = the compiler duplicates implicitly on assignment/pass (like Java
primitives); only allowed for cheap, plain-data types (a `u32` wrapper
qualifies, a `String` never can). If a type is `Copy`, calling `.clone()`
on it is a clippy lint (`clone_on_copy`).

## `String` vs `&str` (and getters)
`String` owns its heap buffer; `&str` is a borrowed view into someone
else's string data. A getter that only lets the caller *read* a field
should return `&str` (a borrow, no allocation), not `String` (which forces
a clone each call). Rule of thumb: own in struct fields, borrow in
parameters and read-only return values.

## `dead_code` lint
rustc warns about items nothing uses. In a binary crate "used" means
reachable from `main`, so new code that is only exercised by tests is
reported as dead until something calls it (clippy `-D warnings` turns that
into an error). `#[allow(dead_code)]` silences it forever;
`#[expect(dead_code)]` silences it but warns once it's no longer needed,
so the exception cleans itself up.

## `#[cfg(test)]`
Conditional compilation: the `mod tests` block exists only when running
`cargo test`, so tests live next to the code (and can see private items via
`use super::*`) without being shipped in the binary. Java's closest
equivalent is the separate `src/test` tree, but without the visibility wall.

## `Option<T>` vs `Result<T, E>`
`Option` says "a value or nothing" (`Some`/`None`): use it when absence is
normal and needs no explanation (e.g. "find a task by id"). `Result` says
"a value or *why it failed*" (`Ok`/`Err`): use it when a caller may need to
know the reason, like rejecting an invalid title. See: Rust Book ch. 6.1, 9.2.

## Return small `Copy` types by value
Borrow when the value is large or owns heap data (`&str` over `String`);
return by value when it's `Copy` and tiny (`bool`, `u32`, a `Copy` newtype).
`&bool` buys nothing, costs a pointer, and makes callers write `*x`.
