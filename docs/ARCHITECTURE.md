# Architecture

## Module layout (target shape, grown incrementally — see ROADMAP.md)

```
src/
  main.rs       entry point: parse CLI, dispatch, report errors, set exit code
  cli.rs        clap derive structs (Cli, Commands) — no business logic
  domain/
    mod.rs
    task.rs     Task, TaskId, Priority — pure data + logic, no I/O
    list.rs     TaskList: add/list/complete/remove over Vec<Task>
  storage/
    mod.rs      Store trait: load() -> Result<TaskList>, save(&TaskList) -> Result<()>
    json.rs     JsonFileStore: Store impl backed by a JSON file via serde
  error.rs      AppError (thiserror) — the one error type the app speaks
tests/
  cli.rs        integration tests driving the compiled binary (assert_cmd)
```

Only `main.rs` (that's task 0.2, via
`cargo init`). Modules are added as the matching roadmap task is picked up.
Don't pre-create empty modules; let the tree grow with the code that needs
it.

## Dependency direction

```
main.rs → cli.rs ─┐
                   ├→ domain (pure)
main.rs → storage ┘      ↑
             (storage depends on domain, never the reverse)
```

`domain` knows nothing about `clap` or file I/O. This is the one rule worth
protecting: it's what makes the domain testable without touching disk, and
makes it possible to swap `JsonFileStore` for something else later (SQLite,
a remote API) without touching `domain` at all. If you find yourself writing
`use clap::...` inside `domain/`, stop — that's a sign the logic belongs in
`main.rs` or `cli.rs` instead.

## Why a `Store` trait instead of calling `serde_json` directly from `main`

It costs one extra `trait` definition now, in exchange for:
- `domain` and `storage` can each be unit-tested in isolation.
- Swapping storage backends later (M6 stretch goal, or just for fun) touches
  one new file, not every call site.
- It's a small, honest example of why Rust programs lean on traits for
  abstraction instead of inheritance — worth internalizing early.

If this feels like overkill for a TODO app — it slightly is, on purpose.
The point of this project is practicing the pattern, not the minimum code to
ship a TODO list.

## Error handling boundary

- `domain` and `storage` return `Result<T, AppError>` (`AppError` defined
  with `thiserror`, one variant per real failure mode: `Io`, `Parse`, ...).
- `main.rs` is the only place that turns an `Err` into a printed message and
  a non-zero exit code. No `.unwrap()` / `.expect()` outside of tests —
  if you reach for one, that's the review's first comment.

## Open questions / revisit later
- Storage location: starts as a hardcoded path (`./todo.json`) for M2,
  revisited in M6 (XDG config dir via the `dirs` crate) — don't build that
  abstraction early, it's a known stretch goal, not a current requirement.
