# Implementation Plan – Codex Memory Persistence Upgrade

> **Legend**  
> Status: `Pending` (not started), `In Progress`, `Blocked`, `Complete`  
> ADR: See `.codex/adr/<task>/<file>.md`

## Phase 0 – Foundations
**Objective:** Establish core data contracts and guard rails for the cache layer.

| ID        | Status   | ADR Path                                        | Summary                                                      | Dependencies |
|-----------|----------|-------------------------------------------------|--------------------------------------------------------------|--------------|
| TASK-010  | Pending  | `.codex/adr/TASK-010/ADR-010-cache-schema.md`   | Define schema versions, top-level keys, validation rules.    | —            |
| TASK-020  | Pending  | `.codex/adr/TASK-020/ADR-020-feature-flags.md`  | Introduce `memory.cache.enabled` flag + config defaults.     | TASK-010     |
| TASK-030  | Pending  | `.codex/adr/TASK-030/ADR-030-cache-manager.md`  | Skeleton cache manager (load/save, backups, migrations).     | TASK-010     |

## Phase 1 – Short-Term Cache
**Objective:** Capture per-turn context and expose recent history to users.

| ID        | Status   | ADR Path                                        | Summary                                                      | Dependencies       |
|-----------|----------|-------------------------------------------------|--------------------------------------------------------------|--------------------|
| TASK-110  | Pending  | `.codex/adr/TASK-110/ADR-110-turn-hook.md`      | Hook session lifecycle to persist turn summaries.            | TASK-030           |
| TASK-120  | Pending  | `.codex/adr/TASK-120/ADR-120-short-cache.md`    | Summarise/prune session caches; manage size constraints.     | TASK-110           |
| TASK-130  | Pending  | `.codex/adr/TASK-130/ADR-130-cli-short-cache.md`| Add `codex cache show --recent` command.                     | TASK-110, TASK-120 |

## Phase 2 – Long-Term Project State
**Objective:** Store project roadmap, task graph, and backlog information.

| ID        | Status   | ADR Path                                        | Summary                                                      | Dependencies              |
|-----------|----------|-------------------------------------------------|--------------------------------------------------------------|---------------------------|
| TASK-210  | Pending  | `.codex/adr/TASK-210/ADR-210-task-graph.md`     | Define task graph model + persistence layout.                | TASK-030                  |
| TASK-220  | Pending  | `.codex/adr/TASK-220/ADR-220-plan-integration.md`| Sync `update_plan` tool and CLI commands with cache.         | TASK-210                  |
| TASK-230  | Pending  | `.codex/adr/TASK-230/ADR-230-manual-edit-docs.md`| Docs + utilities for manual editing & review of project state.| TASK-210, TASK-220        |

## Phase 3 – Git Drift & Reconciliation
**Objective:** Detect branch/head divergence and offer safe recovery paths.

| ID        | Status   | ADR Path                                        | Summary                                                      | Dependencies              |
|-----------|----------|-------------------------------------------------|--------------------------------------------------------------|---------------------------|
| TASK-310  | Pending  | `.codex/adr/TASK-310/ADR-310-git-compare.md`    | Compare cached git info vs live repo before each turn.       | TASK-010, TASK-210        |
| TASK-320  | Pending  | `.codex/adr/TASK-320/ADR-320-reconcile-ui.md`   | CLI/TUI reconciliation flows (fork, rebase, ignore w/ warning).| TASK-310, TASK-220      |
| TASK-330  | Pending  | `.codex/adr/TASK-330/ADR-330-drift-tests.md`    | Automated tests for branch switch, rebase, detached HEAD.    | TASK-320                  |

## Phase 4 – UX & Documentation
**Objective:** Surface cache insights to users and capture institutional knowledge.

| ID        | Status   | ADR Path                                        | Summary                                                      | Dependencies           |
|-----------|----------|-------------------------------------------------|--------------------------------------------------------------|------------------------|
| TASK-410  | Pending  | `.codex/adr/TASK-410/ADR-410-ui-messaging.md`   | Update TUI/CLI to display tasks, drift alerts, cache status. | TASK-130, TASK-320     |
| TASK-420  | Pending  | `.codex/adr/TASK-420/ADR-420-doc-package.md`    | Produce end-user docs & migration notes.                     | TASK-230, TASK-320     |
| TASK-430  | Pending  | `.codex/adr/TASK-430/ADR-430-telemetry.md`      | Instrument cache metrics and alerting thresholds.            | TASK-110, TASK-210     |

## Phase 5 – Opt-In Release
**Objective:** Ship the feature behind a flag and prepare for upstream contribution.

| ID        | Status   | ADR Path                                        | Summary                                                      | Dependencies              |
|-----------|----------|-------------------------------------------------|--------------------------------------------------------------|---------------------------|
| TASK-510  | Pending  | `.codex/adr/TASK-510/ADR-510-rollout-config.md` | Config gating, fallback behaviour, error messaging.          | TASK-320, TASK-410       |
| TASK-520  | Pending  | `.codex/adr/TASK-520/ADR-520-migration-script.md`| Script to seed cache from historical rollouts.               | TASK-030, TASK-210       |
| TASK-530  | Pending  | `.codex/adr/TASK-530/ADR-530-upstream-package.md`| Upstream PR plan, testing matrix, release checklist.         | TASK-510, TASK-520       |

## Cross-Phase Activities
- Maintain `research/refined-research-1/plan.md` and phase ADRs in sync.
- Update `.codex/codex.md` whenever conventions evolve.
- After each task completion:
  1. Finalise ADR (status → `Complete` + outcomes).
  2. Commit with ADR ID reference.
  3. Push to `master`, then merge into current `refined-research-*` branch.
