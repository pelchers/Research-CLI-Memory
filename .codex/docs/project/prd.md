# Product Requirements – Codex Memory Persistence Upgrade

## Problem Statement
Codex currently records raw rollouts and a global history.jsonl, yet it lacks a persistent roadmap tying turns to git milestones. Users repeatedly explain context, lose partially completed work, and cannot see drift between cache state and the working tree.

## Goals
1. Persist a canonical project/task graph that survives restarts and branch changes.
2. Surface actionable “next steps” in both CLI and TUI without replaying transcripts.
3. Detect git drift early and provide safe reconciliation workflows.
4. Supply documentation/templates so new repos can adopt the conventions quickly.

## Non-Goals
- Implement multi-agent execution (kept as optional future work).
- Build remote/cloud storage (local file-based cache only for this phase).
- Modify Codex execution sandbox or security model.

## Target Users
- Engineers working on multi-day, multi-branch repositories using Codex CLI.
- Researchers evaluating Codex persistence vs. other AI CLIs (Claude Code, Gemini).
- Maintainers who need auditability of Codex sessions for compliance or collaboration.

## Functional Requirements
- Auto-load project_state.json and per-session cache files when flag enabled.
- update_plan integration: plan updates mirror into task graph nodes/ADR references.
- CLI commands: codex cache show, codex tasks show|complete|edit, codex cache reconcile.
- Drift detection + reconciliation flows (fork/rebase/ignore with warnings).
- Journal/vector memory hooks (optional modules) reference cache metadata.

## Non-Functional Requirements
- File-based storage, git-friendly (JSON/TOML) for diff/PR review.
- Writes must not block turn loop >20 ms on benchmark hardware.
- Configurable feature flags (memory.cache.enabled, write_mode).
- Comprehensive telemetry (cache hit/miss, validation failures, drift events).
- Migration tooling with backups before overwriting cache files.

## Success Metrics
- Resume flow: >90% of beta users report they can continue work without rereading prior chats.
- Drift alerts appear within one turn of branch change; reconciliation actions recorded 100% of the time.
- CLI/TUI commands respond within 100 ms on repos =5k files.
- Migration script successfully seeds cache from rollouts in test scenarios (tracked via sandbox cases).

## Risks & Mitigations
- **Cache corruption** ? versioned schema + backups + validation sandbox.
- **User confusion** ? UX research (Phase 4) + documentation refresh (overview/TRD).
- **Performance impact** ? asynchronous writes, retention policies, telemetry monitoring.

## Dependencies
- Codex git metadata (collect_git_info).
- Feature flag infrastructure/config loader.
- ADR/task graph discipline (repo-specific documentation + ADR updates).
