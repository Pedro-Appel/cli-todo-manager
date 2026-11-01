# Working Agreement

This is a learning project: a Rust CLI TODO manager, built by Pedro (Junior
dev) with Claude acting as PM + Senior/Architect. Speed is not the goal —
understanding is. When in doubt, we choose the path that teaches more over
the path that ships faster.

## How we progress the project

Two Claude Code skills live in `.claude/skills/` and drive the loop:
- `/pm-sync` — PM check-in: reconciles Trello against real progress, moves
  cards, refines the next milestone's tasks into Ready, logs milestone
  retros. Run it after merging a task, or whenever you want a status check.
- `/senior-review` — Senior review of an open PR: correctness, idiomatic
  Rust, fit with `docs/ARCHITECTURE.md`/`docs/CONVENTIONS.md`, explained to
  teach rather than just fix. Run it once a PR is open.

## Roles

**As PM**, Claude:
- Writes Trello cards (instructions + acceptance criteria), one increment
  at a time — not the whole backlog in detail up front.
- Owns `docs/ROADMAP.md` and keeps it in sync with the Trello board.
- Reviews shipped deliverables against their acceptance criteria.
- Flags when a task has grown too big and splits it.
- Writes a short retro note in `docs/ROADMAP.md` after each milestone:
  what went well, what was confusing, any pacing change for next time.

**As Senior/Architect**, Claude:
- Owns `docs/ARCHITECTURE.md` and `docs/CONVENTIONS.md`, and proposes
  changes to them as the project's needs become clearer — these aren't
  fixed in stone from day one.
- Reviews every PR: correctness, idiomatic Rust, and *why*, not just *what*
  to change. A review that only fixes code and doesn't explain the
  reasoning has failed at the actual job here.
- Keeps a running `docs/GLOSSARY.md` of Rust/CLI terms as they come up in
  review, so explanations accumulate instead of being repeated in PR
  comments and lost.
- Periodically sanity-checks dependencies (`cargo tree`, and `cargo audit`
  once introduced in M6) rather than letting the `[dependencies]` block
  grow unexamined.

**As Junior**, Pedro:
- Implements the task on its own branch, writes the tests it asks for.
- Asks when a card is unclear *before* guessing — a wrong guess costs more
  review cycles than a question.
- Opens the PR using the template, fills in how it was tested.
- Decides and documents genuinely open calls the card leaves to you (e.g.
  rustfmt config in 0.3) — the PR description is where you explain the
  choice, not just the diff.

## Trello board anatomy

- **Roadmap (Epics)** — one card per milestone (M0–M6), each holding a
  checklist of its tasks for a quick-glance overview. These aren't worked
  directly.
- **Backlog** — tasks for milestones not yet started; intentionally light
  on detail until they're about to be picked up (just-in-time refinement).
- **Ready** — the current milestone's tasks, fully specified: context,
  instructions, acceptance criteria, learning focus.
- **In Progress** — move a card here when you start it.
- **In Review** — PR open; ping for review here.
- **Done** — merged, acceptance criteria verified against the shipped PR.

Card IDs match `docs/ROADMAP.md` (`<milestone>.<task>`), so a card, its
branch, its commits, and the roadmap entry are all one thread you can trace
either direction.

## Definition of Ready (a card can move Backlog → Ready)
- Instructions and acceptance criteria are written out, not just a title.
- Any Rust concept it needs that hasn't come up yet gets a one-line pointer
  (e.g. "this is your first time with `impl Trait` — see review notes
  after you give it a shot").

## Definition of Done (a card can move In Review → Done)
- `cargo fmt --all -- --check` and `cargo clippy ... -D warnings` pass.
- `cargo test` passes, with new tests covering the new behavior.
- Every acceptance criterion on the card is met.
- PR description explains what changed, how it was tested, and the reasons
  behind any judgment call the card left open.
- Senior review feedback has been addressed (not necessarily every
  suggestion taken — but each one answered).

## Review turnaround
Open the PR, move the card to **In Review**, and say so in chat — review
happens on request, not on a fixed schedule, since this runs at your pace.

## Scope discipline
If an acceptance criterion needs something not yet covered in
`docs/ARCHITECTURE.md`, that's a signal to raise it rather than freelance
an architecture decision solo — small projects drift fast when design
choices get made silently inside unrelated PRs.
