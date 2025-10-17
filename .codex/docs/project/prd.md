# Product Requirements – Codex Memory Persistence Upgrade

## Problem Statement
Developers using Codex lose project momentum because session rollouts and history logs do not surface actionable “what’s next” context. Replaying transcripts or rereading diffs is slow, error-prone, and blind to git branch divergence.

## Objectives
1. Persist a canonical project roadmap that survives restarts and branch changes.
2. Provide users with clear visibility into active tasks, pending approvals, and completed milestones.
3. Detect and communicate git drift before new work begins.

## Target Users
- Engineers working on multi-day or multi-branch repositories with Codex.
- Researchers evaluating Codex persistence in comparison with other AI CLIs.

## Success Metrics
- Users can resume a project with actionable “next steps” without rereading prior chats.
- Git branch changes trigger cache reconciliation prompts in >95% of tested scenarios.
- Task graph commands (`codex tasks`, `codex plan show`, etc.) return results within 100 ms for repositories up to 5k files.

## Functional Requirements
- Read/write `project_state.json` and per-session cache files automatically with minimal user setup.
- CLI exposes commands to inspect and modify state (show tasks, mark complete, clear).
- Plan updates issued via `update_plan` or CLI commands sync to the cache.
- Git drift detection offers options: fork state for new branch, rebase cache, or ignore (with warning).

## Non-Functional Requirements
- File-based storage; no external services.
- Git-friendly formats (JSON/TOML) to support manual diffing and PR review.
- Cache writes must not block Codex execution longer than 20 ms per turn on benchmark hardware.
- Feature must be gated by config flag (`memory.cache.enabled`).
