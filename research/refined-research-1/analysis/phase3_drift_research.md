# Phase 3 – Git Drift & Reconciliation Research

## Objectives
- Document strategies for detecting and resolving git drift between cached state and the working tree.
- Align reconciliation UX with Codex session flows (approvals, plan updates).
- Identify validation and test requirements.

## Findings
1. **Drift Detection**
   - Compare cached `git.branch` + `git.head` against live repo before each user turn (extend `collect_git_info` usage).
   - Classify drift types:
     - `FastForward` – HEAD advanced on same branch (safe to update).
     - `BranchSwitch` – branch name changed.
     - `HistoryRewrite` – HEAD replaced (rebase/force-push).
     - `Unknown` – git command failure (detached HEAD, repo missing).
2. **Reconciliation Options**
   - **Fork Cache**: copy long/short caches under new branch key; preserve old history for reference.
   - **Rebase State**: rewrite cached git info to new HEAD, mark dependent tasks as “needs review”.
   - **Ignore Once**: continue session with warning banner (record decision in cache metadata).
3. **User Experience**
   - Display drift banners in TUI + CLI summary (Phase 4 research).
   - Provide command `codex cache reconcile` for manual invocation.
   - Log reconciliation history for auditing.
4. **Testing**
   - Build integration tests that script branch switches, rebases, and detached HEAD states (reuse sandbox/fixtures approach).
   - Validate that forks/rebases preserve task graph integrity (no orphaned dependencies).

## Open Questions
- Should Codex automatically fork on branch switch, or prompt first? Usability testing required.
- How to surface drift state in ADRs/implementation plan? Possibly include reconciliation records in ADR appendices.

## References
- ADR-310/320/330 placeholders (implementation plan).
- `phase1_cache_hook_plan.md` (hook integration point for drift checks).
