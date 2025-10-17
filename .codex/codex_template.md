# Codex Memory Research – Agent Conventions

This document defines the operational rules Codex must follow at the start of every session and while executing tasks. Treat it as canonical unless superseded by updated instructions in `.codex/`.

---

## Start-of-Session Protocol
On every new chat session:

1. **Scan project artefacts**
   - `.codex/docs/project/` – PRD, TRD, implementation plan, overview.
   - `.codex/adr/` – preliminary ADRs for active subtasks.
   - `research/` – inspect subdirectories to determine the latest research phase (prefer the most recently updated folder; confirm via `plan.md` inside it).  
   - Any additional context noted in the previous session summary.
2. **Identify active workstream**
   - Read `.codex/docs/project/implementation-plan.md` to understand phases, task IDs, and dependencies.
   - Open the phase-specific `plan.md` located under the active research folder (naming conventions may evolve; rely on timestamps and plan content rather than hard-coded prefixes).
   - Review ADRs relevant to the task(s) you are about to touch.
3. **Establish plan visibility**
   - If work involves multiple steps, call the `update_plan` tool to maintain a live checklist (max one `in_progress` item).
4. **Confirm git state**
   - Run `git status -sb` and note the current branch. Ensure local branches mirror the documented workflow before making changes.

---

## Repository Awareness
- `.codex/` holds the authoritative conventions, project docs, and ADRs; instructions here override older records unless explicitly deprecated.
- `research/` stores evidence and analysis. Cite files with relative paths and line numbers (e.g., `research/refined-research-1/analysis/codex_gap_analysis.md:12`) when referencing findings.
- Do not assume fixed folder names—use file content (plans, timestamps) to identify the current phase or archive.

---

## Planning & ADR Discipline
- **Every subtask requires a preliminary ADR** before implementation begins.
  - Location: `.codex/adr/TASK-<ID>/ADR-<ID>-<slug>.md`.
  - Required sections: Context, Decision, Implementation Steps, Success Criteria, References/Artifacts, Status (`Planning` → `Complete`).
- Update ADRs with outcomes once the subtask is delivered.
- Commit messages must reference the ADR ID (e.g., `ADR-210`) to maintain traceability.
- When drafting plans, align with `.codex/docs/project/implementation-plan.md` and keep the phase tracker in `research/.../plan.md` up to date.

---

## Standard Workflow
### Before Coding
1. Read the relevant ADR(s) and the latest plan entries.
2. Break the task into verifiable steps and register them via `update_plan`.
3. Check for existing utilities or patterns that apply.

### On Every Code Change
- MUST write or update tests first (TDD mindset).
- MUST re-read the target file immediately before editing.
- MUST use non-destructive edits unless instructed otherwise.
- MUST run linting/type checks if modifications touch code.
- MUST update ADR with decisions and outcomes.
- MUST verify success criteria and validation rules prior to marking a task complete.
- MUST commit with ADR reference and descriptive message.
- MUST run relevant automation/cleanup scripts when files are added or reorganised.

### After Completing a Subtask
1. Ensure all tests and validations pass (document what was run).
2. Finalise the ADR (`Status: Complete`) and note implementation results.
3. Summarise changes:
   - Files + line references.
   - Validation executed (tests, scripts).
   - Remaining risks or follow-up tasks.
4. Push changes following the branch policy below.

---

## Branching & Git Policy
- `master` maintains the latest published snapshot.
- Each research phase operates on its own long-lived branch (e.g., mirrors the phase described in the plan). Determine the correct branch from project docs rather than assuming a prefix.
- Standard push sequence:
  1. Work on `master`.
  2. Commit (`git commit -m "Implement cache schema ADR-010"`).
  3. `git push origin master`.
  4. Switch to the phase branch, merge master, push (`git checkout <phase-branch> && git merge master && git push`).
- Never bypass pushing `master` before updating the phase branch.

---

## Quality Gates (Must Hold True Before Sign-off)
- Documentation updated (`.codex/docs/project`, ADRs, phase plans).
- References include precise file paths and line numbers.
- No unresolved TODOs or placeholder text in ADRs or plans.
- Cache-related instructions in `.codex/codex.md` remain consistent with implementation; update this doc when conventions change.

---

## Communication Guidelines
- Cite sources (file path + line) for all factual assertions.
- Flag risks, blockers, or open questions separately from resolved items.
- When instructions conflict, apply order of precedence:
  1. Latest `.codex/` guidance.
  2. Active phase plan (`research/.../plan.md`).
  3. ADRs and project docs.
  4. Archived research notes.
- Summaries should mirror research outputs: highlight gaps, proposed improvements, and next steps.

---

## Quick Reference Checklist
- [ ] Scanned `.codex/` docs and active plan.
- [ ] Reviewed ADR(s) and updated status.
- [ ] Registered plan steps via `update_plan`.
- [ ] Read files before editing; run validations.
- [ ] Documented results with citations and pushed to `master` then phase branch.
