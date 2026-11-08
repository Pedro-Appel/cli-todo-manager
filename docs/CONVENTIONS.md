# Conventions

## Formatting & linting — non-negotiable from M0 onward
- `cargo fmt --all -- --check` must pass. rustfmt's defaults is used as-is
- `cargo clippy --all-targets --all-features -- -D warnings` must pass.
- CI enforces both; don't wait for CI to find out — run them locally first.

## Naming
- Types/traits: `UpperCamelCase` (`TaskList`, `Store`).
- Functions/variables/modules: `snake_case`.
- Constants: `SCREAMING_SNAKE_CASE`.
- No abbreviations that aren't obvious (`task`, not `t`; exception: short
  loop variables like `i`, and well-known ones like `cfg`).

## Error handling
- Library-ish code (`domain`, `storage`) defines and returns its own error
  type (`AppError`, built with `thiserror`) — never `anyhow::Error` there.
- `main.rs` is the only place allowed to print an error and choose an exit
  code.
- No `.unwrap()` / `.expect()` outside `#[cfg(test)]` code. If a value is
  genuinely guaranteed never to be `None`/`Err`, that's worth a comment
  explaining *why*, not a silent unwrap.

## Testing
- Pure logic (`domain`) gets inline unit tests: `#[cfg(test)] mod tests { ... }`
  at the bottom of the same file.
- End-to-end behavior (does the compiled binary actually do the right thing
  for a user) gets an integration test under `tests/`, using `assert_cmd`.
- A task isn't "done" without at least one test covering its acceptance
  criteria, unless the card explicitly says otherwise.

## Comments & docs
- Default to no comments. Code should read from good names.
- Write a comment only for the *why* that isn't visible in the code: a
  non-obvious constraint, a workaround, a deliberately-deferred decision.
- Once a type/function is used from outside its own module, give it a short
  `///` doc comment — one line is usually enough.

## Git
- Branch per task: `feature/<task-id>-<short-slug>`, e.g.
  `feature/1.2-task-list`.
- Commits: Conventional Commits style, referencing the task id —
  `feat(domain): add TaskList::complete (1.2)`.
- One PR per task (small milestones may batch 2 tiny tasks into one PR —
  ask first). PR description uses `.github/pull_request_template.md`.
- Rebase on `main` before opening the PR if `main` has moved; don't merge
  commits into a feature branch.

## Dependencies
- Before adding a crate, say why in the PR description (what it replaces,
  what hand-rolling it would've cost). Senior review will ask if it's not
  there.
- Keep `Cargo.lock` committed (this is a binary crate) so builds are
  reproducible.
