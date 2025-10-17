# Implementation Plan – Codex Memory Persistence Upgrade

## Phase 0 – Foundations
- [ ] Finalise cache schema and config flags (`memory.cache.enabled`).
- [ ] Build cache manager prototype (load/save, schema validation, backups).
- [ ] Instrument logging + metrics hooks.

## Phase 1 – Short-Term Cache
- [ ] Hook session lifecycle to write per-turn summaries (turn result, approvals, git snapshot).
- [ ] Implement short cache pruning (keep recent N turns, summarise older entries).
- [ ] Add CLI command `codex cache show --recent`.

## Phase 2 – Long-Term Project State
- [ ] Persist `project_state.json` with task graph, milestones, backlog.
- [ ] Extend `update_plan` handler and CLI commands to sync plan/task updates.
- [ ] Provide manual edit tooling (`codex tasks edit`, docs).

## Phase 3 – Git Drift & Reconciliation
- [ ] Compare cached git info vs live repo before each turn.
- [ ] Implement reconciliation options (fork cache, rebase state, ignore with warning).
- [ ] Automated tests across branch switch / rebase / detached HEAD.

## Phase 4 – UX & Documentation
- [ ] Enhance TUI/CLI messaging (display active tasks, drift alerts).
- [ ] Ship documentation in `.codex/docs/project`.
- [ ] Gather pilot feedback, iterate on defaults.

## Phase 5 – Opt-In Release
- [ ] Enable feature flag for early adopters.
- [ ] Monitor telemetry (cache write failures, drift frequency).
- [ ] Prepare upstream PR with migration guide and rollout plan.
