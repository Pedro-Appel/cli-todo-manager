# Roadmap

The project is split into milestones (deliverables). Each milestone ships a
working, demoable increment of the CLI. Tasks are tracked on Trello
(https://trello.com/b/cblgjxhV/cli-manager); this file is the stable,
versioned record of the plan, and gets a short retro note after each
milestone ships.

Task IDs follow `<milestone>.<task>`, e.g. `1.3`. Branches and commits
reference this ID (see `docs/CONVENTIONS.md`).

## M0 — Environment & Hello World
Goal: your own toolchain and project skeleton, built by hand, not handed to
you — this milestone is as much the point as any later one.
- 0.1 Install Rust via `rustup`; verify `rustc`/`cargo`/`clippy`/`rustfmt`
- 0.2 `cargo init` the project here; get `cargo run` printing Hello World;
  first commit — inspect what `cargo init` generated before committing it
- 0.3 Decide and write a `rustfmt.toml` (or deliberately keep defaults —
  either is fine, say why); run `cargo fmt`
- 0.4 Run `cargo clippy`; decide the project's policy (we're adopting
  `-D warnings`, i.e. clippy warnings fail the build) and say why in the PR
- 0.5 Add a GitHub Actions workflow that runs fmt-check/clippy/test/build on
  every PR; open the actual PR that introduces it and watch it go green

Learning focus: `rustup`/`cargo` basics, what `cargo init` actually
generates, reading compiler/clippy diagnostics, GitHub Actions basics, the
branch → PR → CI → merge loop.

## M1 — Core Domain & In-Memory CRUD
Goal: a fully working TODO CLI with no persistence — state lives only for
one run (lost on exit). Visible, demoable progress.
- 1.1 `Task` struct + `TaskId` (domain model), unit tests
- 1.2 In-memory `TaskList` (add / list / complete / remove)
- 1.3 `clap` CLI skeleton (subcommands parse but aren't wired yet)
- 1.4 Wire subcommands to `TaskList` in `main.rs`

Learning focus: structs, ownership & borrowing, `Vec<T>`, pattern matching,
`clap` derive macros.

## M2 — Persistence
Goal: tasks survive between runs, saved to a JSON file.
- 2.1 `Store` trait (`load` / `save`) — the seam for swapping storage later
- 2.2 `JsonFileStore` using `serde` + `serde_json`
- 2.3 Wire persistence into `main` (load at startup, save after each mutation)
- 2.4 `AppError` via `thiserror` for I/O / parse failures

Learning focus: traits as abstraction boundaries, `serde` derive, `Result`
and the `?` operator, error design.

## M3 — CLI UX
Goal: the tool is pleasant to use, not just functional.
- 3.1 Polish `clap` help text, command/arg naming
- 3.2 Colored, aligned output for `list`
- 3.3 Filters & sorting (`--all` / `--pending` / `--done`, sort by id/date)

Learning focus: iterators & iterator chains, closures, crate evaluation.

## M4 — Testing & Robustness
Goal: confidence the tool won't silently corrupt data or crash ugly.
- 4.1 Unit tests for domain logic
- 4.2 Integration tests (`assert_cmd` + `tempfile`) driving the real binary
- 4.3 Harden CI: fmt/clippy gates treated as non-negotiable
- 4.4 Proper exit codes & user-facing error messages (vs. panics)

Learning focus: test organization (`tests/` vs `#[cfg(test)]`), `panic!` vs
`Result`, exit code conventions.

## M5 — Advanced Features
Goal: a genuinely useful personal TODO tool.
- 5.1 Priority (`enum Priority { Low, Medium, High }`)
- 5.2 Due dates (`chrono`)
- 5.3 Tags (`Vec<String>`, add/remove)
- 5.4 Combined filtering & sorting across priority/due date/tags

Learning focus: enums with data, `Ord`/`PartialOrd`, working with a bigger
dependency (`chrono`), combinators.

## M6 — Packaging & Distribution
Goal: something you could hand to someone else to install and use.
- 6.1 README usage docs & examples (gifs/screenshots optional)
- 6.2 Release workflow: build binaries in GitHub Actions on tag push
- 6.3 Shell completion generation (`clap_complete`)
- 6.4 Stretch: configurable storage path; `cargo audit` in CI

Learning focus: release engineering basics, shipping a binary crate.

## Retro log
<!-- One short entry per milestone once it ships: what went well, what was
confusing, what we'd change in pacing or scope. Keeps the plan honest. -->

### M0 — Environment & Hello World (shipped 2026-11-08, PR #1)
- **Went well:** quick turnaround on review feedback; kept strict
  `-D warnings` and SHA-pinned actions; the first branch → PR → CI → review →
  merge loop was completed end to end.
- **Confusing:** CI took two review rounds. `cargo fmt` rewrites files
  while `cargo fmt --all -- --check` can fail; a SHA-pinned action needs
  explicit inputs (no toolchain name); the order must be checkout →
  toolchain → cheap checks → test/build. All now in `docs/GLOSSARY.md`.
- **Pacing/process change:** tasks 0.2–0.5 were bundled in one PR on a
  differently named branch. From M1 on: one branch and PR per task, branch
  name matches the card (`feature/1.1-task-model`), Conventional Commit
  messages, and PR descriptions that justify open decisions. Bundling needs
  agreeing up front.
- **Open item:** never verified that the fmt check fails on bad input;
  optional local check.
