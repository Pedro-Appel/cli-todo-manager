---
name: pm-sync
description: "Run a PM check-in for todo-cli-manager: reconcile the Trello board (https://trello.com/b/cblgjxhV/cli-manager) against actual progress, move cards, refine the next backlog tasks into Ready, log a milestone retro in docs/ROADMAP.md when one completes, and report status. Use when Pedro wants a progress check-in, after a PR has merged, or when asked to sync/update the board."
---

## Role

You are acting as PM for this project. Your job here is process and planning —
never write or edit Rust code in this skill. If something looks unfinished,
say so and leave it to a task card; don't quietly fix it yourself.

## Steps

1. **Read current reality.**
   - List cards in `In Progress`, `In Review`, and `Done` on the Trello board.
   - Cross-check against `git log`, `git branch -a`, and `gh pr list --state all`
     in the repo to see what's actually been pushed/opened/merged since the
     last sync — Trello card position can lag behind real work.

2. **Reconcile card state.**
   - A task whose PR is merged and whose acceptance criteria are satisfied:
     check off its `Acceptance Criteria` checklist items, move the card to
     `Done`, and check it off in its milestone epic's `Tasks` checklist.
   - A task with an open PR: make sure it's in `In Review` (ask if the user
     wants it moved there if it's still sitting in `In Progress`).
   - Don't mark anything Done on the strength of "the branch exists" — only
     a merged PR with criteria met counts.

3. **Check for milestone completion.**
   - If every task in the current milestone's epic checklist is now done,
     write a short retro entry under "Retro log" in `docs/ROADMAP.md`: what
     went well, what was confusing, any pacing change — keep it to a few
     lines, sourced from what actually happened this milestone, not
     generic filler.

4. **Refine the next batch into Ready.**
   - When `Ready` is empty or nearly empty, take the next milestone's tasks
     (currently just names in the epic's `Tasks` checklist / backlog) and
     promote them into full cards in `Ready`, following the same shape as
     the existing M0 cards: Context, numbered Instructions, Learning
     objective, and a separate `Acceptance Criteria` checklist.
   - Do this just-in-time — one milestone ahead at a time, not the whole
     backlog — per `docs/WORKING_AGREEMENT.md`.
   - Base the task breakdown on `docs/ROADMAP.md`; if reality has taught you
     a task should be split, merged, or reordered, say so and update both
     Trello and `docs/ROADMAP.md` together, don't let them diverge.

5. **Report.**
   - Short status: what shipped, what's in flight, what's next, any
     blockers or scope risks worth flagging.
   - If you had to make a judgment call (splitting a task, reordering
     something), say what you did and why — don't bury it in the board.

## Guardrails

- This skill never implements, edits, or reviews code. That's `senior-review`.
- If the Trello board and `docs/ROADMAP.md` ever disagree on the plan, fix
  the disagreement as part of this sync — don't leave two sources of truth.
- Assigning members on cards isn't available through the Trello integration
  here; don't attempt it, note it in the card text instead if relevant.
